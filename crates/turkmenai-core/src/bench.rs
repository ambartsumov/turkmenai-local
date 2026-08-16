//! On-device benchmarks, shown in the console. Two honest measurements:
//!
//! 1. **Download** — the real throughput on the user's link, plus a *modelled*
//!    estimate of how a naive client (120 s timeout, no resume, restart-from-zero
//!    on every drop) would have fared. The resume advantage is only ever claimed
//!    when interruptions actually happened during the transfer — no invented
//!    "N× faster" numbers.
//! 2. **Inference** — tokens/sec and time-to-first-token measured against the
//!    running llama-server, with the RAM the run consumed.

use crate::llama::LlamaServerEndpoint;
use crate::{CoreError, HardwareProfile};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadBenchmark {
    pub total_bytes: u64,
    pub elapsed_ms: u64,
    pub avg_bps: u64,
    /// Interruptions our resumable downloader transparently survived.
    pub interruptions: u32,
    /// Modelled time for a no-resume client on the same link; `None` when the
    /// inputs are too small to model. Equal to our transfer time when there were
    /// no interruptions (resume gave no measurable advantage that run).
    pub naive_estimate_ms: Option<u64>,
    /// Whether a naive client would realistically finish at all.
    pub naive_would_complete: bool,
    pub explanation: String,
}

/// Build a download benchmark from a finished transfer's measured facts.
pub fn download_benchmark(
    total_bytes: u64,
    elapsed_ms: u64,
    avg_bps: u64,
    interruptions: u32,
) -> DownloadBenchmark {
    let (naive_estimate_ms, naive_would_complete, explanation) =
        model_naive(total_bytes, avg_bps, elapsed_ms, interruptions);
    DownloadBenchmark {
        total_bytes,
        elapsed_ms,
        avg_bps,
        interruptions,
        naive_estimate_ms,
        naive_would_complete,
        explanation,
    }
}

/// The honest model. A naive client must transfer the whole file inside one
/// uninterrupted window. If our resilient run survived `k` interruptions over a
/// pure-transfer time `T`, we model drops as a Poisson process with rate
/// `λ = k / T`; the chance a single naive attempt of duration `T` completes is
/// `p = e^(−k)`, so expected attempts ≈ `e^k` and a worst-case naive time is
/// `T · e^k` (a full re-download per failed attempt). With `k = 0` there is no
/// advantage to claim and the estimate equals our own transfer time.
fn model_naive(
    total_bytes: u64,
    avg_bps: u64,
    elapsed_ms: u64,
    interruptions: u32,
) -> (Option<u64>, bool, String) {
    if avg_bps == 0 || total_bytes == 0 {
        return (
            None,
            true,
            "Transfer too small or too fast to model a baseline.".into(),
        );
    }
    let transfer_ms = (total_bytes as u128 * 1000 / avg_bps as u128) as u64;
    if interruptions == 0 {
        return (
            Some(elapsed_ms.max(transfer_ms)),
            true,
            "The link stayed up for the whole transfer, so resume gave no measurable advantage this run. On an unstable link a no-resume client would restart from zero on each drop.".into(),
        );
    }
    let k = interruptions as f64;
    let p_success = (-k).exp();
    let naive_ms = (transfer_ms as f64 * k.exp()) as u64;
    // If a naive attempt almost never survives, be honest that it may never finish.
    let would_complete = p_success > 0.02; // ~1 in 50 attempts still succeeds
    let explanation = format!(
        "Our downloader survived {interruptions} interruption(s) and finished. A no-resume client restarting from zero would need ~{:.0}× the transfer time on this link{}.",
        k.exp(),
        if would_complete { "" } else { " and would very likely never complete" }
    );
    (Some(naive_ms), would_complete, explanation)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InferenceBenchmark {
    pub model: String,
    pub prompt_tokens: Option<u64>,
    pub generated_tokens: u64,
    pub total_ms: u64,
    pub time_to_first_token_ms: Option<u64>,
    pub tokens_per_sec: f64,
    /// RAM the run consumed (available-memory delta), best-effort.
    pub ram_used_mib: Option<u64>,
    pub ram_total_mib: u64,
    pub cpu: String,
}

/// Benchmark a short generation against the running loopback llama-server.
/// Requires a ready server; never reaches the network beyond 127.0.0.1.
pub fn inference_benchmark(
    endpoint: &LlamaServerEndpoint,
    model: &str,
    prompt: &str,
    max_tokens: u32,
) -> Result<InferenceBenchmark, CoreError> {
    let hw = HardwareProfile::detect();
    let ram_before = available_ram_mib();

    // Time-to-first-token: a 1-token probe captures prompt-eval + one decode.
    let ttft = {
        let probe = json!({ "model": model, "messages": [{ "role": "user", "content": prompt }], "max_tokens": 1, "temperature": 0 });
        let start = Instant::now();
        endpoint
            .chat(&probe)
            .ok()
            .map(|_| start.elapsed().as_millis() as u64)
    };

    let payload = json!({
        "model": model,
        "messages": [{ "role": "user", "content": prompt }],
        "max_tokens": max_tokens,
        "temperature": 0
    });
    let start = Instant::now();
    let response = endpoint.chat(&payload)?;
    let total_ms = start.elapsed().as_millis() as u64;

    let usage = response.get("usage");
    let generated_tokens = usage
        .and_then(|u| u.get("completion_tokens"))
        .and_then(|v| v.as_u64())
        .unwrap_or_else(|| estimate_tokens(&response));
    let prompt_tokens = usage
        .and_then(|u| u.get("prompt_tokens"))
        .and_then(|v| v.as_u64());
    let tokens_per_sec = if total_ms > 0 {
        generated_tokens as f64 * 1000.0 / total_ms as f64
    } else {
        0.0
    };

    let ram_after = available_ram_mib();
    let ram_used_mib = match (ram_before, ram_after) {
        (Some(before), Some(after)) if before > after => Some(before - after),
        _ => None,
    };

    Ok(InferenceBenchmark {
        model: model.to_string(),
        prompt_tokens,
        generated_tokens,
        total_ms,
        time_to_first_token_ms: ttft,
        tokens_per_sec,
        ram_used_mib,
        ram_total_mib: hw.ram_mib,
        cpu: hw.cpu,
    })
}

/// Rough token estimate from the returned text when the server omits usage
/// (~4 chars per token) — only a fallback so tokens/sec is never blank.
fn estimate_tokens(response: &serde_json::Value) -> u64 {
    let text = response
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    (text.chars().count() as u64 / 4).max(1)
}

/// Available RAM in MiB (Linux /proc/meminfo MemAvailable). `None` elsewhere.
fn available_ram_mib() -> Option<u64> {
    let contents = std::fs::read_to_string("/proc/meminfo").ok()?;
    contents
        .lines()
        .find(|line| line.starts_with("MemAvailable:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|kb| kb.parse::<u64>().ok())
        .map(|kb| kb / 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_interruptions_claims_no_advantage() {
        // 100 MB at 10 MB/s = 10s; no drops → estimate equals our transfer time.
        let b = download_benchmark(100_000_000, 10_000, 10_000_000, 0);
        assert_eq!(b.interruptions, 0);
        assert!(b.naive_would_complete);
        assert_eq!(b.naive_estimate_ms, Some(10_000));
        assert!(b.explanation.contains("no measurable advantage"));
    }

    #[test]
    fn interruptions_make_naive_slower_and_maybe_hopeless() {
        // Same transfer but 4 interruptions survived → e^4 ≈ 54× worst case.
        let b = download_benchmark(100_000_000, 40_000, 10_000_000, 4);
        let naive = b.naive_estimate_ms.unwrap();
        assert!(
            naive > 10_000 * 50,
            "naive should be ~54x the 10s transfer: {naive}"
        );
        // 8 interruptions → p_success = e^-8 ≈ 0.0003 < 2% → unlikely to ever finish.
        let hopeless = download_benchmark(100_000_000, 80_000, 10_000_000, 8);
        assert!(!hopeless.naive_would_complete);
    }

    #[test]
    fn tiny_transfer_is_not_modelled() {
        let b = download_benchmark(0, 0, 0, 0);
        assert_eq!(b.naive_estimate_ms, None);
    }
}

/** Design note — «Горный сигнал»: truthful local-only operational console; native runtime states are explicit and never emulated in the browser. */
import { ArrowLeft, Bug, CheckCircle2, ChevronLeft, ChevronRight, CircleDotDashed, ClipboardCopy, Copy, Cpu, Database, Download, ExternalLink, Languages, LayoutGrid, LockKeyhole, RadioTower, RefreshCw, ServerCog, Square, TerminalSquare } from "lucide-react";
import { product } from "@/generated/product";
import { Link } from "wouter";
import { useEffect, useState, type ReactNode } from "react";
import { copy, type Language } from "@/i18n";
import { discoverRuntime, getCatalogAll, getCatalogRecommendations, getDatasetRecommendations, getDesktopStatus, getHardware, installEngine, installModel, startRuntime, stopRuntime, getTransferStatus, provisionTransfer, benchmarkInference, type CatalogSource, type DatasetEvaluation, type DatasetsResult, type DownloadProgress, type FitLevel, type Hardware, type InstalledModel, type InferenceBenchmark, type Recommendation, type RecommendationsResult, type RuntimeConfig, type RuntimeStatus, type TransferStatus } from "@/lib/desktop";

type Priority = "balanced" | "speed" | "quality" | "memory" | "download";
type Section = "overview" | "models" | "datasets" | "runtime" | "api";
const WIZARD_KEY = "turkmenai.first-run.v1";
const DEFAULT_RUNTIME_CONFIG: RuntimeConfig = { executable_path: null, model_path: null, port: 8080, context_size: 4096, gpu_layers: 0 };
const PRIORITY_OBJECTIVE: Record<Priority, string> = { balanced: "balanced", speed: "fastest", quality: "best_quality", memory: "lowest_ram", download: "lowest_download" };

function detectInitialLanguage(): Language { const value = typeof window === "undefined" ? null : window.localStorage.getItem("turkmenai.language"); return value === "ru" || value === "tk" || value === "en" ? value : "en"; }

function fitLabel(t: Record<string, string>, fit: FitLevel | DatasetEvaluation["fit"]): string {
  const map: Record<string, string> = { excellent: t.fitExcellent, good: t.fitGood, usable: t.fitUsable, slow: t.fitSlow, unsupported: t.fitUnsupported, fits: t.fitFits, tight: t.fitTight };
  return map[fit] ?? fit;
}
function sourceLabel(t: Record<string, string>, source: CatalogSource): string { return source === "remote" ? t.sourceLive : source === "cache" ? t.sourceCache : t.sourceBuiltin; }
function gib(mib: number): string { return mib >= 1024 ? `${(mib / 1024).toFixed(1)} GB` : `${mib} MB`; }
// Local 3-language strings for the new install/benchmark UI, so we don't bloat
// the shared i18n table (whose key-parity is CI-checked).
function L(language: Language, en: string, ru: string, tk: string): string { return language === "ru" ? ru : language === "tk" ? tk : en; }
function fmtSpeed(bps: number): string { const mbps = bps / (1024 * 1024); return mbps >= 1 ? `${mbps.toFixed(1)} MB/s` : `${(bps / 1024).toFixed(0)} KB/s`; }
function fmtBytes(b: number): string { return b >= 1024 * 1024 * 1024 ? `${(b / 1024 ** 3).toFixed(2)} GB` : `${(b / 1024 ** 2).toFixed(0)} MB`; }
function fmtMs(ms: number): string { return ms >= 60000 ? `${Math.floor(ms / 60000)}m ${Math.round((ms % 60000) / 1000)}s` : ms >= 1000 ? `${(ms / 1000).toFixed(1)}s` : `${ms} ms`; }

function runtimeMessage(runtime: RuntimeStatus | null, t: Record<string, string>) {
  if (!runtime) return { title: t.runtimeBrowser, detail: t.wizardRuntimeText, state: "browser" };
  if (runtime.engine_state !== "ready" && !runtime.executable_path) return { title: t.engineNotInstalled, detail: t.engineHintAuto, state: "engine" };
  if (runtime.process?.state === "failed" || runtime.health === "failed") return { title: t.runtimeFailed, detail: runtime.process?.error || t.runtimeHint, state: "failed" };
  if (runtime.health === "ready") return { title: t.runtimeReady, detail: t.runtimeHint, state: "ready" };
  if (runtime.health === "loading") return { title: t.runtimeLoading, detail: t.runtimeHint, state: "loading" };
  if (!runtime.config.model_path) return { title: t.engineReady, detail: t.enginePickModel, state: "ready" };
  return { title: t.runtimeUnreachable, detail: t.runtimeHint, state: "unreachable" };
}

function RuntimeBrief({ language, runtime, checking, refresh }: { language: Language; runtime: RuntimeStatus | null; checking: boolean; refresh: () => void }) {
  const t = copy[language]; const message = runtimeMessage(runtime, t);
  return <div className={`wizard-runtime runtime-${message.state}`}><span className="status-orb">{message.state === "ready" ? <CheckCircle2 size={28}/> : <CircleDotDashed size={28}/>}</span><div><strong>{message.title}</strong><p>{message.detail}</p><button type="button" className="runtime-refresh" onClick={refresh} disabled={checking}><RefreshCw size={14} className={checking ? "spin" : ""}/>{checking ? t.runtimeChecking : t.runtimeRefresh}</button></div></div>;
}

function FirstRunWizard({ language, setLanguage, hardware, loading, runtime, runtimeChecking, refreshRuntime, finish }: { language: Language; setLanguage: (value: Language) => void; hardware: Hardware | null; loading: boolean; runtime: RuntimeStatus | null; runtimeChecking: boolean; refreshRuntime: () => void; finish: () => void }) {
  const [step, setStep] = useState(0); const [priority, setPriority] = useState<Priority>("balanced"); const t = copy[language];
  const steps = [t.wizardLanguage, t.wizardHardware, t.wizardUseCase, t.wizardRuntime];
  const priorities: { id: Priority; label: string }[] = [{ id: "balanced", label: t.wizardBalanced }, { id: "speed", label: t.wizardSpeed }, { id: "quality", label: t.wizardQuality }, { id: "memory", label: t.wizardMemory }, { id: "download", label: t.wizardDownload }];
  const choosePriority = (id: Priority) => { setPriority(id); window.localStorage.setItem("turkmenai.priority", id); };
  const hardwareText = hardware ? `${hardware.cpu} · ${Math.round(hardware.ram_mib / 1024)} GB RAM · ${hardware.os}` : loading ? "…" : t.notConnected;
  const proceed = () => step === steps.length - 1 ? finish() : setStep((value) => value + 1);
  return <main className="onboarding-shell"><section className="onboarding-panel"><div className="onboarding-progress"><span>01 / {String(step + 1).padStart(2, "0")}</span><div>{steps.map((title, index) => <i className={index <= step ? "done" : ""} key={title} />)}</div></div><p className="eyebrow"><span className="signal-dot" />{t.wizardEyebrow}</p><h1>{t.wizardTitle}</h1><p className="onboarding-lead">{t.wizardLead}</p><div className="wizard-stage">{step === 0 && <><h2><Languages size={23}/>{t.wizardLanguage}</h2><div className="wizard-choice-row">{(["en", "ru", "tk"] as Language[]).map((value) => <button className={language === value ? "selected" : ""} type="button" onClick={() => setLanguage(value)} key={value}>{value.toUpperCase()}</button>)}</div></>}{step === 1 && <><h2><Cpu size={23}/>{t.wizardHardware}</h2><div className="wizard-report"><span>{t.wizardDetected}</span><strong>{hardwareText}</strong><small>{t.wizardLocalOnly}</small></div></>}{step === 2 && <><h2><CircleDotDashed size={23}/>{t.wizardUseCase}</h2><div className="wizard-priorities">{priorities.map(({ id, label }) => <button type="button" key={id} className={priority === id ? "selected" : ""} onClick={() => choosePriority(id)}>{label}{priority === id && <CheckCircle2 size={16}/>}</button>)}</div></>}{step === 3 && <><h2><ServerCog size={23}/>{t.wizardRuntime}</h2><RuntimeBrief language={language} runtime={runtime} checking={runtimeChecking} refresh={refreshRuntime}/></>}</div><div className="wizard-actions"><button type="button" className="wizard-back" onClick={() => setStep((value) => Math.max(value - 1, 0))} disabled={step === 0}><ChevronLeft size={17}/>{t.wizardBack}</button><button type="button" className="wizard-next" onClick={proceed}>{step === steps.length - 1 ? t.wizardFinish : t.wizardContinue}<ChevronRight size={17}/></button></div></section></main>;
}

function ModelCard({ rec, t, language, inApp }: { rec: Recommendation; t: Record<string, string>; language: Language; inApp: boolean }) {
  const [copied, setCopied] = useState(false);
  const [installing, setInstalling] = useState(false);
  const [progress, setProgress] = useState<DownloadProgress | null>(null);
  const [installed, setInstalled] = useState<InstalledModel | null>(null);
  const [error, setError] = useState<string | null>(null);
  const m = rec.model; const desc = m.description[language] || m.description.en || "";
  const copyUrl = async () => { await navigator.clipboard?.writeText(rec.download_url); setCopied(true); window.setTimeout(() => setCopied(false), 1800); };
  const install = async () => {
    setInstalling(true); setError(null); setProgress(null); setInstalled(null);
    try {
      const result = await installModel(
        { model_id: m.id, repo: m.repo, revision: m.revision, file: m.file, download_url: rec.download_url, sha256: m.sha256 },
        (p) => setProgress(p),
      );
      if (result) { setInstalled(result); window.localStorage.setItem("turkmenai.model_path", result.path); }
    } catch (e) { setError(String(e)); } finally { setInstalling(false); }
  };
  const pct = progress && progress.total_bytes ? Math.min(100, Math.round((progress.bytes_downloaded / progress.total_bytes) * 100)) : null;
  const bench = installed?.benchmark;
  return <article className={`catalog-card fit-${rec.fit}`}>
    <div className="catalog-card-top"><span className="catalog-cat">{m.category}</span><span className={`fit-badge fit-${rec.fit}`}>{fitLabel(t, rec.fit)}</span></div>
    <h3>{m.name}</h3>
    <p className="catalog-repo">{m.repo}</p>
    {desc && <p className="catalog-desc">{desc}</p>}
    <div className="catalog-meta"><span>{t.sizeLabel}: <b>{gib(m.download_mib)}</b></span><span>{t.ramLabel}: <b>{gib(rec.estimated_ram_mib)}</b></span><span>{t.licenseLabel}: <b>{m.license}</b></span>{m.params_b > 0 && <span><b>{m.params_b}B</b> · {m.quant}</span>}</div>
    {rec.reasons[0] && <p className="catalog-why"><b>{t.whyFits}:</b> {rec.reasons[0]}</p>}
    {installing && <div className="install-progress"><div className="install-bar"><i style={{ width: `${pct ?? 8}%` }} className={pct === null ? "indet" : ""} /></div><div className="install-stats"><span>{pct !== null ? `${pct}%` : L(language, "downloading…", "загрузка…", "ýüklenýär…")}</span>{progress && <span>{fmtSpeed(progress.speed_bps)}</span>}{progress && progress.total_bytes && <span>{fmtBytes(progress.bytes_downloaded)} / {fmtBytes(progress.total_bytes)}</span>}{progress && progress.retries > 0 && <span className="install-reconnect">↻ {L(language, "reconnecting", "переподключение", "täzeden birikmek")} ×{progress.retries}</span>}</div></div>}
    {bench && <div className="install-done"><p><CheckCircle2 size={14}/> {L(language, "Installed", "Установлено", "Gurnaldy")} · {installed?.backend === "hf_xet" ? "Xet" : L(language, "built-in", "встроенный", "içerki")}</p><div className="bench-row"><span>{L(language, "Speed", "Скорость", "Tizlik")}: <b>{fmtSpeed(bench.avg_bps)}</b></span><span>{L(language, "Time", "Время", "Wagt")}: <b>{fmtMs(bench.elapsed_ms)}</b></span>{bench.interruptions > 0 && <span>↻ {bench.interruptions}</span>}</div>{bench.naive_estimate_ms !== null && bench.interruptions > 0 && <p className="bench-naive">{L(language, "Without our resume tech", "Без нашей докачки", "Bizsiz dowam etdirmesiz")}: <b>~{fmtMs(bench.naive_estimate_ms)}</b>{!bench.naive_would_complete && <> — {L(language, "likely never finishes", "скорее всего не завершится", "gutarmaz")}</>}</p>}<p className="bench-explain">{bench.explanation}</p><p className="bench-path">{L(language, "Now open Runtime — the model path is pre-filled.", "Теперь откройте Runtime — путь к модели уже подставлен.", "Indi Runtime açyň — model ýoly öňünden goýlan.")}</p></div>}
    {error && <output className="runtime-error">{error}</output>}
    <div className="catalog-actions">{inApp && !installed && <button type="button" className="wizard-next" onClick={install} disabled={installing}><Download size={15}/>{installing ? L(language, "Installing…", "Установка…", "Gurulýar…") : L(language, "Install", "Установить", "Gurmak")}</button>}<a className="download-action" href={rec.download_url} target="_blank" rel="noreferrer"><Download size={15}/>Hugging Face</a><button type="button" className="runtime-refresh" onClick={copyUrl}><Copy size={13}/>{copied ? t.copied : t.copyLink}</button></div>
  </article>;
}

function ModelsPanel({ language, priority }: { language: Language; priority: Priority }) {
  const t = copy[language];
  const [result, setResult] = useState<RecommendationsResult | null>(null);
  const [all, setAll] = useState<Recommendation[] | null>(null);
  const [loading, setLoading] = useState(true); const [refreshing, setRefreshing] = useState(false); const [showIncompatible, setShowIncompatible] = useState(false); const [inApp, setInApp] = useState(true);
  const load = async (refresh: boolean) => {
    const objective = PRIORITY_OBJECTIVE[priority];
    const next = await getCatalogRecommendations(objective, refresh);
    if (next === null) { setInApp(false); setLoading(false); return; }
    setResult(next); setInApp(true);
    if (showIncompatible) setAll(await getCatalogAll(false));
  };
  useEffect(() => { setLoading(true); load(false).finally(() => setLoading(false)); /* eslint-disable-next-line */ }, [priority]);
  const refresh = async () => { setRefreshing(true); try { await load(true); } finally { setRefreshing(false); } };
  const toggleIncompatible = async () => { const next = !showIncompatible; setShowIncompatible(next); if (next && !all) setAll(await getCatalogAll(false)); };
  if (!inApp) return <div className="catalog-empty"><LayoutGrid size={26}/><p>{t.runtimeBrowser}</p><span>{t.modelsLead}</span></div>;
  const incompatible = (all || []).filter((rec) => rec.fit === "unsupported");
  return <section className="catalog-panel"><div className="catalog-head"><div><p className="eyebrow"><span className="signal-dot"/>{t.models.toUpperCase()}</p><h1>{t.modelsTitle}</h1><p>{t.modelsLead}</p></div><button type="button" className="wizard-next" onClick={refresh} disabled={refreshing}><RefreshCw size={15} className={refreshing ? "spin" : ""}/>{refreshing ? t.catalogRefreshing : t.catalogRefresh}</button></div>
    {result && <div className="catalog-source"><span className={`src-badge src-${result.source}`}>{sourceLabel(t, result.source)}</span>{result.source === "builtin" && <small>{t.offlineHint}</small>}</div>}
    {loading ? <div className="catalog-empty"><RefreshCw size={22} className="spin"/></div> : (result && result.recommendations.length > 0 ? <div className="catalog-grid">{result.recommendations.map((rec) => <ModelCard key={rec.model.id} rec={rec} t={t} language={language} inApp={inApp}/>)}</div> : <div className="catalog-empty"><p>{t.noModels}</p></div>)}
    {result && result.recommendations.length > 0 && <button type="button" className="catalog-toggle" onClick={toggleIncompatible}>{showIncompatible ? t.hideIncompatible : t.showIncompatible}</button>}
    {showIncompatible && incompatible.length > 0 && <div className="catalog-grid dimmed">{incompatible.map((rec) => <ModelCard key={rec.model.id} rec={rec} t={t} language={language} inApp={inApp}/>)}</div>}
  </section>;
}

function DatasetsPanel({ language }: { language: Language }) {
  const t = copy[language];
  const [result, setResult] = useState<DatasetsResult | null>(null);
  const [loading, setLoading] = useState(true); const [refreshing, setRefreshing] = useState(false); const [inApp, setInApp] = useState(true);
  const load = async (refresh: boolean) => { const next = await getDatasetRecommendations(refresh); if (next === null) { setInApp(false); return; } setResult(next); setInApp(true); };
  useEffect(() => { setLoading(true); load(false).finally(() => setLoading(false)); }, []);
  const refresh = async () => { setRefreshing(true); try { await load(true); } finally { setRefreshing(false); } };
  if (!inApp) return <div className="catalog-empty"><Database size={26}/><p>{t.runtimeBrowser}</p><span>{t.datasetsLead}</span></div>;
  return <section className="catalog-panel"><div className="catalog-head"><div><p className="eyebrow"><span className="signal-dot"/>{t.datasets.toUpperCase()}</p><h1>{t.datasetsTitle}</h1><p>{t.datasetsLead}</p></div><button type="button" className="wizard-next" onClick={refresh} disabled={refreshing}><RefreshCw size={15} className={refreshing ? "spin" : ""}/>{refreshing ? t.catalogRefreshing : t.catalogRefresh}</button></div>
    {result && <div className="catalog-source"><span className={`src-badge src-${result.source}`}>{sourceLabel(t, result.source)}</span>{result.source === "builtin" && <small>{t.offlineHint}</small>}</div>}
    {loading ? <div className="catalog-empty"><RefreshCw size={22} className="spin"/></div> : (result && result.datasets.length > 0 ? <div className="catalog-grid">{result.datasets.map((item) => { const d = item.dataset; const desc = d.description[language] || d.description.en || ""; return <article key={d.id} className={`catalog-card fit-${item.fit}`}><div className="catalog-card-top"><span className="catalog-cat">{d.category}</span><span className={`fit-badge fit-${item.fit}`}>{fitLabel(t, item.fit)}</span></div><h3>{d.name}</h3><p className="catalog-repo">{d.repo}</p>{desc && <p className="catalog-desc">{desc}</p>}<div className="catalog-meta"><span>{t.diskLabel}: <b>{gib(item.required_disk_mib)}</b></span><span>{t.licenseLabel}: <b>{d.license}</b></span>{d.languages.length > 0 && <span>{t.languagesLabel}: <b>{d.languages.slice(0, 4).join(", ")}</b></span>}</div>{item.reasons[0] && <p className="catalog-why"><b>{t.whyFits}:</b> {item.reasons[0]}</p>}<div className="catalog-actions"><a className="download-action" href={item.page_url} target="_blank" rel="noreferrer"><Download size={15}/>Hugging Face</a></div></article>; })}</div> : <div className="catalog-empty"><p>{t.noModels}</p></div>)}
  </section>;
}

function ApiPanel({ language }: { language: Language }) {
  const t = copy[language];
  const core = [
    { m: "GET", p: "/api/v1/health" },
    { m: "GET", p: "/api/v1/hardware" },
    { m: "GET", p: "/api/v1/capabilities" },
    { m: "GET", p: "/api/v1/catalog" },
    { m: "GET", p: "/api/v1/catalog/recommendations" },
    { m: "GET", p: "/api/v1/datasets" },
    { m: "GET", p: "/api/v1/datasets/recommendations" },
    { m: "POST", p: "/api/v1/analyze" },
    { m: "POST", p: "/api/v1/plan" },
    { m: "GET/POST", p: "/api/v1/runtime" },
    { m: "POST", p: "/api/v1/runtime/start" },
    { m: "POST", p: "/api/v1/runtime/stop" },
  ];
  const openai = [
    { m: "GET", p: "/v1/models" },
    { m: "POST", p: "/v1/chat/completions" },
  ];
  return <section className="catalog-panel"><div className="catalog-head"><div><p className="eyebrow"><span className="signal-dot"/>API</p><h1>{t.apiTitle}</h1><p>{t.apiLead}</p></div><code className="api-base">http://127.0.0.1:8742</code></div>
    <div className="api-groups"><div><h3>{t.apiCoreGroup}</h3><ul className="api-list">{core.map((e) => <li key={e.p}><span className={`api-method m-${e.m.split("/")[0].toLowerCase()}`}>{e.m}</span><code>{e.p}</code></li>)}</ul></div><div><h3>{t.apiOpenAIGroup}</h3><ul className="api-list">{openai.map((e) => <li key={e.p}><span className={`api-method m-${e.m.toLowerCase()}`}>{e.m}</span><code>{e.p}</code></li>)}</ul><p className="api-note">{t.apiLoopbackNote}</p></div></div>
  </section>;
}

// REPORT BUG — collects real on-device diagnostics (version, OS, CPU/RAM/GPU,
// runtime + transport state) and opens a prefilled GitHub issue. Nothing is sent
// anywhere until the user clicks; the same block can be copied to the clipboard.
function ReportBug({ language, runtime }: { language: Language; runtime: RuntimeStatus | null }) {
  const [status, setStatus] = useState<Awaited<ReturnType<typeof getDesktopStatus>>>(null);
  const [hw, setHw] = useState<Hardware | null>(null);
  const [xet, setXet] = useState<TransferStatus | null>(null);
  const [copied, setCopied] = useState(false);
  useEffect(() => { getDesktopStatus().then(setStatus); getHardware().then(setHw); getTransferStatus().then((s) => s && setXet(s)); }, []);
  const inApp = Boolean(status);
  const lines = [
    `Product: ${product.product} v${status?.core_version ?? product.version} (${product.channel})`,
    `Platform: ${status?.platform ?? (typeof navigator !== "undefined" ? navigator.platform : "unknown")}`,
    `OS: ${hw?.os ?? "—"}`,
    `CPU: ${hw?.cpu ?? "—"}`,
    `RAM: ${hw ? (hw.ram_mib / 1024).toFixed(1) + " GB" : "—"}  ·  Free disk: ${hw ? (hw.free_disk_mib / 1024).toFixed(1) + " GB" : "—"}`,
    `GPU: ${hw?.accelerators?.length ? hw.accelerators.join(", ") : "none"}${hw?.vram_mib ? ` (${(hw.vram_mib / 1024).toFixed(1)} GB VRAM)` : ""}`,
    `Runtime: engine ${runtime?.engine_state ?? "?"}${runtime?.engine ? ` (${runtime.engine.backend} ${runtime.engine.version})` : ""} · health ${runtime?.health ?? "—"}`,
    `Transport: builtin ${xet?.builtin_ready ? "ready" : "—"} · xet ${xet?.xet.state ?? "—"}${xet?.xet.hf_version ? ` (${xet.xet.hf_version})` : ""}`,
    `UA: ${typeof navigator !== "undefined" ? navigator.userAgent : ""}`,
  ];
  const diagnostics = lines.join("\n");
  const bugTitle = `[bug] v${status?.core_version ?? product.version} — `;
  const bugBody = `**What happened**\n\n\n**Steps to reproduce**\n1. \n2. \n\n**Expected**\n\n\n---\n**Diagnostics** (auto-filled)\n\`\`\`\n${diagnostics}\n\`\`\`\n`;
  const issueUrl = `${product.repository}/issues/new?labels=bug&title=${encodeURIComponent(bugTitle)}&body=${encodeURIComponent(bugBody)}`;
  const copyDiag = async () => { try { await navigator.clipboard?.writeText(diagnostics); setCopied(true); window.setTimeout(() => setCopied(false), 2000); } catch { /* ignore */ } };
  return <section className="lab-panel">
    <div className="lab-head"><span>{L(language, "DIAGNOSTICS & BUG REPORT", "ДИАГНОСТИКА И ОТЧЁТ ОБ ОШИБКЕ", "DIAGNOSTIKA WE ÝALŇYŞLYK HABARY")}</span></div>
    {!inApp && <p className="lab-hint">{L(language, "Open in the desktop app for full hardware diagnostics.", "Откройте в десктоп-приложении для полной диагностики железа.", "Enjam diagnostikasy üçin desktop programmasynda açyň.")}</p>}
    <pre className="lab-detail" style={{ whiteSpace: "pre-wrap", fontFamily: "monospace", fontSize: ".74rem", lineHeight: 1.5 }}>{diagnostics}</pre>
    <div className="runtime-actions">
      <a className="wizard-next" href={issueUrl} target="_blank" rel="noreferrer"><Bug size={15}/>{L(language, "Report a bug", "Сообщить об ошибке", "Ýalňyşlyk barada habar bermek")}<ExternalLink size={13}/></a>
      <button type="button" className="runtime-refresh" onClick={copyDiag}><ClipboardCopy size={13}/>{copied ? L(language, "Copied", "Скопировано", "Göçürildi") : L(language, "Copy diagnostics", "Копировать диагностику", "Diagnostikany göçürmek")}</button>
    </div>
  </section>;
}

function LabPanel({ language, runtime }: { language: Language; runtime: RuntimeStatus | null }) {
  const [transfer, setTransfer] = useState<TransferStatus | null>(null);
  const [provisioning, setProvisioning] = useState(false);
  const [bench, setBench] = useState<InferenceBenchmark | null>(null);
  const [benching, setBenching] = useState(false);
  const [benchErr, setBenchErr] = useState<string | null>(null);
  useEffect(() => { getTransferStatus().then((status) => status && setTransfer(status)); }, []);
  const provision = async () => { setProvisioning(true); try { const status = await provisionTransfer(); if (status) setTransfer(status); } finally { setProvisioning(false); } };
  const ready = runtime?.health === "ready";
  const runBench = async () => {
    if (!runtime) return;
    setBenching(true); setBenchErr(null);
    try {
      const model = runtime.config.model_path?.split(/[/\\]/).pop() || "model";
      const result = await benchmarkInference(runtime.config.port, model);
      if (result) setBench(result);
    } catch (e) { setBenchErr(String(e)); } finally { setBenching(false); }
  };
  const xet = transfer?.xet;
  return <section className="lab-panel">
    <div className="lab-head"><span>{L(language, "TRANSFER & BENCHMARKS", "ТРАНСПОРТ И БЕНЧМАРКИ", "TRANSPORT WE BENÇMARKLAR")}</span></div>
    <div className="lab-transfer">
      <div className="lab-line"><CheckCircle2 size={15}/><b>{L(language, "Built-in downloader", "Встроенный загрузчик", "Içerki ýükleýji")}</b><span className="lab-ok">{L(language, "ready", "готов", "taýýar")}</span></div>
      {transfer && <div className={`lab-line ${xet?.state === "ready" ? "" : "lab-missing"}`}>{xet?.state === "ready" ? <CheckCircle2 size={15}/> : <CircleDotDashed size={15}/>}<b>Xet {xet?.hf_version ? `(${xet.hf_version})` : ""}</b><span className={xet?.state === "ready" ? "lab-ok" : "lab-warn"}>{xet?.state === "ready" ? L(language, "ready", "готов", "taýýar") : L(language, "not installed", "не установлен", "gurulmadyk")}</span></div>}
      {transfer && <p className="lab-detail">{xet?.detail[language]}</p>}
      {xet?.state === "not_installed" && <div className="lab-instructions"><p>{L(language, "Set up accelerated downloads:", "Настроить ускоренную загрузку:", "Çaltlaşdyrylan ýüklemäni gurmak:")}</p><ol>{xet.instructions.map((step, i) => <li key={i}>{step.text[language]}{step.command && <code>{step.command}</code>}</li>)}</ol><button type="button" className="runtime-refresh" onClick={provision} disabled={provisioning}><Download size={13}/>{provisioning ? L(language, "Setting up…", "Настройка…", "Gurulýar…") : L(language, "Try auto-setup", "Авто-настройка", "Awto-gurnama")}</button></div>}
    </div>
    <div className="lab-bench">
      <button type="button" className="wizard-next" onClick={runBench} disabled={!ready || benching}><Cpu size={15}/>{benching ? L(language, "Benchmarking…", "Замер…", "Ölçenýär…") : L(language, "Run inference benchmark", "Запустить бенчмарк", "Bençmark işletmek")}</button>
      {!ready && <p className="lab-hint">{L(language, "Start the runtime to benchmark this model.", "Запустите runtime, чтобы протестировать модель.", "Modeli synamak üçin runtime işlediň.")}</p>}
      {benchErr && <output className="runtime-error">{benchErr}</output>}
      {bench && <div className="bench-grid"><div><span>{L(language, "Tokens/sec", "Токенов/с", "Token/sek")}</span><b>{bench.tokens_per_sec.toFixed(1)}</b></div><div><span>{L(language, "First token", "Первый токен", "Ilkinji token")}</span><b>{bench.time_to_first_token_ms !== null ? fmtMs(bench.time_to_first_token_ms) : "—"}</b></div><div><span>{L(language, "Generated", "Сгенерировано", "Döredildi")}</span><b>{bench.generated_tokens}</b></div><div><span>{L(language, "Total time", "Общее время", "Umumy wagt")}</span><b>{fmtMs(bench.total_ms)}</b></div><div><span>RAM</span><b>{bench.ram_used_mib !== null ? `${bench.ram_used_mib} MB` : "—"} / {(bench.ram_total_mib / 1024).toFixed(0)} GB</b></div><div className="bench-cpu"><span>CPU</span><b>{bench.cpu}</b></div></div>}
    </div>
  </section>;
}

export default function Console() {
  const [language, setLanguageState] = useState<Language>(detectInitialLanguage); const [hardware, setHardware] = useState<Hardware | null>(null); const [connected, setConnected] = useState(false); const [loading, setLoading] = useState(true); const [firstRun, setFirstRun] = useState(() => typeof window !== "undefined" && !window.localStorage.getItem(WIZARD_KEY)); const [runtime, setRuntime] = useState<RuntimeStatus | null>(null); const [runtimeConfig, setRuntimeConfig] = useState<RuntimeConfig>(() => { const saved = typeof window === "undefined" ? null : window.localStorage.getItem("turkmenai.model_path"); return saved ? { ...DEFAULT_RUNTIME_CONFIG, model_path: saved } : DEFAULT_RUNTIME_CONFIG; }); const [runtimeChecking, setRuntimeChecking] = useState(false); const [runtimeError, setRuntimeError] = useState<string | null>(null); const [section, setSection] = useState<Section>("overview"); const [priority] = useState<Priority>(() => { const value = typeof window === "undefined" ? null : window.localStorage.getItem("turkmenai.priority"); return value === "speed" || value === "quality" || value === "memory" || value === "download" ? value : "balanced"; }); const t = copy[language];
  const setLanguage = (value: Language) => { setLanguageState(value); window.localStorage.setItem("turkmenai.language", value); };
  const syncRuntime = async () => { setRuntimeChecking(true); setRuntimeError(null); try { const next = await discoverRuntime(); setRuntime(next); if (next) setRuntimeConfig(next.config); } catch (error) { setRuntimeError(error instanceof Error ? error.message : t.runtimeStartError); } finally { setRuntimeChecking(false); } };
  useEffect(() => { Promise.all([getDesktopStatus(), getHardware(), discoverRuntime()]).then(([status, profile, nextRuntime]) => { setConnected(Boolean(status)); setHardware(profile); setRuntime(nextRuntime); if (nextRuntime) setRuntimeConfig(nextRuntime.config); }).finally(() => setLoading(false)); }, []);
  const finish = () => { window.localStorage.setItem(WIZARD_KEY, "complete"); setFirstRun(false); };
  const updateRuntimeConfig = (field: keyof RuntimeConfig, value: string | number | null) => setRuntimeConfig((current) => ({ ...current, [field]: value }));
  const handleStart = async () => { setRuntimeChecking(true); setRuntimeError(null); try { const next = await startRuntime(runtimeConfig); if (!next) throw new Error(t.runtimeBrowser); setRuntime(next); setRuntimeConfig(next.config); } catch (error) { setRuntimeError(error instanceof Error ? error.message : t.runtimeStartError); } finally { setRuntimeChecking(false); } };
  const handleStop = async () => { setRuntimeChecking(true); setRuntimeError(null); try { const next = await stopRuntime(); setRuntime(next); } catch (error) { setRuntimeError(error instanceof Error ? error.message : t.runtimeStartError); } finally { setRuntimeChecking(false); } };
  const [engineInstalling, setEngineInstalling] = useState(false);
  const handleEngineInstall = async () => { setEngineInstalling(true); setRuntimeError(null); try { await installEngine(); await syncRuntime(); } catch (error) { setRuntimeError(error instanceof Error ? error.message : t.runtimeStartError); } finally { setEngineInstalling(false); } };
  if (firstRun) return <FirstRunWizard language={language} setLanguage={setLanguage} hardware={hardware} loading={loading} runtime={runtime} runtimeChecking={runtimeChecking} refreshRuntime={syncRuntime} finish={finish} />;
  const title = connected ? t.nativeReady : t.serviceMissing; const detail = connected ? t.nativeText : t.serviceText; const hardwareText = hardware ? `${hardware.cpu} · ${Math.round(hardware.ram_mib / 1024)} GB RAM · ${hardware.os}` : loading ? "…" : t.serviceText; const runtimeState = runtimeMessage(runtime, t);
  const navItem = (id: Section, icon: ReactNode, label: string) => <a className={section === id ? "nav-active" : ""} onClick={() => setSection(id)} role="button" tabIndex={0}>{icon}{label}</a>;
  return <main className="console-shell"><header className="console-header"><Link href="/" className="back-link"><ArrowLeft size={17}/>{t.back}</Link><div className="console-brand"><img src="/assets/turkmenai-mark.svg" alt="" width={26} height={26}/>TurkmenAI <em>Local</em></div><div className="language-switch">{(["en", "ru", "tk"] as Language[]).map((item) => <button key={item} onClick={() => setLanguage(item)} className={language === item ? "active" : ""}>{item.toUpperCase()}</button>)}</div></header><div className="console-layout"><aside className="console-nav"><span>LOCAL CONTROL</span>{navItem("overview", <TerminalSquare size={17}/>, t.overview)}{navItem("models", <LayoutGrid size={17}/>, t.models)}{navItem("datasets", <Database size={17}/>, t.datasets)}{navItem("runtime", <ServerCog size={17}/>, t.runtimes)}{navItem("api", <RadioTower size={17}/>, t.api)}</aside><section className="console-main">
    {section === "models" && <ModelsPanel language={language} priority={priority}/>}
    {section === "datasets" && <DatasetsPanel language={language}/>}
    {section === "api" && <ApiPanel language={language}/>}
    {(section === "overview" || section === "runtime") && <>
    <div className="console-intro"><p className="eyebrow"><span className="signal-dot"/>127.0.0.1 ONLY</p><h1>{t.consoleTitle}</h1><p>{t.consoleLead}</p></div><div className="console-status"><div className="status-orb"><CircleDotDashed size={32}/></div><div><p>{title}</p><span>{detail}</span></div><code>{connected ? "native://core" : "tmai server"}</code></div><div className="console-cards"><article><div className="console-card-label">{t.hardwareFound}</div><b>{connected ? t.localProfile : t.notConnected}</b><span>{hardwareText}</span></article><article><div className="console-card-label">{t.privacy}</div><div className="privacy-lines"><span><LockKeyhole size={14}/>{t.telemetryOff}</span><span><LockKeyhole size={14}/>{t.cloudOff}</span><span><LockKeyhole size={14}/>{t.lanOff}</span></div></article><article><div className="console-card-label">RUNTIME</div><b>{runtimeState.title}</b><span>{runtimeState.detail}</span></article></div>{runtime && <section className="runtime-control"><div className="runtime-control-head"><div><span>{t.runtimeConfigure}</span><strong>{runtimeState.title}</strong></div><button type="button" className="runtime-refresh" onClick={syncRuntime} disabled={runtimeChecking}><RefreshCw size={14} className={runtimeChecking ? "spin" : ""}/>{runtimeChecking ? t.runtimeChecking : t.runtimeRefresh}</button></div>{runtime.engine_state !== "ready" ? <div className="engine-banner"><div><strong>{t.engineNotInstalled}</strong><p>{t.engineHintAuto}</p></div><button type="button" className="wizard-next" onClick={handleEngineInstall} disabled={engineInstalling}><Download size={15}/>{engineInstalling ? t.engineInstalling : t.engineInstall}</button></div> : <p className="engine-version">{t.engineVersion}: {runtime.engine?.backend} {runtime.engine?.version} · {t.engineReady}</p>}<div className="runtime-inputs"><label>{t.runtimeExecutable}<input value={runtimeConfig.executable_path || ""} onChange={(event) => updateRuntimeConfig("executable_path", event.target.value || null)} placeholder="/usr/local/bin/llama-server"/></label><label>{t.runtimeModel}<input value={runtimeConfig.model_path || ""} onChange={(event) => updateRuntimeConfig("model_path", event.target.value || null)} placeholder="/path/to/model.gguf"/></label><label>{t.runtimePort}<input type="number" min="1" max="65535" value={runtimeConfig.port} onChange={(event) => updateRuntimeConfig("port", Number(event.target.value) || 8080)}/></label></div><p>{t.runtimeHint}</p>{runtimeError && <output className="runtime-error">{runtimeError}</output>}<div className="runtime-actions"><button type="button" className="wizard-next" onClick={handleStart} disabled={runtimeChecking}><ServerCog size={16}/>{t.runtimeStart}</button>{runtime.process && <button type="button" className="runtime-stop" onClick={handleStop} disabled={runtimeChecking}><Square size={14}/>{t.runtimeStop}</button>}</div></section>}<LabPanel language={language} runtime={runtime}/><ReportBug language={language} runtime={runtime}/><div className="console-command"><div><span>{t.nextStep}</span><strong>{t.nextStepText}</strong></div><code>tmai server 8742</code></div>
    </>}
  </section></div></main>;
}

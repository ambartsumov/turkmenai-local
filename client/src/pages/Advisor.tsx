/** Training Advisor: "what can I train on THIS machine?"
 *  A metadata-driven planner over the model catalog. It estimates LoRA / QLoRA
 *  feasibility from RAM/VRAM and points to Axolotl. Estimates are deliberately
 *  rough and labelled as such — no fabricated precision. */
import { useState } from "react";
import { Cpu, MemoryStick, CircleCheck, CircleX, ExternalLink, TriangleAlert } from "lucide-react";
import { Link } from "wouter";
import type { LocalizedText } from "@/generated/product";
import { PageChrome } from "@/components/PageChrome";
import { useApi, usePersistedLanguage } from "@/lib/useApi";

type Model = { id: string; name: string; params_b: number; quant: string; min_ram_mib: number; description: LocalizedText };

// Rough VRAM/RAM budgets (GB) for parameter-efficient fine-tuning. Weights + LoRA
// adapters + optimizer state + activations, order-of-magnitude only.
const qloraGB = (b: number) => Math.max(2, b * 1.5); // 4-bit base weights
const loraGB = (b: number) => Math.max(4, b * 4); // fp16 base weights
const AXOLOTL = "https://github.com/axolotl-ai-cloud/axolotl";

function Verdict({ ok, label }: { ok: boolean; label: string }) {
  return <span className={`feat-chip feat-chip-${ok ? "stable" : "removed"}`} style={{ display: "inline-flex", alignItems: "center", gap: ".35rem" }}>
    {ok ? <CircleCheck size={13} /> : <CircleX size={13} />}{label}</span>;
}

export default function Advisor() {
  const [language, setLanguage] = usePersistedLanguage();
  const guessedRam = typeof navigator !== "undefined" && (navigator as any).deviceMemory ? Number((navigator as any).deviceMemory) : 16;
  const [ram, setRam] = useState<number>(guessedRam);
  const [vram, setVram] = useState<number>(0);
  const { data } = useApi<{ models: Model[] }>("/api/models/index.json");

  const onGpu = vram > 0;
  const budget = onGpu ? vram : ram; // GPU path uses VRAM; CPU path uses system RAM (works, but slow)

  return <PageChrome language={language} setLanguage={setLanguage}>
    <section className="build-section">
      <p className="eyebrow"><span className="signal-dot" />TRAINING / ADVISOR</p>
      <h2>What can you train on this PC?</h2>
      <p className="hero-lead">Enter your memory and see which catalog models are realistic to fine-tune with LoRA / QLoRA. Estimates are rough, order-of-magnitude — verify on your own hardware before committing.</p>

      <div className="score-sample" style={{ marginTop: "1.5rem", flexWrap: "wrap", gap: "1rem" }}>
        <div><b><Cpu size={14} /> SYSTEM RAM (GB)</b><input type="number" min={2} max={512} value={ram} onChange={(e) => setRam(Number(e.target.value) || 0)} style={{ width: "6rem", marginTop: ".35rem", background: "transparent", color: "inherit", border: "1px solid currentColor", borderRadius: 6, padding: ".25rem .5rem" }} /></div>
        <i>·</i>
        <div><b><MemoryStick size={14} /> GPU VRAM (GB, 0 = CPU)</b><input type="number" min={0} max={192} value={vram} onChange={(e) => setVram(Number(e.target.value) || 0)} style={{ width: "6rem", marginTop: ".35rem", background: "transparent", color: "inherit", border: "1px solid currentColor", borderRadius: 6, padding: ".25rem .5rem" }} /></div>
      </div>

      {!onGpu && <p className="download-policy" style={{ marginTop: "1rem", display: "flex", gap: ".5rem", alignItems: "center" }}><TriangleAlert size={15} /> CPU-only fine-tuning works but is slow. For anything above ~1.5B, a GPU (or a free Kaggle/Colab T4) is strongly recommended.</p>}

      {data && <div className="build-grid" style={{ marginTop: "1.75rem" }}>{[...data.models].sort((a, b) => a.params_b - b.params_b).map((m) => {
        const q = qloraGB(m.params_b), l = loraGB(m.params_b);
        const canQ = budget >= q, canL = budget >= l;
        return <article key={m.id} className={`feature-card feat-${canQ ? "stable" : "planned"}`}>
          <div className="feature-card-top"><Cpu size={22} /><span className="feat-chip feat-chip-beta">{m.params_b}B</span></div>
          <Link href={`/models/${m.id}`} style={{ textDecoration: "none" }}><h3>{m.name}</h3></Link>
          <div style={{ display: "flex", gap: ".5rem", flexWrap: "wrap", margin: ".5rem 0" }}>
            <Verdict ok={canQ} label={`QLoRA · ~${q.toFixed(1)} GB`} />
            <Verdict ok={canL} label={`LoRA · ~${l.toFixed(1)} GB`} />
          </div>
          <p className="download-policy">{canQ ? (onGpu ? "Fits your GPU for QLoRA." : "Fits your RAM (CPU-offload QLoRA, slow but possible).") : "Needs more memory — pick a smaller model or add a GPU."}</p>
        </article>;
      })}</div>}

      <div className="hero-actions" style={{ marginTop: "1.75rem" }}>
        <a className="primary-action" href={AXOLOTL} target="_blank" rel="noreferrer"><ExternalLink size={16} /> Axolotl (LoRA/QLoRA recipes)</a>
        <Link href="/models" className="quiet-action">Browse models</Link>
      </div>
    </section>
  </PageChrome>;
}

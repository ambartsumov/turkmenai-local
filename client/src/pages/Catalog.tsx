/** Metadata-driven catalog pages: /models, /models/:slug, /datasets, /datasets/:slug.
 *  Everything renders from the generated /api/*.json — the single source of truth. */
import { Box, Database, Download, ExternalLink, HardDrive, Cpu, ScrollText, Tag } from "lucide-react";
import { Link, useParams } from "wouter";
import type { LocalizedText } from "@/generated/product";
import { PageChrome, BackLink } from "@/components/PageChrome";
import { useApi, usePersistedLanguage } from "@/lib/useApi";

const HF = "https://huggingface.co";
const mib = (n: number) => (n >= 1024 ? `${(n / 1024).toFixed(1)} GB` : `${n} MB`);

type Model = {
  id: string; name: string; repo: string; revision: string; file: string; sha256: string | null;
  license: string; task: string; format: string; params_b: number; quant: string;
  download_mib: number; min_ram_mib: number; rec_ram_mib: number; context: number;
  trust: string; tags: string[]; objectives: string[]; description: LocalizedText;
};
type Dataset = {
  id: string; name: string; repo: string; revision: string; category: string; license: string;
  languages: string[]; download_mib: number; unpacked_mib: number; num_examples: number;
  risk: string; description: LocalizedText;
};

function States({ loading, error }: { loading: boolean; error: string | null }) {
  if (loading) return <p className="hero-lead">Loading…</p>;
  return <p className="hero-lead">Not found{error ? ` — ${error}` : ""}.</p>;
}

export function ModelsIndex() {
  const [language, setLanguage] = usePersistedLanguage();
  const { data, loading, error } = useApi<{ models: Model[] }>("/api/models/index.json");
  return <PageChrome language={language} setLanguage={setLanguage}>
    <section className="build-section">
      <div className="build-head"><div><p className="eyebrow"><span className="signal-dot" />CATALOG / MODELS</p><h2>Models</h2></div></div>
      {!data ? <States loading={loading} error={error} /> : <div className="build-grid">{data.models.map((m) => (
        <Link key={m.id} href={`/models/${m.id}`} className="feature-card feat-stable" style={{ textDecoration: "none" }}>
          <div className="feature-card-top"><Box size={22} /><span className="feat-chip feat-chip-stable">{m.params_b}B · {m.quant}</span></div>
          <h3>{m.name}</h3><p>{m.description[language]}</p>
          <p className="download-policy" style={{ marginTop: "auto" }}>{mib(m.download_mib)} · RAM {mib(m.min_ram_mib)}+ · {m.license}</p>
        </Link>))}</div>}
    </section>
  </PageChrome>;
}

export function ModelDetail() {
  const [language, setLanguage] = usePersistedLanguage();
  const { slug } = useParams();
  const { data, loading, error } = useApi<{ models: Model[] }>("/api/models/index.json");
  const m = data?.models.find((x) => x.id === slug);
  return <PageChrome language={language} setLanguage={setLanguage}>
    <section className="build-section">
      <BackLink href="/models" label="All models" />
      {!m ? <States loading={loading} error={error} /> : <>
        <p className="eyebrow"><span className="signal-dot" />MODEL / {m.task.toUpperCase()}</p>
        <h2>{m.name}</h2>
        <p className="hero-lead">{m.description[language]}</p>
        <div className="score-sample" style={{ marginTop: "1.5rem" }}>
          <div><b><Cpu size={14} /> PARAMS</b><span>{m.params_b}B · {m.quant} · {m.format.toUpperCase()}</span></div>
          <i>·</i>
          <div><b><Download size={14} /> DOWNLOAD</b><span>{mib(m.download_mib)}</span></div>
          <i>·</i>
          <div><b><HardDrive size={14} /> RAM</b><span>min {mib(m.min_ram_mib)} · rec {mib(m.rec_ram_mib)}</span></div>
        </div>
        <div className="security-tags" style={{ marginTop: "1.25rem" }}>
          <span><Tag size={12} /> {m.license}</span><span>ctx {m.context.toLocaleString()}</span><span>trust: {m.trust.replace(/_/g, " ")}</span>
          {m.tags.map((t) => <span key={t}>{t}</span>)}
        </div>
        <div className="hero-actions" style={{ marginTop: "1.75rem" }}>
          <a className="primary-action" href={`${HF}/${m.repo}`} target="_blank" rel="noreferrer"><ExternalLink size={16} /> Hugging Face repo</a>
          <Link href="/advisor" className="quiet-action"><ScrollText size={16} /> Can I train this?</Link>
        </div>
        <p className="download-policy" style={{ marginTop: "1.5rem" }}>Repo <code>{m.repo}</code> · file <code>{m.file}</code>{m.sha256 ? <> · sha256 <code>{m.sha256.slice(0, 16)}…</code></> : null}</p>
      </>}
    </section>
  </PageChrome>;
}

export function DatasetsIndex() {
  const [language, setLanguage] = usePersistedLanguage();
  const { data, loading, error } = useApi<{ datasets: Dataset[] }>("/api/datasets/index.json");
  return <PageChrome language={language} setLanguage={setLanguage}>
    <section className="build-section">
      <div className="build-head"><div><p className="eyebrow"><span className="signal-dot" />CATALOG / DATASETS</p><h2>Datasets</h2></div></div>
      {!data ? <States loading={loading} error={error} /> : <div className="build-grid">{data.datasets.map((d) => (
        <Link key={d.id} href={`/datasets/${d.id}`} className="feature-card feat-beta" style={{ textDecoration: "none" }}>
          <div className="feature-card-top"><Database size={22} /><span className="feat-chip feat-chip-beta">{d.category}</span></div>
          <h3>{d.name}</h3><p>{d.description[language]}</p>
          <p className="download-policy" style={{ marginTop: "auto" }}>{d.num_examples.toLocaleString()} ex · {mib(d.download_mib)} · {d.license}</p>
        </Link>))}</div>}
    </section>
  </PageChrome>;
}

export function DatasetDetail() {
  const [language, setLanguage] = usePersistedLanguage();
  const { slug } = useParams();
  const { data, loading, error } = useApi<{ datasets: Dataset[] }>("/api/datasets/index.json");
  const d = data?.datasets.find((x) => x.id === slug);
  return <PageChrome language={language} setLanguage={setLanguage}>
    <section className="build-section">
      <BackLink href="/datasets" label="All datasets" />
      {!d ? <States loading={loading} error={error} /> : <>
        <p className="eyebrow"><span className="signal-dot" />DATASET / {d.category.toUpperCase()}</p>
        <h2>{d.name}</h2>
        <p className="hero-lead">{d.description[language]}</p>
        <div className="score-sample" style={{ marginTop: "1.5rem" }}>
          <div><b><ScrollText size={14} /> EXAMPLES</b><span>{d.num_examples.toLocaleString()}</span></div>
          <i>·</i>
          <div><b><Download size={14} /> DOWNLOAD</b><span>{mib(d.download_mib)}</span></div>
          <i>·</i>
          <div><b><HardDrive size={14} /> UNPACKED</b><span>{mib(d.unpacked_mib)}</span></div>
        </div>
        <div className="security-tags" style={{ marginTop: "1.25rem" }}>
          <span><Tag size={12} /> {d.license}</span><span>risk: {d.risk.replace(/_/g, " ")}</span>{d.languages.map((l) => <span key={l}>{l}</span>)}
        </div>
        <div className="hero-actions" style={{ marginTop: "1.75rem" }}>
          <a className="primary-action" href={`${HF}/datasets/${d.repo}`} target="_blank" rel="noreferrer"><ExternalLink size={16} /> Hugging Face dataset</a>
        </div>
      </>}
    </section>
  </PageChrome>;
}

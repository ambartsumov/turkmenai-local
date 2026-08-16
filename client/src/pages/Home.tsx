/** Design note — «Горный сигнал»: dark technical editorial layout, cyan signal path, contour texture, asymmetric composition. */
import { ArrowDownRight, ArrowUpRight, Box, CheckCircle2, ChevronRight, Copy, Cpu, Database, Download, Globe2, HeartHandshake, LaptopMinimal, Layers3, LockKeyhole, Network, TerminalSquare } from "lucide-react";
import { Link } from "wouter";
import { useState } from "react";
import { copy, type Language } from "@/i18n";
import { releases } from "@/releases";
import { product, features } from "@/generated/product";

const SUPPORT_ADDRESS = "0x6841c694ed117bf02b7ab7acd75d4e235302fdda";
const ACTIONS_URL = releases.actionsUrl;
// Self-contained assets shipped in client/public/assets — no dependency on any
// external/temporary host, so the site keeps its branding fully offline-capable.
const assetUrl = (filename: string) => `/assets/${filename}`;

function LanguageSwitch({ language, setLanguage }: { language: Language; setLanguage: (value: Language) => void }) {
  return <div className="language-switch" aria-label="Language">{(["en", "ru", "tk"] as Language[]).map((item) => <button key={item} onClick={() => setLanguage(item)} className={language === item ? "active" : ""}>{item.toUpperCase()}</button>)}</div>;
}

function SignalLogo() { return <img className="brand-mark" src={assetUrl("turkmenai-mark.svg")} alt="TurkmenAI" width={32} height={32} />; }

function DownloadMatrix({ t }: { t: Record<string, string> }) {
  return <section id="downloads" className="downloads-section"><div className="downloads-head"><div><p className="eyebrow"><span className="signal-dot" />07 / DISTRIBUTION</p><h2>{t.downloadsTitle}</h2></div><p>{t.downloadsLead}</p></div><div className="download-grid">{releases.assets.map((target) => <article className={`download-card ${target.status}`} key={`${target.os}-${target.arch}-${target.packageName}`}><div className="download-card-top"><LaptopMinimal size={20}/><span>{target.status === "verified" ? t.verified : target.status === "building" ? t.ciTarget : t.planned}</span></div><h3>{target.os}</h3><p>{target.arch} · {target.packageName}</p>{target.status === "verified" && target.url ? <a className="download-action" href={target.url}><Download size={15}/>{t.directDownload}</a> : <a className="build-action" href={ACTIONS_URL} target="_blank" rel="noreferrer">{t.trackBuild}<ArrowUpRight size={15}/></a>}</article>)}</div><p className="download-policy">{t.directPolicy}</p></section>;
}

const FEATURE_ICON: Record<string, typeof Box> = { core: Cpu, models: Box, datasets: Database, downloads: Download, performance: Network, training: Layers3 };

function FeatureGrid({ language }: { language: Language }) {
  // Implemented first (stable → beta → experimental), planned last — the site
  // reflects real status straight from metadata/features.json, never hand-typed.
  const rank: Record<string, number> = { stable: 0, beta: 1, experimental: 2, planned: 3, deprecated: 4, removed: 5 };
  const ordered = [...features].sort((a, b) => (rank[a.status] ?? 9) - (rank[b.status] ?? 9));
  return <div className="build-grid">{ordered.map((f) => {
    const Icon = FEATURE_ICON[f.category] ?? Box;
    return <article key={f.id} className={`feature-card feat-${f.status}`}><div className="feature-card-top"><Icon size={22} /><span className={`feat-chip feat-chip-${f.status}`}>{f.status === "planned" ? "planned" : f.status} · v{f.since}</span></div><h3>{f.name[language]}</h3><p>{f.description[language]}</p></article>;
  })}</div>;
}

export default function Home() {
  const [language, setLanguage] = useState<Language>("en");
  const [copied, setCopied] = useState(false);
  const t = copy[language];
  const copySupportAddress = async () => { await navigator.clipboard?.writeText(SUPPORT_ADDRESS); setCopied(true); window.setTimeout(() => setCopied(false), 2200); };
  return <main className="site-shell">
    <header className="topbar"><a className="brand" href="#top"><SignalLogo /><span>TurkmenAI <em>Local</em></span></a><nav aria-label="Main navigation"><a href="#product">{t.navProduct}</a><a href="#security">{t.navSecurity}</a><a href="#offline">{t.navOffline}</a><a href="#api">{t.navApi}</a><a href="#downloads">{t.navDownloads}</a><a href="#support">{t.navSupport}</a></nav><LanguageSwitch language={language} setLanguage={setLanguage} /></header>
    <section id="top" className="hero-section"><div className="hero-art" aria-hidden="true" /><div className="hero-grid" aria-hidden="true" /><div className="hero-copy"><p className="eyebrow"><span className="signal-dot" />{t.signal}</p><h1>{t.hero}</h1><p className="hero-lead">{t.lead}</p><div className="hero-actions"><a href="#downloads" className="primary-action"><Download size={17} />{t.download}<ArrowDownRight size={16} /></a><Link href="/console" className="quiet-action"><TerminalSquare size={17} />{t.openConsole}<ArrowUpRight size={16} /></Link></div></div><aside className="signal-panel" aria-label="Product principles"><div className="panel-index">01 <span>/ CORE</span></div><div className="status-line"><CheckCircle2 size={17} />{t.offline}</div><div className="status-line"><LockKeyhole size={17} />{t.noTelemetry}</div><div className="status-line"><Network size={17} />{t.localApi}</div></aside><div className="scroll-invite"><span>SCROLL</span><i /></div></section>
    <section id="product" className="method-section"><div className="section-label">02 / METHOD</div><div className="method-intro"><h2>{t.how}</h2><p>{t.howLead}</p></div><div className="method-steps">{[{ number: "01", Icon: Cpu, title: t.stepOne, text: t.stepOneText }, { number: "02", Icon: Layers3, title: t.stepTwo, text: t.stepTwoText }, { number: "03", Icon: TerminalSquare, title: t.stepThree, text: t.stepThreeText }].map(({ number, Icon, title, text }) => <article className="method-card" key={number}><div className="card-top"><span>{number}</span><Icon size={21} /></div><h3>{title}</h3><p>{text}</p><ChevronRight size={19} /></article>)}</div></section>
    <section className="plan-section"><div className="plan-image"><img className="plan-mark" src={assetUrl("turkmenai-mark.svg")} alt={t.executionImage} /><div className="image-label">EXECUTION / PLAN / 01</div></div><div className="plan-copy"><p className="eyebrow"><span className="signal-dot" />03 / PLANNING</p><h2>{t.execution}</h2><p>{t.execLead}</p><div className="score-sample"><div><b>MODEL</b><span>Artifact + dependencies</span></div><i>+</i><div><b>BACKEND</b><span>Capability metadata</span></div><i>+</i><div><b>HARDWARE</b><span>Local memory budget</span></div></div><a href="#api" className="quiet-action">API / ExecutionPlan<ArrowUpRight size={16} /></a></div></section>
    <section id="security" className="security-section"><div className="security-big-number">04</div><div><p className="eyebrow"><span className="signal-dot" />SECURITY / BY DESIGN</p><h2>{t.security}</h2></div><p>{t.securityText}</p><div className="security-tags"><span>Weights only</span><span>Explicit permissions</span><span>Hash verified</span><span>Loopback API</span></div></section>
    <section id="offline" className="offline-section"><div className="offline-art" aria-hidden="true" /><div className="offline-copy"><p className="eyebrow"><span className="signal-dot" />05 / RESILIENCE</p><h2>{t.poorInternet}</h2><p>{t.poorText}</p></div><div className="offline-note"><Database size={22}/><span>Content-addressed local store<br/><b>SHA-256 immutable blobs</b></span></div></section>
    <section id="api" className="build-section"><div className="build-head"><div><p className="eyebrow"><span className="signal-dot" />06 / BUILD</p><h2>{t.learn}</h2></div><span className="release-badge">v{product.version} / {product.channel.toUpperCase()}</span></div><FeatureGrid language={language} /><div className="build-console-cta"><TerminalSquare size={18}/><span>{t.localApi}</span><Link href="/console" className="inline-link">{t.openConsole}<ArrowUpRight size={15}/></Link></div></section>
    <DownloadMatrix t={t} />
    <section id="support" className="support-section"><div className="support-copy"><p className="eyebrow"><span className="signal-dot" />08 / SUPPORT</p><h2>{t.supportTitle}</h2><p>{t.supportLead}</p></div><div className="support-card"><div className="support-card-top"><HeartHandshake size={22}/><span>{t.supportNetwork}</span></div><p>{t.supportAddress}</p><code>{SUPPORT_ADDRESS}</code><button type="button" onClick={copySupportAddress} className="support-copy-button"><Copy size={16}/>{copied ? t.supportCopied : t.supportCopy}</button><small>{t.supportWarning}</small><span className="support-live" aria-live="polite">{copied ? t.supportCopied : ""}</span></div></section>
    <footer><div className="footer-brand"><SignalLogo /><span>TurkmenAI <em>Local</em></span></div><nav className="footer-nav" aria-label="Catalog"><Link href="/releases">Releases</Link><Link href="/models">Models</Link><Link href="/datasets">Datasets</Link><Link href="/advisor">Training Advisor</Link><Link href="/console">Console</Link></nav><p>{t.footer}</p><p className="footer-license">{t.license}</p><p className="footer-diagnostics">v{product.version} · {product.channel} · <a href={`${product.repository}/issues/new?labels=bug&title=${encodeURIComponent(`[bug] v${product.version} — `)}&body=${encodeURIComponent(`\n\n---\n${product.product} v${product.version} (${product.channel})\nUA: ${typeof navigator !== "undefined" ? navigator.userAgent : ""}`)}`} target="_blank" rel="noreferrer">{t.reportBug} <ArrowUpRight size={12} /></a></p></footer>
  </main>;
}

/** Design note — «Горный сигнал»: truthful local-only operational console; native runtime states are explicit and never emulated in the browser. */
import { ArrowLeft, CheckCircle2, ChevronLeft, ChevronRight, CircleDotDashed, Copy, Cpu, Database, Download, Languages, LayoutGrid, LockKeyhole, RadioTower, RefreshCw, ServerCog, Square, TerminalSquare } from "lucide-react";
import { Link } from "wouter";
import { useEffect, useState, type ReactNode } from "react";
import { copy, type Language } from "@/i18n";
import { discoverRuntime, getCatalogAll, getCatalogRecommendations, getDatasetRecommendations, getDesktopStatus, getHardware, startRuntime, stopRuntime, type CatalogSource, type DatasetEvaluation, type DatasetsResult, type FitLevel, type Hardware, type Recommendation, type RecommendationsResult, type RuntimeConfig, type RuntimeStatus } from "@/lib/desktop";

type Priority = "balanced" | "speed" | "quality" | "memory" | "download";
type Section = "overview" | "models" | "datasets" | "runtime";
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

function runtimeMessage(runtime: RuntimeStatus | null, t: Record<string, string>) {
  if (!runtime) return { title: t.runtimeBrowser, detail: t.wizardRuntimeText, state: "browser" };
  if (!runtime.executable_path) return { title: t.runtimeNotInstalled, detail: t.wizardRuntimeText, state: "missing" };
  if (runtime.process?.state === "failed" || runtime.health === "failed") return { title: t.runtimeFailed, detail: runtime.process?.error || t.runtimeHint, state: "failed" };
  if (runtime.health === "ready") return { title: t.runtimeReady, detail: t.runtimeHint, state: "ready" };
  if (runtime.health === "loading") return { title: t.runtimeLoading, detail: t.runtimeHint, state: "loading" };
  if (!runtime.config.model_path) return { title: t.runtimeNotConfigured, detail: t.wizardRuntimeText, state: "missing" };
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
  const hardwareText = hardware ? `${hardware.cpu} · ${Math.round(hardware.ram_mib / 1024)} GB RAM · ${hardware.os}` : loading ? "…" : t.notConnected;
  const proceed = () => step === steps.length - 1 ? finish() : setStep((value) => value + 1);
  return <main className="onboarding-shell"><section className="onboarding-panel"><div className="onboarding-progress"><span>01 / {String(step + 1).padStart(2, "0")}</span><div>{steps.map((title, index) => <i className={index <= step ? "done" : ""} key={title} />)}</div></div><p className="eyebrow"><span className="signal-dot" />{t.wizardEyebrow}</p><h1>{t.wizardTitle}</h1><p className="onboarding-lead">{t.wizardLead}</p><div className="wizard-stage">{step === 0 && <><h2><Languages size={23}/>{t.wizardLanguage}</h2><div className="wizard-choice-row">{(["en", "ru", "tk"] as Language[]).map((value) => <button className={language === value ? "selected" : ""} type="button" onClick={() => setLanguage(value)} key={value}>{value.toUpperCase()}</button>)}</div></>}{step === 1 && <><h2><Cpu size={23}/>{t.wizardHardware}</h2><div className="wizard-report"><span>{t.wizardDetected}</span><strong>{hardwareText}</strong><small>{t.wizardLocalOnly}</small></div></>}{step === 2 && <><h2><CircleDotDashed size={23}/>{t.wizardUseCase}</h2><div className="wizard-priorities">{priorities.map(({ id, label }) => <button type="button" key={id} className={priority === id ? "selected" : ""} onClick={() => setPriority(id)}>{label}{priority === id && <CheckCircle2 size={16}/>}</button>)}</div></>}{step === 3 && <><h2><ServerCog size={23}/>{t.wizardRuntime}</h2><RuntimeBrief language={language} runtime={runtime} checking={runtimeChecking} refresh={refreshRuntime}/></>}</div><div className="wizard-actions"><button type="button" className="wizard-back" onClick={() => setStep((value) => Math.max(value - 1, 0))} disabled={step === 0}><ChevronLeft size={17}/>{t.wizardBack}</button><button type="button" className="wizard-next" onClick={proceed}>{step === steps.length - 1 ? t.wizardFinish : t.wizardContinue}<ChevronRight size={17}/></button></div></section></main>;
}

function ModelCard({ rec, t, language }: { rec: Recommendation; t: Record<string, string>; language: Language }) {
  const [copied, setCopied] = useState(false);
  const m = rec.model; const desc = m.description[language] || m.description.en || "";
  const copyUrl = async () => { await navigator.clipboard?.writeText(rec.download_url); setCopied(true); window.setTimeout(() => setCopied(false), 1800); };
  return <article className={`catalog-card fit-${rec.fit}`}>
    <div className="catalog-card-top"><span className="catalog-cat">{m.category}</span><span className={`fit-badge fit-${rec.fit}`}>{fitLabel(t, rec.fit)}</span></div>
    <h3>{m.name}</h3>
    <p className="catalog-repo">{m.repo}</p>
    {desc && <p className="catalog-desc">{desc}</p>}
    <div className="catalog-meta"><span>{t.sizeLabel}: <b>{gib(m.download_mib)}</b></span><span>{t.ramLabel}: <b>{gib(rec.estimated_ram_mib)}</b></span><span>{t.licenseLabel}: <b>{m.license}</b></span>{m.params_b > 0 && <span><b>{m.params_b}B</b> · {m.quant}</span>}</div>
    {rec.reasons[0] && <p className="catalog-why"><b>{t.whyFits}:</b> {rec.reasons[0]}</p>}
    <div className="catalog-actions"><a className="download-action" href={rec.download_url} target="_blank" rel="noreferrer"><Download size={15}/>Hugging Face</a><button type="button" className="runtime-refresh" onClick={copyUrl}><Copy size={13}/>{copied ? t.copied : t.copyLink}</button></div>
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
    {loading ? <div className="catalog-empty"><RefreshCw size={22} className="spin"/></div> : (result && result.recommendations.length > 0 ? <div className="catalog-grid">{result.recommendations.map((rec) => <ModelCard key={rec.model.id} rec={rec} t={t} language={language}/>)}</div> : <div className="catalog-empty"><p>{t.noModels}</p></div>)}
    {result && result.recommendations.length > 0 && <button type="button" className="catalog-toggle" onClick={toggleIncompatible}>{showIncompatible ? t.hideIncompatible : t.showIncompatible}</button>}
    {showIncompatible && incompatible.length > 0 && <div className="catalog-grid dimmed">{incompatible.map((rec) => <ModelCard key={rec.model.id} rec={rec} t={t} language={language}/>)}</div>}
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

export default function Console() {
  const [language, setLanguageState] = useState<Language>(detectInitialLanguage); const [hardware, setHardware] = useState<Hardware | null>(null); const [connected, setConnected] = useState(false); const [loading, setLoading] = useState(true); const [firstRun, setFirstRun] = useState(() => typeof window !== "undefined" && !window.localStorage.getItem(WIZARD_KEY)); const [runtime, setRuntime] = useState<RuntimeStatus | null>(null); const [runtimeConfig, setRuntimeConfig] = useState<RuntimeConfig>(DEFAULT_RUNTIME_CONFIG); const [runtimeChecking, setRuntimeChecking] = useState(false); const [runtimeError, setRuntimeError] = useState<string | null>(null); const [section, setSection] = useState<Section>("overview"); const t = copy[language];
  const setLanguage = (value: Language) => { setLanguageState(value); window.localStorage.setItem("turkmenai.language", value); };
  const syncRuntime = async () => { setRuntimeChecking(true); setRuntimeError(null); try { const next = await discoverRuntime(); setRuntime(next); if (next) setRuntimeConfig(next.config); } catch (error) { setRuntimeError(error instanceof Error ? error.message : t.runtimeStartError); } finally { setRuntimeChecking(false); } };
  useEffect(() => { Promise.all([getDesktopStatus(), getHardware(), discoverRuntime()]).then(([status, profile, nextRuntime]) => { setConnected(Boolean(status)); setHardware(profile); setRuntime(nextRuntime); if (nextRuntime) setRuntimeConfig(nextRuntime.config); }).finally(() => setLoading(false)); }, []);
  const finish = () => { window.localStorage.setItem(WIZARD_KEY, "complete"); setFirstRun(false); };
  const updateRuntimeConfig = (field: keyof RuntimeConfig, value: string | number | null) => setRuntimeConfig((current) => ({ ...current, [field]: value }));
  const handleStart = async () => { setRuntimeChecking(true); setRuntimeError(null); try { const next = await startRuntime(runtimeConfig); if (!next) throw new Error(t.runtimeBrowser); setRuntime(next); setRuntimeConfig(next.config); } catch (error) { setRuntimeError(error instanceof Error ? error.message : t.runtimeStartError); } finally { setRuntimeChecking(false); } };
  const handleStop = async () => { setRuntimeChecking(true); setRuntimeError(null); try { const next = await stopRuntime(); setRuntime(next); } catch (error) { setRuntimeError(error instanceof Error ? error.message : t.runtimeStartError); } finally { setRuntimeChecking(false); } };
  if (firstRun) return <FirstRunWizard language={language} setLanguage={setLanguage} hardware={hardware} loading={loading} runtime={runtime} runtimeChecking={runtimeChecking} refreshRuntime={syncRuntime} finish={finish} />;
  const title = connected ? t.nativeReady : t.serviceMissing; const detail = connected ? t.nativeText : t.serviceText; const hardwareText = hardware ? `${hardware.cpu} · ${Math.round(hardware.ram_mib / 1024)} GB RAM · ${hardware.os}` : loading ? "…" : t.serviceText; const runtimeState = runtimeMessage(runtime, t);
  const navItem = (id: Section, icon: ReactNode, label: string) => <a className={section === id ? "nav-active" : ""} onClick={() => setSection(id)} role="button" tabIndex={0}>{icon}{label}</a>;
  return <main className="console-shell"><header className="console-header"><Link href="/" className="back-link"><ArrowLeft size={17}/>{t.back}</Link><div className="console-brand"><img src="/assets/turkmenai-mark.svg" alt="" width={26} height={26}/>TurkmenAI <em>Local</em></div><div className="language-switch">{(["en", "ru", "tk"] as Language[]).map((item) => <button key={item} onClick={() => setLanguage(item)} className={language === item ? "active" : ""}>{item.toUpperCase()}</button>)}</div></header><div className="console-layout"><aside className="console-nav"><span>LOCAL CONTROL</span>{navItem("overview", <TerminalSquare size={17}/>, t.overview)}{navItem("models", <LayoutGrid size={17}/>, t.models)}{navItem("datasets", <Database size={17}/>, t.datasets)}{navItem("runtime", <ServerCog size={17}/>, t.runtimes)}<a><RadioTower size={17}/>{t.api}</a></aside><section className="console-main">
    {section === "models" && <ModelsPanel language={language} priority="balanced"/>}
    {section === "datasets" && <DatasetsPanel language={language}/>}
    {(section === "overview" || section === "runtime") && <>
    <div className="console-intro"><p className="eyebrow"><span className="signal-dot"/>127.0.0.1 ONLY</p><h1>{t.consoleTitle}</h1><p>{t.consoleLead}</p></div><div className="console-status"><div className="status-orb"><CircleDotDashed size={32}/></div><div><p>{title}</p><span>{detail}</span></div><code>{connected ? "native://core" : "tmai server"}</code></div><div className="console-cards"><article><div className="console-card-label">{t.hardwareFound}</div><b>{connected ? t.localProfile : t.notConnected}</b><span>{hardwareText}</span></article><article><div className="console-card-label">{t.privacy}</div><div className="privacy-lines"><span><LockKeyhole size={14}/>{t.telemetryOff}</span><span><LockKeyhole size={14}/>{t.cloudOff}</span><span><LockKeyhole size={14}/>{t.lanOff}</span></div></article><article><div className="console-card-label">RUNTIME</div><b>{runtimeState.title}</b><span>{runtimeState.detail}</span></article></div>{runtime && <section className="runtime-control"><div className="runtime-control-head"><div><span>{t.runtimeConfigure}</span><strong>{runtimeState.title}</strong></div><button type="button" className="runtime-refresh" onClick={syncRuntime} disabled={runtimeChecking}><RefreshCw size={14} className={runtimeChecking ? "spin" : ""}/>{runtimeChecking ? t.runtimeChecking : t.runtimeRefresh}</button></div><div className="runtime-inputs"><label>{t.runtimeExecutable}<input value={runtimeConfig.executable_path || ""} onChange={(event) => updateRuntimeConfig("executable_path", event.target.value || null)} placeholder="/usr/local/bin/llama-server"/></label><label>{t.runtimeModel}<input value={runtimeConfig.model_path || ""} onChange={(event) => updateRuntimeConfig("model_path", event.target.value || null)} placeholder="/path/to/model.gguf"/></label><label>{t.runtimePort}<input type="number" min="1" max="65535" value={runtimeConfig.port} onChange={(event) => updateRuntimeConfig("port", Number(event.target.value) || 8080)}/></label></div><p>{t.runtimeHint}</p>{runtimeError && <output className="runtime-error">{runtimeError}</output>}<div className="runtime-actions"><button type="button" className="wizard-next" onClick={handleStart} disabled={runtimeChecking}><ServerCog size={16}/>{t.runtimeStart}</button>{runtime.process && <button type="button" className="runtime-stop" onClick={handleStop} disabled={runtimeChecking}><Square size={14}/>{t.runtimeStop}</button>}</div></section>}<div className="console-command"><div><span>{t.nextStep}</span><strong>{t.nextStepText}</strong></div><code>tmai server 8742</code></div>
    </>}
  </section></div></main>;
}

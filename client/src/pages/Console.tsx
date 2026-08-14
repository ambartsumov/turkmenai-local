/** Design note — «Горный сигнал»: a truthful local-only console, designed as quiet operations software rather than a generic dashboard. */
import { ArrowLeft, CircleDotDashed, Cpu, LockKeyhole, RadioTower, ServerCog, TerminalSquare } from "lucide-react";
import { Link } from "wouter";
import { useEffect, useState } from "react";
import { copy, type Language } from "@/i18n";
import { getDesktopStatus, getHardware, type Hardware } from "@/lib/desktop";

export default function Console() {
  const [language, setLanguage] = useState<Language>("ru");
  const [hardware, setHardware] = useState<Hardware | null>(null);
  const [connected, setConnected] = useState(false);
  const [loading, setLoading] = useState(true);
  const t = copy[language];
  useEffect(() => { Promise.all([getDesktopStatus(), getHardware()]).then(([status, profile]) => { setConnected(Boolean(status)); setHardware(profile); }).finally(() => setLoading(false)); }, []);
  const title = connected ? t.nativeReady : t.serviceMissing;
  const detail = connected ? t.nativeText : t.serviceText;
  const hardwareText = hardware ? `${hardware.cpu} · ${Math.round(hardware.ram_mib / 1024)} GB RAM · ${hardware.os}` : loading ? "…" : t.serviceText;
  return <main className="console-shell"><header className="console-header"><Link href="/" className="back-link"><ArrowLeft size={17}/>{t.back}</Link><div className="console-brand"><img src="/manus-storage/turkmenai-logo-symbol_d3087e01.png" alt=""/>TurkmenAI <em>Local</em></div><div className="language-switch">{(["ru", "tk", "en"] as Language[]).map((item) => <button key={item} onClick={() => setLanguage(item)} className={language === item ? "active" : ""}>{item.toUpperCase()}</button>)}</div></header><div className="console-layout"><aside className="console-nav"><span>LOCAL CONTROL</span><a className="nav-active"><TerminalSquare size={17}/>{t.overview}</a><a><Cpu size={17}/>{t.hardware}</a><a><ServerCog size={17}/>{t.runtimes}</a><a><RadioTower size={17}/>{t.api}</a></aside><section className="console-main"><div className="console-intro"><p className="eyebrow"><span className="signal-dot"/>127.0.0.1 ONLY</p><h1>{t.consoleTitle}</h1><p>{t.consoleLead}</p></div><div className="console-status"><div className="status-orb"><CircleDotDashed size={32}/></div><div><p>{title}</p><span>{detail}</span></div><code>{connected ? "native://core" : "tmai server"}</code></div><div className="console-cards"><article><div className="console-card-label">{t.hardwareFound}</div><b>{connected ? t.localProfile : t.notConnected}</b><span>{hardwareText}</span></article><article><div className="console-card-label">{t.privacy}</div><div className="privacy-lines"><span><LockKeyhole size={14}/>{t.telemetryOff}</span><span><LockKeyhole size={14}/>{t.cloudOff}</span><span><LockKeyhole size={14}/>{t.lanOff}</span></div></article><article><div className="console-card-label">EXECUTION</div><b>{t.noActivePlan}</b><span>{t.noActivePlanText}</span></article></div><div className="console-command"><div><span>{t.nextStep}</span><strong>{t.nextStepText}</strong></div><code>tmai server 8742</code></div></section></div></main>;
}

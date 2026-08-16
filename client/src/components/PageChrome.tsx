import { ArrowUpLeft } from "lucide-react";
import { Link } from "wouter";
import type { Language } from "@/i18n";
import type { ReactNode } from "react";

const assetUrl = (filename: string) => `/assets/${filename}`;

/** Shared header for the metadata-driven catalog pages: brand, back link, language switch. */
export function PageChrome({ language, setLanguage, children }: { language: Language; setLanguage: (v: Language) => void; children: ReactNode }) {
  return (
    <main className="site-shell">
      <header className="topbar">
        <Link className="brand" href="/"><img className="brand-mark" src={assetUrl("turkmenai-mark.svg")} alt="TurkmenAI" width={32} height={32} /><span>TurkmenAI <em>Local</em></span></Link>
        <nav aria-label="Catalog navigation"><Link href="/#downloads">Downloads</Link><Link href="/releases">Releases</Link><Link href="/models">Models</Link><Link href="/datasets">Datasets</Link><Link href="/advisor">Training</Link></nav>
        <div className="language-switch" aria-label="Language">{(["en", "ru", "tk"] as Language[]).map((item) => <button key={item} onClick={() => setLanguage(item)} className={language === item ? "active" : ""}>{item.toUpperCase()}</button>)}</div>
      </header>
      {children}
    </main>
  );
}

export function BackLink({ href, label }: { href: string; label: string }) {
  return <Link href={href} className="quiet-action" style={{ marginBottom: "1.5rem" }}><ArrowUpLeft size={16} />{label}</Link>;
}

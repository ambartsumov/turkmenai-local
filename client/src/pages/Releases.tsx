/** Release archive: /releases (history) and /releases/:tag (assets + checksums).
 *  Renders from generated /api/releases/*.json — real published assets only. */
import { Download, ShieldCheck, Package, Clock } from "lucide-react";
import { Link, useParams } from "wouter";
import { PageChrome, BackLink } from "@/components/PageChrome";
import { useApi, usePersistedLanguage } from "@/lib/useApi";

const size = (n: number) => (n >= 1 << 20 ? `${(n / (1 << 20)).toFixed(1)} MB` : `${(n / 1024).toFixed(0)} KB`);
const when = (iso: string) => new Date(iso).toISOString().slice(0, 10);

type IndexRelease = { version: string; tag: string; channel: string; published_at: string; url: string };
type Artifact = { package: string; url: string; size: number; sha256: string | null };
type Platform = { platform: string; status: string; artifacts: Artifact[] };
type Latest = { product?: string; version: string; tag: string; channel: string; published_at: string; release_url: string; platforms: Platform[] };

export function ReleasesIndex() {
  const [language, setLanguage] = usePersistedLanguage();
  const { data, loading, error } = useApi<{ latest: { tag: string }; releases: IndexRelease[] }>("/api/releases/index.json");
  return <PageChrome language={language} setLanguage={setLanguage}>
    <section className="build-section">
      <div className="build-head"><div><p className="eyebrow"><span className="signal-dot" />DISTRIBUTION / RELEASES</p><h2>Releases</h2></div></div>
      {!data ? <p className="hero-lead">{loading ? "Loading…" : `Not found${error ? ` — ${error}` : ""}.`}</p> :
        <div className="build-grid">{data.releases.map((r) => (
          <Link key={r.tag} href={`/releases/${r.tag}`} className={`feature-card ${r.tag === data.latest.tag ? "feat-stable" : "feat-beta"}`} style={{ textDecoration: "none" }}>
            <div className="feature-card-top"><Package size={22} /><span className={`feat-chip feat-chip-${r.tag === data.latest.tag ? "stable" : "beta"}`}>{r.tag === data.latest.tag ? "latest" : r.channel}</span></div>
            <h3>{r.tag}</h3><p><Clock size={13} /> {when(r.published_at)} · {r.channel}</p>
          </Link>))}</div>}
    </section>
  </PageChrome>;
}

export function ReleaseDetail() {
  const [language, setLanguage] = usePersistedLanguage();
  const { tag } = useParams();
  // Only the latest release ships a full asset manifest; older tags link out to GitHub.
  const latest = useApi<Latest>("/api/releases/latest.json");
  const isLatest = latest.data?.tag === tag;
  return <PageChrome language={language} setLanguage={setLanguage}>
    <section className="build-section">
      <BackLink href="/releases" label="All releases" />
      <p className="eyebrow"><span className="signal-dot" />RELEASE / {tag}</p>
      <h2>{tag}</h2>
      {isLatest && latest.data ? <>
        <p className="hero-lead">{latest.data.product ?? "TurkmenAI Local"} {latest.data.version} · {latest.data.channel} · published {when(latest.data.published_at)}</p>
        {latest.data.platforms.filter((p) => p.status === "verified").map((p) => (
          <div key={p.platform} style={{ marginTop: "1.5rem" }}>
            <p className="eyebrow" style={{ marginBottom: ".5rem" }}>{p.platform}</p>
            <div className="download-grid">{p.artifacts.map((a) => (
              <article className="download-card verified" key={a.url}>
                <div className="download-card-top"><ShieldCheck size={18} /><span>{size(a.size)}</span></div>
                <h3>{a.package}</h3>
                {a.sha256 ? <p style={{ fontFamily: "monospace", fontSize: ".72rem", wordBreak: "break-all" }}>{a.sha256.slice(0, 24)}…</p> : null}
                <a className="download-action" href={a.url}><Download size={15} /> Download</a>
              </article>))}</div>
          </div>))}
        <p className="download-policy" style={{ marginTop: "1.5rem" }}>Every checksum is read from the published <code>SHA256SUMS.txt</code>. <a href={latest.data.release_url} target="_blank" rel="noreferrer">GitHub release ↗</a></p>
      </> : <p className="hero-lead">{latest.loading ? "Loading…" : <>Full asset manifest is kept for the latest release. <a href={`https://github.com/ambartsumov/turkmenai-local/releases/tag/${tag}`} target="_blank" rel="noreferrer">Open {tag} on GitHub ↗</a></>}</p>}
    </section>
  </PageChrome>;
}

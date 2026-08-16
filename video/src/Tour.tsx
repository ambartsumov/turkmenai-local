import React from "react";
import { AbsoluteFill, Sequence, interpolate, spring, useCurrentFrame, useVideoConfig, Easing } from "remotion";
import { T, FPS } from "./theme";
import { tri, type Lang } from "./copy";

// Snappy spring for energetic pacing.
const pop = (frame: number, fps: number, delay = 0) => spring({ frame: frame - delay, fps, config: { damping: 16, stiffness: 140, mass: 0.6 } });

const Bg: React.FC = () => {
  const f = useCurrentFrame();
  const drift = interpolate(f, [0, 40 * FPS], [0, -50]);
  return (
    <AbsoluteFill style={{ background: `radial-gradient(120% 70% at 80% 10%, ${T.signalDim}, transparent 55%), linear-gradient(160deg, ${T.ink}, ${T.deep})` }}>
      <AbsoluteFill style={{ backgroundImage: `linear-gradient(rgba(49,216,199,.05) 1px, transparent 1px), linear-gradient(90deg, rgba(49,216,199,.05) 1px, transparent 1px)`, backgroundSize: "64px 64px", transform: `translateY(${drift}px)`, maskImage: "linear-gradient(180deg, transparent, black 22%, black 82%, transparent)" }} />
    </AbsoluteFill>
  );
};

const Mark: React.FC<{ size?: number }> = ({ size = 54 }) => (
  <svg width={size} height={size} viewBox="0 0 48 48" fill="none">
    <path d="M6 30 L18 30 L24 12 L30 36 L36 24 L42 24" stroke={T.signal} strokeWidth={3} strokeLinecap="round" strokeLinejoin="round" />
    <circle cx="24" cy="24" r="21" stroke={T.signal} strokeOpacity={0.35} strokeWidth={2} />
  </svg>
);

// A beat = one headline (top) + one focused visual (center) + one short line.
// Single eye path, no competing paragraphs.
const Beat: React.FC<{ index: string; head: string; line: string; children: React.ReactNode }> = ({ index, head, line, children }) => {
  const f = useCurrentFrame();
  const { fps } = useVideoConfig();
  const h = pop(f, fps, 2);
  const under = interpolate(f, [6, 22], [0, 1], { extrapolateLeft: "clamp", extrapolateRight: "clamp", easing: Easing.out(Easing.cubic) });
  const vis = pop(f, fps, 10);
  const lineOp = interpolate(f, [18, 30], [0, 1], { extrapolateLeft: "clamp", extrapolateRight: "clamp" });
  return (
    <AbsoluteFill style={{ padding: "0 80px" }}>
      {/* headline */}
      <div style={{ position: "absolute", top: 210, left: 80, right: 80 }}>
        <div style={{ display: "flex", alignItems: "center", gap: 14, color: T.signal, font: `700 24px ${T.mono}`, letterSpacing: 3, opacity: h }}>
          <span style={{ width: 14, height: 14, borderRadius: 99, background: T.signal, boxShadow: `0 0 22px ${T.signal}` }} /> {index} / 09
        </div>
        <div style={{ marginTop: 18, font: `800 82px ${T.sans}`, color: T.fog, letterSpacing: -3, lineHeight: 1.0, opacity: h, transform: `translateX(${interpolate(h, [0, 1], [-36, 0])}px)` }}>{head}</div>
        <div style={{ height: 5, width: `${under * 100}%`, maxWidth: 520, background: T.signal, marginTop: 20, boxShadow: `0 0 16px ${T.signal}` }} />
      </div>
      {/* focused visual */}
      <div style={{ position: "absolute", top: 560, left: 80, right: 80, height: 800, display: "flex", alignItems: "center", justifyContent: "center", opacity: vis, transform: `scale(${interpolate(vis, [0, 1], [0.9, 1])})` }}>{children}</div>
      {/* one-line caption */}
      <div style={{ position: "absolute", bottom: 250, left: 80, right: 80, textAlign: "center", font: `600 40px ${T.sans}`, color: "#c3d2d3", opacity: lineOp }}>{line}</div>
    </AbsoluteFill>
  );
};

const card: React.CSSProperties = { border: `1px solid ${T.line}`, background: T.panel2, padding: 26, borderRadius: 10, width: "100%" };
const chip = (c: string): React.CSSProperties => ({ font: `700 20px ${T.mono}`, color: c, border: `1px solid ${c}`, padding: "8px 12px", borderRadius: 6, letterSpacing: 1 });
const mono = (s: number, c = "#81969a"): React.CSSProperties => ({ font: `700 ${s}px ${T.mono}`, color: c, letterSpacing: 2 });

// ---- focused visuals -------------------------------------------------------

const VisHardware: React.FC = () => {
  const f = useCurrentFrame();
  const items = [["CPU", 0], ["RAM", 6], ["GPU / VRAM", 12], ["DISK", 18]] as const;
  return (
    <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 18, width: "100%" }}>
      {items.map(([k, d]) => {
        const w = interpolate(f, [14 + d, 34 + d], [0, 100], { extrapolateLeft: "clamp", extrapolateRight: "clamp" });
        return (
          <div key={k} style={{ ...card, minHeight: 170 }}>
            <div style={mono(19)}>{k}</div>
            <div style={{ marginTop: 26, height: 12, background: "#12333a", borderRadius: 6, overflow: "hidden" }}><div style={{ height: "100%", width: `${w}%`, background: `linear-gradient(90deg,#00c2b8,${T.signal})` }} /></div>
            <div style={{ marginTop: 14, font: `500 22px ${T.sans}`, color: "#9baeb2" }}>detected locally</div>
          </div>
        );
      })}
    </div>
  );
};

const ModelRow: React.FC<{ name: string; cat: string; fit: string; on: boolean }> = ({ name, cat, fit, on }) => (
  <div style={{ ...card, borderColor: on ? T.signal : T.line, boxShadow: on ? `0 0 0 2px ${T.signal}` : "none" }}>
    <div style={{ display: "flex", alignItems: "center" }}>
      <span style={{ ...mono(18), textTransform: "uppercase" }}>{cat}</span>
      <span style={{ marginLeft: "auto", ...chip(fit === "EXCELLENT" ? T.signal : T.sand), fontSize: 16 }}>{fit}</span>
    </div>
    <div style={{ marginTop: 12, font: `700 34px ${T.sans}`, color: T.fog }}>{name}</div>
  </div>
);
const VisModels: React.FC = () => (
  <div style={{ display: "grid", gap: 16, width: "100%" }}>
    <ModelRow name="Qwen2.5 3B Instruct" cat="chat" fit="EXCELLENT" on />
    <ModelRow name="Whisper Small" cat="speech" fit="GOOD" on={false} />
    <ModelRow name="Gemma 2 2B" cat="chat" fit="EXCELLENT" on={false} />
  </div>
);

const VisDatasets: React.FC = () => (
  <div style={{ display: "grid", gap: 16, width: "100%" }}>
    {[["Alpaca", "instruction", "CC-BY-NC"], ["Turkmen ASR", "speech", "CC-BY"], ["OpenAssistant", "chat", "Apache-2.0"]].map(([n, c, lic]) => (
      <div key={n} style={card}>
        <div style={{ display: "flex", alignItems: "center" }}>
          <span style={{ ...mono(18), textTransform: "uppercase" }}>{c}</span>
          <span style={{ marginLeft: "auto", ...chip(T.sand), fontSize: 15 }}>{lic}</span>
        </div>
        <div style={{ marginTop: 12, font: `700 32px ${T.sans}`, color: T.fog }}>{n}</div>
      </div>
    ))}
  </div>
);

const InstallCard: React.FC<{ lang: Lang; resume: boolean }> = ({ lang, resume }) => {
  const f = useCurrentFrame();
  // resume beat stalls at ~45% then jumps; install beat fills smoothly.
  const pct = resume
    ? Math.round(interpolate(f, [10, 40, 58, 90], [0, 45, 45, 100], { extrapolateRight: "clamp", easing: Easing.inOut(Easing.quad) }))
    : Math.round(interpolate(f, [10, 70], [0, 100], { extrapolateRight: "clamp", easing: Easing.inOut(Easing.quad) }));
  const reconnecting = resume && f > 40 && f < 60;
  const done = pct >= 100;
  return (
    <div style={{ ...card }}>
      <div style={{ display: "flex", alignItems: "center" }}>
        <span style={{ ...mono(18), textTransform: "uppercase" }}>chat</span>
        <span style={{ marginLeft: "auto", ...chip(T.signal), fontSize: 16 }}>EXCELLENT</span>
      </div>
      <div style={{ marginTop: 12, font: `700 34px ${T.sans}`, color: T.fog }}>Qwen2.5 3B Instruct</div>
      <div style={{ marginTop: 22, height: 14, background: "#12333a", borderRadius: 7, overflow: "hidden" }}>
        <div style={{ height: "100%", width: `${pct}%`, background: reconnecting ? "#e0a94a" : `linear-gradient(90deg,#00c2b8,${T.signal})` }} />
      </div>
      <div style={{ marginTop: 16, display: "flex", alignItems: "center", gap: 16, ...mono(22, "#8fb3b3") }}>
        <span>{done ? tri("ready", lang) : `${pct}%`}</span>
        {reconnecting ? <span style={{ color: "#e0a94a" }}>↻ {tri("reconnect", lang)}</span> : <span>{done ? "SHA-256 ✓" : "resumable"}</span>}
      </div>
      <div style={{ marginTop: 22, alignSelf: "flex-start", background: done ? "#0c2a30" : T.signal, color: done ? T.signal : "#06232a", font: `800 26px ${T.sans}`, padding: "16px 24px", borderRadius: 6, display: "inline-block" }}>{done ? `✓ ${tri("ready", lang)}` : tri("install", lang)}</div>
    </div>
  );
};

const VisPrivate: React.FC = () => {
  const f = useCurrentFrame();
  const rows = [["127.0.0.1 ONLY", T.signal], ["NO TELEMETRY", T.signal], ["NO CLOUD", T.signal]] as const;
  return (
    <div style={{ display: "grid", gap: 18, width: "100%" }}>
      {rows.map(([t, c], i) => {
        const s = pop(f, FPS, 8 + i * 8);
        return <div key={t} style={{ ...card, display: "flex", alignItems: "center", gap: 18, opacity: s, transform: `translateX(${interpolate(s, [0, 1], [-30, 0])}px)` }}><span style={{ color: c, font: `800 30px ${T.mono}` }}>✓</span><span style={{ font: `700 34px ${T.mono}`, color: T.fog, letterSpacing: 2 }}>{t}</span></div>;
      })}
    </div>
  );
};

const VisOffline: React.FC = () => {
  const f = useCurrentFrame();
  const cut = f > 20;
  return (
    <div style={{ display: "grid", gap: 26, justifyItems: "center", width: "100%" }}>
      <svg width="180" height="180" viewBox="0 0 24 24" fill="none">
        <path d="M2 8.5C7 4 17 4 22 8.5" stroke={cut ? "#3a4b52" : T.signal} strokeWidth="1.6" strokeLinecap="round" />
        <path d="M5 12C8.5 9 15.5 9 19 12" stroke={cut ? "#3a4b52" : T.signal} strokeWidth="1.6" strokeLinecap="round" />
        <path d="M8 15.5C10 14 14 14 16 15.5" stroke={cut ? "#3a4b52" : T.signal} strokeWidth="1.6" strokeLinecap="round" />
        <circle cx="12" cy="19" r="1.4" fill={cut ? "#3a4b52" : T.signal} />
        {cut && <path d="M4 4 L20 20" stroke={T.sand} strokeWidth="1.8" strokeLinecap="round" />}
      </svg>
      <div style={{ ...card, textAlign: "center", background: "#0c2a30", borderColor: T.signal }}>
        <span style={{ font: `700 30px ${T.mono}`, color: T.signal, letterSpacing: 2 }}>● llama.cpp · 127.0.0.1 · running</span>
      </div>
    </div>
  );
};

const VisBench: React.FC = () => {
  const f = useCurrentFrame();
  const metrics = ["Tokens / sec", "First token", "CPU / RAM"];
  return (
    <div style={{ display: "grid", gap: 16, width: "100%" }}>
      {metrics.map((m, i) => {
        const w = interpolate(f, [10 + i * 8, 40 + i * 8], [0, [82, 58, 70][i]], { extrapolateLeft: "clamp", extrapolateRight: "clamp" });
        return <div key={m} style={card}>
          <div style={{ display: "flex", justifyContent: "space-between", ...mono(19) }}><span>{m}</span><span style={{ color: T.signal }}>measured on device</span></div>
          <div style={{ marginTop: 16, height: 16, background: "#12333a", borderRadius: 8, overflow: "hidden" }}><div style={{ height: "100%", width: `${w}%`, background: `linear-gradient(90deg,#00c2b8,${T.signal})` }} /></div>
        </div>;
      })}
    </div>
  );
};

const VisLang: React.FC = () => {
  const f = useCurrentFrame();
  const active = Math.floor(f / 22) % 3; // cycle EN→RU→TK
  const langs = ["EN", "RU", "TK"];
  const words = ["Chat", "Чат", "Söhbet"];
  return (
    <div style={{ display: "grid", gap: 30, justifyItems: "center", width: "100%" }}>
      <div style={{ display: "flex", gap: 12 }}>{langs.map((l, i) => <div key={l} style={{ font: `800 30px ${T.mono}`, padding: "14px 20px", borderRadius: 6, background: i === active ? T.signal : "transparent", color: i === active ? "#052228" : "#9eb0b3", border: `1px solid ${i === active ? T.signal : T.line}` }}>{l}</div>)}</div>
      <div style={{ ...card, textAlign: "center", minWidth: 360 }}><span style={{ font: `800 56px ${T.sans}`, color: T.fog }}>{words[active]}</span></div>
    </div>
  );
};

// ---- intro / cta -----------------------------------------------------------

const Intro: React.FC<{ lang: Lang }> = ({ lang }) => {
  const f = useCurrentFrame();
  const { fps } = useVideoConfig();
  const s = pop(f, fps);
  const line = interpolate(f, [8, 30], [0, 1], { extrapolateRight: "clamp" });
  return (
    <AbsoluteFill style={{ justifyContent: "center", alignItems: "center", padding: 80 }}>
      <div style={{ transform: `scale(${interpolate(s, [0, 1], [0.5, 1])})`, opacity: s }}><Mark size={150} /></div>
      <div style={{ marginTop: 40, font: `800 96px ${T.sans}`, color: T.fog, letterSpacing: -4, textAlign: "center" }}>TurkmenAI <span style={{ color: T.signal }}>Local</span></div>
      <div style={{ height: 4, width: interpolate(line, [0, 1], [0, 480]), background: T.signal, margin: "34px 0", boxShadow: `0 0 20px ${T.signal}` }} />
      <div style={{ font: `500 42px ${T.sans}`, color: "#c3d2d3", textAlign: "center", opacity: interpolate(f, [20, 40], [0, 1], { extrapolateRight: "clamp" }) }}>{tri("tagline", lang)}</div>
      <div style={{ marginTop: 24, ...mono(24, T.signal), letterSpacing: 5, opacity: interpolate(f, [30, 50], [0, 1], { extrapolateRight: "clamp" }) }}>{tri("kicker", lang)}</div>
    </AbsoluteFill>
  );
};

const CTA: React.FC<{ lang: Lang }> = ({ lang }) => {
  const f = useCurrentFrame();
  const { fps } = useVideoConfig();
  const s = pop(f, fps);
  const pulse = 1 + 0.035 * Math.sin(f / 5);
  return (
    <AbsoluteFill style={{ justifyContent: "center", alignItems: "center", padding: 80 }}>
      <div style={{ opacity: s, transform: `scale(${interpolate(s, [0, 1], [0.6, 1])})` }}><Mark size={130} /></div>
      <div style={{ marginTop: 28, font: `800 78px ${T.sans}`, color: T.fog, letterSpacing: -3, textAlign: "center" }}>TurkmenAI <span style={{ color: T.signal }}>Local</span></div>
      <div style={{ marginTop: 40, background: T.signal, color: "#04222a", font: `800 46px ${T.sans}`, padding: "26px 46px", borderRadius: 8, transform: `scale(${pulse})` }}>{tri("cta", lang)}</div>
      <div style={{ marginTop: 30, ...mono(44, T.signal), letterSpacing: 2 }}>turkmenai.tech</div>
      <div style={{ marginTop: 18, ...mono(22, T.muted), letterSpacing: 3 }}>WINDOWS · macOS · LINUX · v0.3.0</div>
    </AbsoluteFill>
  );
};

// ---- timeline (fast beats) -------------------------------------------------

const ProgressRail: React.FC = () => {
  const f = useCurrentFrame();
  const { durationInFrames } = useVideoConfig();
  const w = interpolate(f, [0, durationInFrames], [0, 100]);
  return <div style={{ position: "absolute", left: 0, right: 0, top: 0, height: 6, background: "rgba(255,255,255,.06)" }}><div style={{ height: "100%", width: `${w}%`, background: T.signal }} /></div>;
};

export const Tour: React.FC<{ lang: Lang }> = ({ lang }) => {
  const B = 100; // beat length in frames (~3.3s) — snappy
  const start = 70; // after intro
  const beats: { i: string; h: string; c: string; v: React.ReactNode }[] = [
    { i: "01", h: tri("h_hw", lang), c: tri("c_hw", lang), v: <VisHardware /> },
    { i: "02", h: tri("h_models", lang), c: tri("c_models", lang), v: <VisModels /> },
    { i: "03", h: tri("h_data", lang), c: tri("c_data", lang), v: <VisDatasets /> },
    { i: "04", h: tri("h_install", lang), c: tri("c_install", lang), v: <InstallCard lang={lang} resume={false} /> },
    { i: "05", h: tri("h_resume", lang), c: tri("c_resume", lang), v: <InstallCard lang={lang} resume /> },
    { i: "06", h: tri("h_private", lang), c: tri("c_private", lang), v: <VisPrivate /> },
    { i: "07", h: tri("h_offline", lang), c: tri("c_offline", lang), v: <VisOffline /> },
    { i: "08", h: tri("h_bench", lang), c: tri("c_bench", lang), v: <VisBench /> },
    { i: "09", h: tri("h_lang", lang), c: tri("c_lang", lang), v: <VisLang /> },
  ];
  return (
    <AbsoluteFill style={{ fontFamily: T.sans, background: T.ink }}>
      <Bg />
      <Sequence durationInFrames={start}><Intro lang={lang} /></Sequence>
      {beats.map((b, idx) => (
        <Sequence key={b.i} from={start + idx * B} durationInFrames={B}>
          <Beat index={b.i} head={b.h} line={b.c}>{b.v}</Beat>
        </Sequence>
      ))}
      <Sequence from={start + beats.length * B}><CTA lang={lang} /></Sequence>
      <ProgressRail />
    </AbsoluteFill>
  );
};

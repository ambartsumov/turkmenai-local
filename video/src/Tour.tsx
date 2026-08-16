import React from "react";
import { AbsoluteFill, Sequence, interpolate, spring, useCurrentFrame, useVideoConfig, Easing } from "remotion";
import { T, FPS } from "./theme";
import { tri, type Lang } from "./copy";

// ---------- shared atoms ----------------------------------------------------

const Bg: React.FC = () => {
  const f = useCurrentFrame();
  const drift = interpolate(f, [0, 42 * FPS], [0, -60]);
  return (
    <AbsoluteFill style={{ background: `radial-gradient(120% 80% at 78% 12%, ${T.signalDim}, transparent 55%), linear-gradient(160deg, ${T.ink}, ${T.deep})` }}>
      <AbsoluteFill style={{ backgroundImage: `linear-gradient(rgba(49,216,199,.05) 1px, transparent 1px), linear-gradient(90deg, rgba(49,216,199,.05) 1px, transparent 1px)`, backgroundSize: "60px 60px", transform: `translateY(${drift}px)`, maskImage: "linear-gradient(180deg, transparent, black 25%, black 80%, transparent)" }} />
    </AbsoluteFill>
  );
};

const Dot: React.FC = () => <div style={{ width: 14, height: 14, borderRadius: 99, background: T.signal, boxShadow: `0 0 22px ${T.signal}` }} />;

const Mark: React.FC<{ size?: number }> = ({ size = 54 }) => (
  <svg width={size} height={size} viewBox="0 0 48 48" fill="none">
    <path d="M6 30 L18 30 L24 12 L30 36 L36 24 L42 24" stroke={T.signal} strokeWidth={3} strokeLinecap="round" strokeLinejoin="round" />
    <circle cx="24" cy="24" r="21" stroke={T.signal} strokeOpacity={0.35} strokeWidth={2} />
  </svg>
);

// Bottom caption block used in every scene.
const Caption: React.FC<{ index: string; title: string; body: string; startAt?: number }> = ({ index, title, body, startAt = 0 }) => {
  const f = useCurrentFrame() - startAt;
  const { fps } = useVideoConfig();
  const rise = spring({ frame: f, fps, config: { damping: 200 } });
  const y = interpolate(rise, [0, 1], [40, 0]);
  return (
    <div style={{ position: "absolute", left: 70, right: 70, bottom: 210, opacity: rise, transform: `translateY(${y}px)` }}>
      <div style={{ display: "flex", alignItems: "center", gap: 12, color: T.signal, font: `700 22px ${T.mono}`, letterSpacing: 3, marginBottom: 22 }}><Dot /> {index}</div>
      <div style={{ font: `800 76px ${T.sans}`, color: T.fog, letterSpacing: -3, lineHeight: 1.02 }}>{title}</div>
      <div style={{ marginTop: 22, font: `500 34px ${T.sans}`, color: "#b7c7c8", lineHeight: 1.45, maxWidth: 860 }}>{body}</div>
    </div>
  );
};

// A recreated app window (device frame) that scenes render their UI into.
const Screen: React.FC<{ children: React.ReactNode; startAt?: number }> = ({ children, startAt = 0 }) => {
  const f = useCurrentFrame() - startAt;
  const { fps } = useVideoConfig();
  const s = spring({ frame: f, fps, config: { damping: 200 } });
  const scale = interpolate(s, [0, 1], [0.92, 1]);
  return (
    <div style={{ position: "absolute", top: 150, left: 70, right: 70, height: 1150, borderRadius: 22, overflow: "hidden", border: `1px solid ${T.line}`, background: "#07131e", boxShadow: `26px 26px 0 rgba(49,216,199,.07)`, opacity: s, transform: `scale(${scale})`, transformOrigin: "top center" }}>
      <div style={{ height: 74, borderBottom: `1px solid ${T.line}`, display: "flex", alignItems: "center", gap: 12, padding: "0 26px", color: T.fog, font: `800 22px ${T.sans}` }}>
        <Mark size={30} /> TurkmenAI <span style={{ color: T.signal, fontWeight: 500 }}>Local</span>
        <div style={{ marginLeft: "auto", display: "flex", gap: 5 }}>{["EN", "RU", "TK"].map((l, i) => <div key={l} style={{ font: `700 16px ${T.mono}`, padding: "7px 9px", background: i === 0 ? T.signal : "transparent", color: i === 0 ? "#042129" : "#9eb0b3" }}>{l}</div>)}</div>
      </div>
      <div style={{ padding: 30 }}>{children}</div>
    </div>
  );
};

const Cursor: React.FC<{ from: [number, number]; to: [number, number]; clickAt: number; startAt?: number }> = ({ from, to, clickAt, startAt = 0 }) => {
  const f = useCurrentFrame() - startAt;
  const p = interpolate(f, [0, clickAt - 6], [0, 1], { extrapolateLeft: "clamp", extrapolateRight: "clamp", easing: Easing.inOut(Easing.cubic) });
  const x = interpolate(p, [0, 1], [from[0], to[0]]);
  const y = interpolate(p, [0, 1], [from[1], to[1]]);
  const press = f >= clickAt && f < clickAt + 8 ? 0.82 : 1;
  return (
    <div style={{ position: "absolute", left: x, top: y, transform: `scale(${press})`, transition: "none", zIndex: 30 }}>
      <svg width="46" height="46" viewBox="0 0 24 24" fill="none"><path d="M5 3l14 8-6 1.5L10 20 5 3z" fill={T.fog} stroke="#04222a" strokeWidth="1.2" /></svg>
      {f >= clickAt && f < clickAt + 14 && <div style={{ position: "absolute", left: -14, top: -14, width: 60, height: 60, borderRadius: 99, border: `3px solid ${T.signal}`, opacity: interpolate(f, [clickAt, clickAt + 14], [0.9, 0]), transform: `scale(${interpolate(f, [clickAt, clickAt + 14], [0.4, 1.5])})` }} />}
    </div>
  );
};

// ---------- scenes ----------------------------------------------------------

const Intro: React.FC<{ lang: Lang }> = ({ lang }) => {
  const f = useCurrentFrame();
  const { fps } = useVideoConfig();
  const s = spring({ frame: f, fps, config: { damping: 200 } });
  const line = interpolate(f, [10, 45], [0, 1], { extrapolateRight: "clamp" });
  return (
    <AbsoluteFill style={{ justifyContent: "center", alignItems: "center", padding: 80 }}>
      <div style={{ transform: `scale(${interpolate(s, [0, 1], [0.6, 1])})`, opacity: s }}><Mark size={140} /></div>
      <div style={{ marginTop: 40, font: `800 92px ${T.sans}`, color: T.fog, letterSpacing: -4, textAlign: "center", lineHeight: 1 }}>TurkmenAI <span style={{ color: T.signal }}>Local</span></div>
      <div style={{ height: 2, width: interpolate(line, [0, 1], [0, 420]), background: T.signal, margin: "38px 0", boxShadow: `0 0 20px ${T.signal}` }} />
      <div style={{ font: `500 40px ${T.sans}`, color: "#c3d2d3", textAlign: "center", maxWidth: 820, opacity: interpolate(f, [30, 55], [0, 1], { extrapolateRight: "clamp" }) }}>{tri("tagline", lang)}</div>
      <div style={{ marginTop: 26, font: `700 22px ${T.mono}`, color: T.signal, letterSpacing: 6, opacity: interpolate(f, [40, 60], [0, 1], { extrapolateRight: "clamp" }) }}>{tri("kicker", lang)}</div>
    </AbsoluteFill>
  );
};

const Card: React.FC<{ children: React.ReactNode; delay: number; style?: React.CSSProperties }> = ({ children, delay, style }) => {
  const f = useCurrentFrame();
  const { fps } = useVideoConfig();
  const s = spring({ frame: f - delay, fps, config: { damping: 200 } });
  return <div style={{ border: `1px solid ${T.line}`, background: T.panel2, padding: 24, opacity: s, transform: `translateY(${interpolate(s, [0, 1], [26, 0])}px)`, ...style }}>{children}</div>;
};

const label = { font: `700 15px ${T.mono}`, letterSpacing: 2, color: "#81969a" } as React.CSSProperties;

const SceneHardware: React.FC<{ lang: Lang }> = ({ lang }) => (
  <>
    <Screen>
      <div style={{ font: `800 40px ${T.sans}`, color: T.fog, letterSpacing: -1.5, marginBottom: 24 }}>{tri("s1title", lang)}</div>
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
        {[["CPU", "your processor"], ["RAM", "detected locally"], ["GPU / VRAM", "if available"], ["DISK", "free space"]].map(([k, v], i) => (
          <Card key={k} delay={10 + i * 7} style={{ minHeight: 150 }}>
            <div style={label}>{k}</div>
            <div style={{ marginTop: 34, font: `700 30px ${T.sans}`, color: T.fog }}>●●●</div>
            <div style={{ marginTop: 8, font: `500 20px ${T.sans}`, color: "#9baeb2" }}>{v}</div>
          </Card>
        ))}
      </div>
    </Screen>
    <Caption index="01" title={tri("s1title", lang)} body={tri("s1body", lang)} />
  </>
);

const fitColors: Record<string, string> = { EXCELLENT: T.signal, GOOD: "#7fd1a0", USABLE: T.sand };
const SceneCatalog: React.FC<{ lang: Lang }> = ({ lang }) => (
  <>
    <Screen>
      <div style={{ display: "flex", alignItems: "center", gap: 12, marginBottom: 22 }}>
        <div style={{ font: `800 40px ${T.sans}`, color: T.fog, letterSpacing: -1.5 }}>{tri("s2title", lang)}</div>
        <div style={{ marginLeft: "auto", font: `700 15px ${T.mono}`, color: "#052228", background: T.signal, padding: "8px 11px", letterSpacing: 1 }}>LIVE · HF</div>
      </div>
      {[["Qwen2.5 3B Instruct", "chat", "EXCELLENT"], ["Whisper Small", "speech", "GOOD"], ["Gemma 2 2B", "chat", "EXCELLENT"], ["NLLB 600M", "translation", "USABLE"]].map(([name, cat, fit], i) => (
        <Card key={name as string} delay={8 + i * 8} style={{ marginBottom: 12 }}>
          <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
            <div style={{ ...label, textTransform: "uppercase" }}>{cat}</div>
            <div style={{ marginLeft: "auto", font: `700 14px ${T.mono}`, color: fitColors[fit as string], border: `1px solid ${fitColors[fit as string]}`, padding: "5px 8px" }}>{fit}</div>
          </div>
          <div style={{ marginTop: 12, font: `700 30px ${T.sans}`, color: T.fog }}>{name}</div>
        </Card>
      ))}
    </Screen>
    <Caption index="02" title={tri("s2title", lang)} body={tri("s2body", lang)} />
  </>
);

const SceneInstall: React.FC<{ lang: Lang }> = ({ lang }) => {
  const f = useCurrentFrame();
  // Progress with a deliberate stall + resume around 55-70f to tell the resilience story.
  const raw = interpolate(f, [30, 55, 70, 120], [0, 42, 42, 100], { extrapolateRight: "clamp", easing: Easing.inOut(Easing.quad) });
  const pct = Math.round(raw);
  const reconnecting = f > 55 && f < 72;
  const done = pct >= 100;
  return (
    <>
      <Screen>
        <div style={{ font: `800 40px ${T.sans}`, color: T.fog, letterSpacing: -1.5, marginBottom: 22 }}>{tri("s2title", lang)}</div>
        <Card delay={4}>
          <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
            <div style={{ ...label, textTransform: "uppercase" }}>chat</div>
            <div style={{ marginLeft: "auto", font: `700 14px ${T.mono}`, color: T.signal, border: `1px solid ${T.signal}`, padding: "5px 8px" }}>EXCELLENT</div>
          </div>
          <div style={{ marginTop: 12, font: `700 32px ${T.sans}`, color: T.fog }}>Qwen2.5 3B Instruct</div>
          <div style={{ marginTop: 20, height: 12, background: "#12333a", borderRadius: 6, overflow: "hidden" }}>
            <div style={{ height: "100%", width: `${pct}%`, background: `linear-gradient(90deg, #00c2b8, ${T.signal})` }} />
          </div>
          <div style={{ marginTop: 14, display: "flex", gap: 20, font: `700 22px ${T.mono}`, color: "#8fb3b3" }}>
            <span>{done ? tri("ready", lang) : `${pct}%`}</span>
            {reconnecting ? <span style={{ color: "#e0a94a" }}>↻ {tri("reconnect", lang)}</span> : <span>{done ? tri("verified", lang) : "resumable · journaled"}</span>}
          </div>
          <button style={{ marginTop: 22, border: 0, background: done ? "#0c2a30" : T.signal, color: done ? T.signal : "#06232a", font: `800 24px ${T.sans}`, padding: "16px 22px", display: "flex", alignItems: "center", gap: 10 }}>
            {done ? `✓ ${tri("ready", lang)}` : f > 30 ? `${tri("installing", lang)}…` : tri("install", lang)}
          </button>
        </Card>
      </Screen>
      {f < 30 && <Cursor from={[720, 1250]} to={[300, 1090]} clickAt={26} />}
      <Caption index="03" title={done ? tri("s4title", lang) : tri("s3title", lang)} body={done ? tri("s4body", lang) : tri("s3body", lang)} />
    </>
  );
};

const SceneprivacyPill: React.FC<{ text: string; delay: number }> = ({ text, delay }) => {
  const f = useCurrentFrame();
  const { fps } = useVideoConfig();
  const s = spring({ frame: f - delay, fps, config: { damping: 200 } });
  return <div style={{ border: `1px solid ${T.signal}`, color: T.signal, font: `700 26px ${T.mono}`, letterSpacing: 2, padding: "16px 22px", opacity: s, transform: `scale(${interpolate(s, [0, 1], [0.8, 1])})` }}>{text}</div>;
};

const ScenePrivacy: React.FC<{ lang: Lang }> = ({ lang }) => (
  <>
    <AbsoluteFill style={{ justifyContent: "center", alignItems: "center", gap: 26, paddingBottom: 520 }}>
      <SceneprivacyPill text={tri("localApi", lang)} delay={6} />
      <SceneprivacyPill text={tri("noTelemetry", lang)} delay={16} />
      <SceneprivacyPill text={tri("offline", lang)} delay={26} />
    </AbsoluteFill>
    <Caption index="04" title={tri("s5title", lang)} body={tri("s5body", lang)} />
  </>
);

const SceneCTA: React.FC<{ lang: Lang }> = ({ lang }) => {
  const f = useCurrentFrame();
  const { fps } = useVideoConfig();
  const s = spring({ frame: f, fps, config: { damping: 200 } });
  const pulse = 1 + 0.03 * Math.sin(f / 6);
  return (
    <AbsoluteFill style={{ justifyContent: "center", alignItems: "center", padding: 80 }}>
      <div style={{ opacity: s, transform: `scale(${interpolate(s, [0, 1], [0.7, 1])})` }}><Mark size={120} /></div>
      <div style={{ marginTop: 30, font: `800 74px ${T.sans}`, color: T.fog, letterSpacing: -3, textAlign: "center" }}>TurkmenAI <span style={{ color: T.signal }}>Local</span></div>
      <div style={{ marginTop: 40, background: T.signal, color: "#04222a", font: `800 42px ${T.sans}`, padding: "26px 44px", transform: `scale(${pulse})` }}>{tri("cta", lang)}</div>
      <div style={{ marginTop: 30, font: `700 40px ${T.mono}`, color: T.signal, letterSpacing: 2 }}>{tri("ctaUrl", lang)}</div>
      <div style={{ marginTop: 18, font: `700 20px ${T.mono}`, color: T.muted, letterSpacing: 3 }}>v0.3.0 · WINDOWS · macOS · LINUX</div>
    </AbsoluteFill>
  );
};

// ---------- timeline --------------------------------------------------------

export const Tour: React.FC<{ lang: Lang }> = ({ lang }) => {
  const s = FPS;
  return (
    <AbsoluteFill style={{ fontFamily: T.sans, background: T.ink }}>
      <Bg />
      <Sequence durationInFrames={4 * s}><Intro lang={lang} /></Sequence>
      <Sequence from={4 * s} durationInFrames={6 * s}><SceneHardware lang={lang} /></Sequence>
      <Sequence from={10 * s} durationInFrames={7 * s}><SceneCatalog lang={lang} /></Sequence>
      <Sequence from={17 * s} durationInFrames={9 * s}><SceneInstall lang={lang} /></Sequence>
      <Sequence from={26 * s} durationInFrames={8 * s}><ScenePrivacy lang={lang} /></Sequence>
      <Sequence from={34 * s} durationInFrames={8 * s}><SceneCTA lang={lang} /></Sequence>
      {/* progress rail */}
      <ProgressRail />
    </AbsoluteFill>
  );
};

const ProgressRail: React.FC = () => {
  const f = useCurrentFrame();
  const { durationInFrames } = useVideoConfig();
  const w = interpolate(f, [0, durationInFrames], [0, 100]);
  return <div style={{ position: "absolute", left: 0, right: 0, top: 0, height: 6, background: "rgba(255,255,255,.06)" }}><div style={{ height: "100%", width: `${w}%`, background: T.signal }} /></div>;
};

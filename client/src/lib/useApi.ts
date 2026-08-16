import { useEffect, useState } from "react";
import type { Language } from "@/i18n";

/**
 * Fetch a generated /api/*.json endpoint. The website consumes the SAME
 * machine-readable metadata that CI publishes, so pages never drift from the
 * repository's source of truth.
 */
export function useApi<T>(path: string): { data: T | null; loading: boolean; error: string | null } {
  const [state, setState] = useState<{ data: T | null; loading: boolean; error: string | null }>({ data: null, loading: true, error: null });
  useEffect(() => {
    let alive = true;
    setState({ data: null, loading: true, error: null });
    fetch(path, { headers: { accept: "application/json" } })
      .then((r) => { if (!r.ok) throw new Error(`${r.status}`); return r.json(); })
      .then((data) => { if (alive) setState({ data, loading: false, error: null }); })
      .catch((e) => { if (alive) setState({ data: null, loading: false, error: String(e?.message ?? e) }); });
    return () => { alive = false; };
  }, [path]);
  return state;
}

/** Language persisted across the catalog pages (shared with the homepage switch). */
export function usePersistedLanguage(): [Language, (value: Language) => void] {
  const [language, setLanguage] = useState<Language>(() => {
    if (typeof localStorage === "undefined") return "en";
    const saved = localStorage.getItem("lang");
    return saved === "ru" || saved === "tk" || saved === "en" ? saved : "en";
  });
  const set = (value: Language) => { setLanguage(value); try { localStorage.setItem("lang", value); } catch { /* ignore */ } };
  return [language, set];
}

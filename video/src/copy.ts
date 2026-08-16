export type Lang = "en" | "ru" | "tk";
type Tri = Record<Lang, string>;

// Punchy, single-focus copy: a short HEAD (1-3 words) + a one-line CAPTION per
// beat. No paragraphs competing with the UI. Honest capabilities, no numbers.
export const C: Record<string, Tri> = {
  tagline: { en: "Local AI. Built for Turkmenistan.", ru: "Локальный ИИ. Создан для Туркменистана.", tk: "Ýerli AI. Türkmenistan üçin." },
  kicker: { en: "OFFLINE-FIRST · PRIVATE · FREE", ru: "ОФЛАЙН · ПРИВАТНО · БЕСПЛАТНО", tk: "OFLAÝN · HUSUSY · MUGT" },

  h_hw: { en: "Knows your PC", ru: "Знает ваш ПК", tk: "Kompýuteriňizi bilýär" },
  c_hw: { en: "RAM · GPU · disk — read locally", ru: "RAM · GPU · диск — локально", tk: "RAM · GPU · disk — ýerli" },

  h_models: { en: "Models from Hugging Face", ru: "Модели с Hugging Face", tk: "Hugging Face modelleri" },
  c_models: { en: "Ranked for your hardware", ru: "Под ваше железо", tk: "Enjamyňyza görä saýlanýar" },

  h_data: { en: "Datasets too", ru: "И датасеты", tk: "Data-lar hem" },
  c_data: { en: "By task, with license flags", ru: "По задачам, с лицензиями", tk: "Wezipe boýunça, ygtyýarnamaly" },

  h_install: { en: "One click installs", ru: "Установка в один клик", tk: "Bir gezekde gurulýar" },
  c_install: { en: "The engine sets itself up", ru: "Движок ставится сам", tk: "Hereketlendiriji özi gurulýar" },

  h_resume: { en: "Survives bad Internet", ru: "Переживёт обрыв связи", tk: "Internet üzülse-de işleýär" },
  c_resume: { en: "Resumes — never from zero", ru: "Докачивает — не с нуля", tk: "Dowam edýär — noldan däl" },

  h_private: { en: "Private by design", ru: "Приватно по умолчанию", tk: "Başdan hususy" },
  c_private: { en: "127.0.0.1 · no telemetry", ru: "127.0.0.1 · без телеметрии", tk: "127.0.0.1 · telemetriýasyz" },

  h_offline: { en: "Works offline", ru: "Работает офлайн", tk: "Oflaýn işleýär" },
  c_offline: { en: "No cloud. No Internet needed.", ru: "Без облака и интернета", tk: "Bulutsyz, internetsiz" },

  h_bench: { en: "Benchmarks built in", ru: "Бенчмарки внутри", tk: "Bençmarklar içinde" },
  c_bench: { en: "Speed & tokens/sec, on device", ru: "Скорость и tokens/sec", tk: "Tizlik we token/sek" },

  h_lang: { en: "Three languages", ru: "Три языка", tk: "Üç dil" },
  c_lang: { en: "RU · TK · EN everywhere", ru: "RU · TK · EN везде", tk: "RU · TK · EN hemme ýerde" },

  cta: { en: "Download free", ru: "Скачать бесплатно", tk: "Mugt ýükle" },

  // tiny in-UI labels
  install: { en: "Install", ru: "Установить", tk: "Gurmak" },
  reconnect: { en: "reconnecting — resumed", ru: "переподключение — продолжено", tk: "birikme — dowam" },
  ready: { en: "ready", ru: "готово", tk: "taýýar" },
};

export const tri = (k: string, lang: Lang) => C[k]?.[lang] ?? C[k]?.en ?? k;

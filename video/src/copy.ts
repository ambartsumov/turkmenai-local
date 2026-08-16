export type Lang = "en" | "ru" | "tk";
type Tri = Record<Lang, string>;

// Honest product-tour copy — capabilities only, no benchmark numbers.
export const C: Record<string, Tri> = {
  tagline: { en: "Local AI. Built for Turkmenistan.", ru: "Локальный ИИ. Создан для Туркменистана.", tk: "Ýerli AI. Türkmenistan üçin döredildi." },
  kicker: { en: "OFFLINE-FIRST DESKTOP", ru: "ОФЛАЙН-ПРИЛОЖЕНИЕ", tk: "OFLAÝN PROGRAMMA" },

  s1title: { en: "Knows your PC", ru: "Знает ваш ПК", tk: "Kompýuteriňizi bilýär" },
  s1body: { en: "Reads RAM, GPU and disk locally. Nothing leaves your machine.", ru: "Считывает RAM, GPU и диск локально. Ничего не уходит с устройства.", tk: "RAM, GPU we diski ýerli okaýar. Hiç zat çykmaýar." },

  s2title: { en: "Models from Hugging Face", ru: "Модели с Hugging Face", tk: "Hugging Face-den modeller" },
  s2body: { en: "Ranked for your hardware, by specialization, in three languages.", ru: "Под ваше железо, по специализации, на трёх языках.", tk: "Enjamyňyza görä, ugur boýunça, üç dilde." },

  s3title: { en: "One click. It installs.", ru: "Один клик. Установка.", tk: "Bir gezek bas. Gurulýar." },
  s3body: { en: "You pick the model — download, verify and setup are automatic.", ru: "Вы выбираете модель — загрузка, проверка и настройка сами.", tk: "Modeli saýlaýarsyňyz — ýükleme, barlag we gurnama awtomat." },

  s4title: { en: "Built for slow Internet", ru: "Для медленного интернета", tk: "Haýal internet üçin" },
  s4body: { en: "Connection drops? It resumes — never restarts from zero.", ru: "Обрыв связи? Докачивает — не начинает с нуля.", tk: "Baglanyşyk üzüldimi? Dowam edýär — noldan başlanok." },

  s5title: { en: "Private & offline", ru: "Приватно и офлайн", tk: "Hususy we oflaýn" },
  s5body: { en: "Runs on 127.0.0.1. No telemetry. No cloud. Works with no Internet.", ru: "Работает на 127.0.0.1. Без телеметрии. Без облака. Без интернета.", tk: "127.0.0.1-de işleýär. Telemetriýasyz. Bulutsyz. Internetsiz." },

  s6title: { en: "See it in the console", ru: "Всё видно в консоли", tk: "Konsolda görünýär" },
  s6body: { en: "Real download speed, resume, and on-device benchmarks.", ru: "Реальная скорость, докачка и бенчмарки на устройстве.", tk: "Hakyky tizlik, dowam etdirme we enjamda bençmarklar." },

  cta: { en: "Download free", ru: "Скачать бесплатно", tk: "Mugt ýükle" },
  ctaUrl: { en: "turkmenai.tech", ru: "turkmenai.tech", tk: "turkmenai.tech" },

  // small UI labels used inside recreated screens
  install: { en: "Install", ru: "Установить", tk: "Gurmak" },
  installing: { en: "Installing", ru: "Установка", tk: "Gurulýar" },
  reconnect: { en: "reconnecting — resumed", ru: "переподключение — продолжено", tk: "täzeden birikme — dowam" },
  verified: { en: "SHA-256 verified", ru: "SHA-256 проверено", tk: "SHA-256 barlandy" },
  ready: { en: "ready", ru: "готово", tk: "taýýar" },
  offline: { en: "OFFLINE OK", ru: "ОФЛАЙН OK", tk: "OFLAÝN OK" },
  noTelemetry: { en: "NO TELEMETRY", ru: "БЕЗ ТЕЛЕМЕТРИИ", tk: "TELEMETRIÝASYZ" },
  localApi: { en: "127.0.0.1 ONLY", ru: "ТОЛЬКО 127.0.0.1", tk: "DIŇE 127.0.0.1" },
};

export const tri = (k: string, lang: Lang) => C[k]?.[lang] ?? C[k]?.en ?? k;

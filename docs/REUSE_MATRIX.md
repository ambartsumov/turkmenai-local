# Матрица повторного использования

| Функция | Существующий проект | Повторное использование | Метод интеграции | Оригинальный код TurkmenAI | Лицензия / граница | Риск |
|---|---|---:|---|---|---|---|
| Desktop shell и базовый chat UX | Jan | 70% | Отдельная pinned upstream-база и минимальные patches | Навигация TurkmenAI, i18n, панели состояния | Apache-2.0 на уровне репозитория; аудит файлов обязателен | Средний |
| GGUF LLM/VLM | llama.cpp | 85% | Управляемый дочерний процесс и OpenAI-compatible localhost API | План запуска, выбор backend, health/status | MIT; не копировать runtime-код | Низкий |
| Скачивание моделей | `huggingface_hub` + HTTP range | 75% | Манифестный downloader; выборочные файлы и cache | Очередь, poor-network policy, recovery state | Apache-2.0 для Hub; пользовательские лицензии моделей отдельно | Средний |
| Хранилище моделей | Hugging Face cache concepts | 35% | Файловые blobs по SHA-256 | Манифесты, refs, derived-artifact keys, export | Оригинальный код TurkmenAI | Средний |
| ASR | whisper.cpp | 90% | Отдельный локальный процесс/CLI или HTTP wrapper | Адаптер вызова, языковой выбор, job state | MIT | Низкий |
| Transformer ASR/LLM | CTranslate2 | 70% | Изолированный runtime, только совместимые конвертированные артефакты | Определение совместимости, планирование | MIT | Средний |
| GPTQ/AWQ | GPTQModel | 60% | Изолированный Python runtime, явная команда пользователя | План конверсии и safety checks | Apache-2.0; модели отдельно | Высокий |
| ONNX | ONNX Runtime | 75% | Версионированный runtime package | Capability registry и план запуска | MIT | Средний |
| Apple Silicon | MLX / Jan MLX extension | 70% | macOS-only process/plugin backend | Планирование/профиль, не универсальный fallback | Отдельный аудит каждой версии | Средний |
| Image/video generation | ComfyUI | 85% | Только process-level local HTTP integration | Installer manifest, workflow UX, permission gate | GPL-3.0; без линковки и встраивания кода | Высокий |
| TTS | Поддерживаемый Piper/F5-TTS backend | 55% | Изолированный runtime с единым `synthesize` контрактом | Voice registry и запуск | Перепроверяется на конкретном релизе | Высокий |
| LAN discovery | mDNS library | 50% | Опциональный feature flag и аутентифицированный local API | Trust store, hash verification, UI consent | Зависит от библиотеки | Средний |
| Приложения/recipes | Pinokio concepts | 25% | Собственный schema-first manifest без выполнения внешних shell scripts | Permission model и audited installers | Не копировать непроверенные рецепты | Высокий |

**Принцип:** внешние runtime не исполняют произвольный код модели автоматически. Каждое действие проходит модельный resolver, scanner, execution plan и явное подтверждение, если нужен custom code.

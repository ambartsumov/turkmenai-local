# Third-Party Notices

TurkmenAI Local `0.1.0` сохраняет этот реестр для исходного кода, бинарных runtime и process-level интеграций. Запись в реестре не означает, что компонент встраивается в текущий бинарник; точный состав распространённого артефакта фиксируется release-процедурой.

| Компонент | Репозиторий | Зафиксированная версия / политика | Лицензия | Метод | Обязательство |
|---|---|---|---|---|---|
| Jan | https://github.com/janhq/jan | `7ccd3c6d32ac7ac97a58d55a94000cb7c883dc6a` | Apache-2.0; отдельный Rust crate заявляет MIT | Pinned upstream / будущие patches | Сохранить LICENSE, notices и аудит файлов |
| llama.cpp | https://github.com/ggml-org/llama.cpp | Pin в runtime manifest до выпуска | MIT | Process/API | Включить текст MIT при поставке бинарника |
| whisper.cpp | https://github.com/ggml-org/whisper.cpp | Pin в runtime manifest до выпуска | MIT | Process/API | Включить текст MIT при поставке бинарника |
| CTranslate2 | https://github.com/OpenNMT/CTranslate2 | Pin в isolated runtime manifest | MIT | Isolated runtime | Включить текст MIT при поставке |
| GPTQModel | https://github.com/ModelCloud/GPTQModel | Pin в isolated runtime manifest | Apache-2.0 | Optional isolated runtime | Сохранить NOTICE/LICENSE |
| Hugging Face Hub | https://github.com/huggingface/huggingface_hub | Pin в environment lock | Apache-2.0 | Download metadata/cache protocol | Не переносит лицензию моделей; показывать model card license |
| ComfyUI | https://github.com/comfy-org/ComfyUI | Отдельная явная установка пользователя | GPL-3.0 | Process-only local HTTP | Не связывать и не копировать код; отдельно включить лицензию при распространении |

Модельные веса, голоса, workflow и пользовательские плагины не получают статус «проверенных» автоматически. Их лицензия, revision, checksum и source сохраняются на уровне `ModelDescriptor`/manifest.

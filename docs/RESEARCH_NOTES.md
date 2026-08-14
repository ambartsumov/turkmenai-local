# Исследовательские заметки

## Jan: снимок первичной проверки

Проверка официального репозитория `janhq/jan` 14 августа 2026 года показала активный проект на ветке `main` с историей более 8 000 коммитов. В дереве присутствуют `src-tauri`, `web-app`, `core`, `extensions`, `mlx-server` и `tests`; это подтверждает наличие десктопной Tauri-обвязки, фронтенда, расширений и отдельных runtime-слоёв.[1]

Официальное описание Jan заявляет локальный запуск LLM, поиск и загрузку моделей с Hugging Face, OpenAI-совместимый сервер на `localhost:1337`, MCP-интеграцию и пакеты для Windows, macOS и Linux, включая AppImage.[1]

Последние публичные изменения также показывают зрелые интеграционные практики: проверку готовности `llama.cpp` backend, health-check после обновления runtime, сохранение предыдущих версий для отката и стартовый мастер установки.[1]

## Open WebUI: снимок сравнительной проверки

Open WebUI позиционируется как расширяемая самостоятельная веб-платформа для работы офлайн, поддерживающая Ollama и OpenAI-совместимые API. В текущем репозитории архитектура видимо ориентирована на веб-службу с отдельными каталогами `backend`, `src`, `static`, `docs` и `test`, а не на нативную десктопную оболочку.[2]

Этот проект целесообразно рассматривать как совместимый опциональный интерфейс для сетевого режима TurkmenAI Local, но не как основу клиентского desktop-продукта: для задачи требуются встроенные установка runtime, аппаратная диагностика и автономная упаковка в `.exe` и `.AppImage`.[1] [2]

## Ключевые runtime: снимок сравнительной проверки

`llama.cpp` представляет собой активный C/C++ runtime для локального LLM/VLM-inference с минимальной установкой и широким спектром аппаратных backend; в официальном репозитории указан MIT-лицензионный режим.[3] Его следует использовать как основной процессный backend для GGUF, CPU/GPU-гибридного запуска и OpenAI-совместимого сервера.

ComfyUI позиционируется как модульный графовый GUI, API и backend для diffusion-моделей. На момент проверки в официальном репозитории указана GPL-3.0-лицензия.[4] Поэтому TurkmenAI Local не включает его код и не линкуется с ним: поддержка проектируется только как отдельный устанавливаемый процесс с взаимодействием через локальный HTTP API.

## Hub, ASR и трансформеры

Официальная библиотека `huggingface_hub` поддерживает версионный локальный cache, выборочные загрузки по patterns, зафиксированные revisions, оценку объёма до скачивания в dry-run и параллельную загрузку snapshot. TurkmenAI Local использует эти возможности как источник метаданных и cache-политику, дополняя их собственным персистентным состоянием задач, SHA-256 blob-store и контролем сетевого профиля.[5]

Для GGUF Hugging Face документирует прямой запуск через `llama.cpp` с `llama-server` и OpenAI-совместимым endpoint. Это подтверждает выбор процесса `llama-server` как узкого, проверяемого integration boundary.[6]

`whisper.cpp` остаётся пригодной основой ASR: он реализован на C/C++, поддерживает CPU и несколько GPU-ускорителей, квантование, VAD и основные desktop ОС.[7] Для совместимых transformer-моделей CTranslate2 предоставляет оптимизированное CPU/GPU исполнение, reduced precision и конвертеры форматов; он должен устанавливаться изолированно и только по выбранному execution plan.[8]

## Возможности, подлежащие negotiation

Текущая документация `llama.cpp` подтверждает несколько методов speculative decoding: draft-модель, EAGLE-подобные режимы и варианты n-gram. Поэтому в TurkmenAI эта функция представляется только как capability конкретной версии runtime и конкретного плана, а не как гарантия для каждой модели. Включение возможно лишь после короткого измерения в установленном пользователем бюджете.[9]

`llama.cpp` также поддерживает GBNF для ограничения вывода формальной грамматикой, включая valid JSON; документация предупреждает, что объектные JSON schema по умолчанию не допускают `additionalProperties`. Следовательно, API TurkmenAI нормализует только утверждённый subset structured output и честно сообщает, когда backend/plan не предоставляет этот режим.[10]

## Источники

[1]: https://github.com/janhq/jan "janhq/jan — официальный репозиторий"
[2]: https://github.com/open-webui/open-webui "open-webui/open-webui — официальный репозиторий"
[3]: https://github.com/ggml-org/llama.cpp "ggml-org/llama.cpp — официальный репозиторий"
[4]: https://github.com/comfy-org/comfyui "Comfy-Org/ComfyUI — официальный репозиторий"
[5]: https://huggingface.co/docs/huggingface_hub/en/guides/download "Hugging Face Hub — Download files"
[6]: https://huggingface.co/docs/hub/en/gguf-llamacpp "Hugging Face Hub — GGUF usage with llama.cpp"
[7]: https://github.com/ggml-org/whisper.cpp "ggml-org/whisper.cpp — официальный репозиторий"
[8]: https://github.com/OpenNMT/CTranslate2 "OpenNMT/CTranslate2 — официальный репозиторий"
[9]: https://github.com/ggml-org/llama.cpp/blob/master/docs/speculative.md "llama.cpp — Speculative Decoding"
[10]: https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md "llama.cpp — GBNF Guide"

# Контракты Core

## ModelDescriptor

`ModelDescriptor` — результат анализа, а не утверждение о работоспособности. Он хранит источник, revision, обнаруженные файлы, capability hints, зависимостный граф, модельный риск и лицензирование. Поля, полученные только из model card, маркируются как claims; поля от успешного smoke-test или benchmark получают evidence `measured`.

## BackendCapability

Каждый backend описывается отдельным versioned JSON-манифестом. Реестр не имеет фиксированного списка семейств: `architectures` может быть пустым или включать pattern/metadata matcher; поддержка определяется по формату, modality, task и доступной версии runtime.

## ExecutionPlan и RecoveryPlan

`ExecutionPlan` одновременно является пользовательским объяснением, API payload и воспроизводимым входом supervisor. `RecoveryPlan` не вносит изменений сам: он ранжирует безопасные действия, такие как снижение контекста, CPU offload или другой уже имеющийся quant, и требует подтверждения для действий, меняющих runtime или скачивающих новый artifact.

## Граф выполнения

Граф содержит typed nodes: `weights`, `tokenizer`, `config`, `processor`, `projector`, `adapter`, `asr`, `tts`, `embedding`, `reranker`, `vector_store` и `runtime`. Рёбра выражают «требует» и «подаёт выход», что позволяет использовать один механизм для text, vision, audio, RAG и app manifests без собственного engine для каждого типа.

## API

Нативный API использует `/api/v1`, а OpenAI-compatible слой — `/v1`. Поддержанный endpoint включается только когда активный plan предоставляет соответствующую capability; например `/v1/audio/transcriptions` не публикуется без готового ASR runtime.

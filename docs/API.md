# Local API

По умолчанию API слушает только `127.0.0.1:8742`. LAN не открывается и не имеет неявного fallback. Команда запуска — `tmai server`, а базовый URL — `http://127.0.0.1:8742`.

| Endpoint | Назначение | Статус в `0.1.0` |
|---|---|---|
| `GET /api/v1/health` | Privacy-first состояние локальной службы | Реализован |
| `GET /api/v1/hardware` | Обнаруженный локальный hardware profile | Реализован |
| `GET /api/v1/capabilities` | Backend capability registry | Реализован |
| `POST /api/v1/analyze` | Безопасно анализирует источник без исполнения кода | Реализован |
| `POST /api/v1/plan` | Возвращает ranked execution plans | Реализован |
| `GET /v1/models` | OpenAI-compatible список READY моделей | Реализован; вернёт пустой массив до установки |
| `POST /v1/chat/completions` | Inference через READY runtime | Явно вернёт `NO_ACTIVE_RUNTIME`, пока supervisor/runtime ещё не подключён |

```bash
curl http://127.0.0.1:8742/api/v1/health
curl -X POST http://127.0.0.1:8742/api/v1/plan \
  -H 'content-type: application/json' \
  -d '{"source":"owner/model","objective":"balanced"}'
```

Этот контракт преднамеренно не имитирует ответы модели. Inference endpoint станет доступным только после появления проверенного RuntimeSupervisor и smoke-tested model artifact.

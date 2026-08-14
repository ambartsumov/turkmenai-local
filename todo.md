# Рабочий список TurkmenAI Local

## Дополнение: интеллектуальное выполнение и адаптация

- [ ] Добавить `ModelExecutionGraph` с узлами tokenizer, config, weights, processors, adapters, vision/audio-компонентов и runtime-зависимостей.
- [ ] Добавить минимальную оркестрацию `UniversalAIPipeline` с явными входами, узлами, backend и выходами.
- [ ] Расширить `ExecutionPlan` ранжированными безопасными fallback-стратегиями и `RecoveryPlan` без удаления исходных файлов.
- [ ] Реализовать `ModelDoctor`: проверка целостности, графа, runtime, backend, оборудования и плана запуска.
- [ ] Добавить локальную базу benchmark-результатов с разграничением оценок и измеренных данных.
- [ ] Учитывать локальную историю benchmark и успешные планы в объяснимых рекомендациях.
- [ ] Реализовать семействозависимый анализ метаданных без хрупкого фиксированного списка моделей.
- [ ] Расширить planner стратегией поиска предквантованных вариантов перед исходными BF16/FP16-весами.
- [ ] Оформить конфигурационную поддержку калибровочных наборов, включая Turkmen-ориентированные наборы без загрузки пользовательских данных.

## Финальное дополнение: capability-driven платформа

- [ ] Создать `BackendCapabilityRegistry` и feature negotiation для тройного соответствия «модель + backend + hardware».
- [ ] Реализовать multi-objective scoring для планов: balanced, fastest, best-quality, lowest-RAM, lowest-VRAM и lowest-download.
- [ ] Добавить безопасные сценарии «Make it run» и «Optimize performance» с измерением только в заданном benchmark-бюджете.
- [ ] Хранить локальные hardware/model profiles, измеренные конфигурации и достоверно отделять прогнозы от измерений.
- [ ] Спроектировать scheduler локальных запросов с приоритетом interactive chat и жизненными состояниями модели.
- [ ] Добавить capability-aware контролы structured output, tool calling, reasoning, speculative decoding и context autoscaling.
- [ ] Реализовать negotiation runtime features и не показывать недоступные в текущем плане возможности.

## Production-readiness gate

- [ ] Выполнить итоговый TypeScript, Rust workspace и native desktop check после финальных изменений.
- [ ] Пересобрать Linux DEB/RPM/AppImage только из финальной ревизии и обновить SHA-256.
- [ ] Сформировать выпускную папку только с реальными артефактами, документацией, брендингом и исходным кодом без кешей зависимостей.
- [ ] Проверить содержимое выпускной папки и сохранить production-readiness report с известными ограничениями.

## Массовое внедрение

- [ ] Зафиксировать SLO, crash reporting без пользовательских данных и процедуру управления инцидентами до публичного general availability.
- [ ] Провести smoke-тесты на поддерживаемых реальных runtime и модельных артефактах для каждой целевой ОС.
- [ ] Выполнить независимый security review permission-модели, обновлений и LAN-режима перед включением этих возможностей.
- [ ] Запустить staged rollout с telemetry opt-in, обратимой схемой обновления и каналами stable/beta.

## Final production & mass-deployment directive

- [ ] Реализовать zero-config first-run wizard: язык, hardware report, сценарий использования, рекомендованная конфигурация, размер загрузки, установка, benchmark и готовность.
- [ ] Добавить транзакционный lifecycle установки модели: PREPARING, DOWNLOADING, VERIFYING, INSTALLING, CONFIGURING, TESTING, READY, FAILED и ROLLBACK.
- [ ] Реализовать пользовательские recovery-действия без raw stack trace, не блокирующие вход в приложение.
- [ ] Добавить миграции версии локального состояния и тесты сохранности model references, settings, chats, download history и execution plans.
- [ ] Настроить безопасное обновление, uninstall с явным выбором сохранения данных и portable mode без удаления model store.
- [ ] Добавить профильные downloader-тесты: throttling, restart resume, network interruption, hash corruption и recovery verified content.
- [ ] Проверить clean-machine сценарии и реальные runtime/model smoke-тесты перед объявлением general availability.

## Production hardening backlog

- [ ] Добавить стандартизированные error codes, privacy-safe rotating logs, local-only crash export и Privacy Center.
- [ ] Ввести model-provider/artifact-source абстракции, HTTP cache, backoff и offline registry cache.
- [ ] Реализовать versioned database state, миграции, backup/restore и import/export без потери model store.
- [ ] Добавить автопроверки i18n completeness, accessibility, dead links, SBOM/dependency audit и packaging E2E в CI.
- [ ] Реализовать safe/recovery mode, диагностический экспорт, update channels и проверку подписей обновлений.
- [ ] Завершить beginner UX: модельная библиотека, рекомендации, explainability, installation preview, queue и реальные readiness badges.
- [ ] Выполнить CPU/NVIDIA/AMD/Apple и clean-machine matrix, long-run/soak/performance checks и зафиксировать результаты.

## Поддержка проекта

- [ ] Добавить на публичный сайт добровольный блок поддержки с адресом BEP-20 / ERC-20, явным предупреждением о проверке сети и копированием адреса без передачи средств от имени пользователя.
- [ ] Обновить описание и темы GitHub-репозитория TurkmenAI Local после проверки существующего удалённого репозитория; профиль пользователя не изменять без отдельного явного подтверждения.
- [ ] Проверить публичные данные GitHub-профиля и README-профиля, затем подготовить профессиональное описание, focus areas и ссылки без избыточного раскрытия личных данных.
- [ ] Проверить доступность Hugging Face-профиля через настроенный коннектор и обновить публичные метаданные/README только после явной верификации целевого профиля.
- [ ] Исключить любые изменения GitHub и Hugging Face из текущего релизного прохода; продолжить только блок добровольной поддержки и поставку TurkmenAI Local.

# Release Checklist

Этот checklist выполняется для каждой публикации. Пункт считается завершённым только при наличии команды, лога и фактического артефакта; отсутствие поддерживаемой платформы не заменяется файлом с нужным расширением.

| Проверка | Команда / источник | Статус для `0.1.0` |
|---|---|---|
| TypeScript | `pnpm run check` | Выполняется локально |
| Website build | `pnpm run build` | Выполняется локально |
| Core/API/unit tests | `cargo test --workspace` | Выполняется локально |
| Native Linux compile | `cargo check -p turkmenai-desktop` | Выполняется локально |
| Linux desktop bundle | `pnpm desktop:build` | Выполняется перед выпуском |
| Windows installer | GitHub Actions `release.yml` | Настроено, не строится в Linux sandbox |
| macOS DMG | GitHub Actions `release.yml` | Настроено, не строится в Linux sandbox |
| Third-party notices | `THIRD_PARTY_NOTICES.md` | Проверяется вручную |
| Icon family | `desktop/src-tauri/icons/` | Сгенерирована |
| SHA-256 sums | `sha256sum` над фактическими bundle | Создаются только после bundle |
| Model/runtime smoke test | Explicit local fixture + installed runtime | Не выполнен: runtime и fixture не поставляются в `0.1.0` |
| Release gate | Все обязательные строки выше | Не ставить stable-tag при незавершённых проверках |

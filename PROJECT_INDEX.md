# Índice del proyecto — warp-no-ai

Terminal Warp en Rust, fork sin AI/cloud opcional. Workspace Cargo con 65+ crates, binario principal en `app/`, framework UI propio en `crates/warpui/`.

## Raíz

- [WARP.md](WARP.md) — guía dev, comandos, arquitectura
- [README.md](README.md), [CONTRIBUTING.md](CONTRIBUTING.md), [FAQ.md](FAQ.md), [SECURITY.md](SECURITY.md), [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- [Cargo.toml](Cargo.toml), [Cargo.lock](Cargo.lock) — workspace
- [rust-toolchain.toml](rust-toolchain.toml), [.clippy.toml](.clippy.toml), [.rustfmt.toml](.rustfmt.toml)
- [deny.toml](deny.toml), [diesel.toml](diesel.toml)
- LICENSE-AGPL + LICENSE-MIT (dual)

## Binario principal — [app/](app/)

- [app/src/lib.rs](app/src/lib.rs), [app/src/bin/](app/src/bin/) — entry points
- [app/src/app_state.rs](app/src/app_state.rs), [app/src/root_view.rs](app/src/root_view.rs) — estado raíz UI
- [app/src/terminal/](app/src/terminal/) — emulación terminal
- [app/src/ai/](app/src/ai/), [app/src/ai_assistant/](app/src/ai_assistant/) — agente AI
- [app/src/auth/](app/src/auth/) — login
- [app/src/drive/](app/src/drive/) — sync nube
- [app/src/settings/](app/src/settings/), [app/src/settings_view/](app/src/settings_view/)
- [app/src/workspace/](app/src/workspace/), [app/src/workspaces/](app/src/workspaces/)
- [app/src/notebooks/](app/src/notebooks/), [app/src/workflows/](app/src/workflows/)
- [app/src/persistence/](app/src/persistence/) — Diesel/SQLite, schema
- [app/src/platform/](app/src/platform/) — código por OS
- [app/src/features.rs](app/src/features.rs) — feature flags
- [app/src/code/](app/src/code/), [app/src/code_review/](app/src/code_review/)
- [app/src/billing/](app/src/billing/), [app/src/pricing/](app/src/pricing/)
- [app/src/voice/](app/src/voice/), [app/src/themes/](app/src/themes/)
- [app/assets/](app/assets/), [app/resources/](app/resources/), [app/tests/](app/tests/)

## Crates — [crates/](crates/)

### UI framework

- [crates/warpui/](crates/warpui/) — framework Entity-Component-Handle
- [crates/warpui_core/](crates/warpui_core/), [crates/warpui_extras/](crates/warpui_extras/)
- [crates/ui_components/](crates/ui_components/)
- [crates/editor/](crates/editor/) — edición texto

### Núcleo

- [crates/warp_core/](crates/warp_core/) — utilidades, plataforma, `FeatureFlag`
- [crates/warp_terminal/](crates/warp_terminal/) — modelo terminal
- [crates/warp_features/](crates/warp_features/), [crates/warp_util/](crates/warp_util/)
- [crates/warp_logging/](crates/warp_logging/), [crates/simple_logger/](crates/simple_logger/)
- [crates/warp_files/](crates/warp_files/), [crates/virtual_fs/](crates/virtual_fs/)

### Red / IPC

- [crates/graphql/](crates/graphql/), [crates/warp_graphql_schema/](crates/warp_graphql_schema/)
- [crates/ipc/](crates/ipc/), [crates/jsonrpc/](crates/jsonrpc/), [crates/websocket/](crates/websocket/)
- [crates/http_client/](crates/http_client/), [crates/http_server/](crates/http_server/)
- [crates/warp_server_client/](crates/warp_server_client/), [crates/remote_server/](crates/remote_server/)
- [crates/warp_web_event_bus/](crates/warp_web_event_bus/)

### AI / lenguaje

- [crates/ai/](crates/ai/) — integración LLM
- [crates/computer_use/](crates/computer_use/)
- [crates/warp_completer/](crates/warp_completer/) — autocompletado
- [crates/input_classifier/](crates/input_classifier/), [crates/natural_language_detection/](crates/natural_language_detection/)
- [crates/languages/](crates/languages/), [crates/lsp/](crates/lsp/)
- [crates/syntax_tree/](crates/syntax_tree/), [crates/markdown_parser/](crates/markdown_parser/)

### Comandos / shell

- [crates/command/](crates/command/), [crates/command-signatures-v2/](crates/command-signatures-v2/)
- [crates/vim/](crates/vim/), [crates/voice_input/](crates/voice_input/)
- [crates/onboarding/](crates/onboarding/)

### Infra

- [crates/persistence/](crates/persistence/) — DB
- [crates/settings/](crates/settings/), [crates/settings_value/](crates/settings_value/), [crates/settings_value_derive/](crates/settings_value_derive/)
- [crates/firebase/](crates/firebase/) — auth backend
- [crates/managed_secrets/](crates/managed_secrets/), [crates/managed_secrets_wasm/](crates/managed_secrets_wasm/)
- [crates/asset_cache/](crates/asset_cache/), [crates/asset_macro/](crates/asset_macro/)
- [crates/handlebars/](crates/handlebars/), [crates/fuzzy_match/](crates/fuzzy_match/), [crates/sum_tree/](crates/sum_tree/)
- [crates/repo_metadata/](crates/repo_metadata/), [crates/warp_ripgrep/](crates/warp_ripgrep/)
- [crates/watcher/](crates/watcher/), [crates/prevent_sleep/](crates/prevent_sleep/)
- [crates/node_runtime/](crates/node_runtime/), [crates/warp_js/](crates/warp_js/)
- [crates/isolation_platform/](crates/isolation_platform/), [crates/app-installation-detection/](crates/app-installation-detection/)
- [crates/channel_versions/](crates/channel_versions/), [crates/field_mask/](crates/field_mask/), [crates/string-offset/](crates/string-offset/)

### Targets / herramientas

- [crates/warp_cli/](crates/warp_cli/) — CLI standalone
- [crates/serve-wasm/](crates/serve-wasm/) — target WASM
- [crates/integration/](crates/integration/) — tests E2E

## Scripts — [script/](script/)

- [script/bootstrap](script/bootstrap), [script/run](script/run), [script/presubmit](script/presubmit)
- [script/bundle](script/bundle), `script/install_cargo_*`
- [script/macos/](script/macos/), [script/linux/](script/linux/), [script/windows/](script/windows/), [script/wasm/](script/wasm/)
- [script/Entitlements.plist](script/Entitlements.plist)

## Otros

- [specs/](specs/) — 125 specs APP-* (PRODUCT/TECH docs por feature)
- [resources/](resources/) — assets bundled, channel-gated-skills
- [docker/](docker/) — imágenes
- [.github/](.github/) — CI, PR template, workflows
- [.warp/](.warp/), [.agents/](.agents/), [.claude/](.claude/) — config tooling
- [command-signatures-v2/](command-signatures-v2/) — datos firmas comandos

## Patrones clave (de WARP.md)

- **Entity-Handle UI**: `App` dueño, `ViewHandle<T>` referencias entre views. `AppContext` da acceso temporal durante render/eventos.
- **Feature flags runtime > cfg**: `FeatureFlag::X.is_enabled()` preferido sobre `#[cfg(...)]`. Variantes en `crates/warp_core/src/features.rs`. `DOGFOOD_FLAGS` / `PREVIEW_FLAGS` / `RELEASE_FLAGS` controlan rollout.
- **DB**: Diesel + SQLite, migrations en `migrations/`, schema en `app/src/persistence/schema.rs`.
- **GraphQL**: schema en `crates/graphql/api/schema.graphql`, codegen TS+Rust.
- **Cross-platform**: macOS, Windows, Linux, WASM. Código específico bajo `cfg`.
- **Cuidado `TerminalModel.lock()`** — locks múltiples = deadlock UI (beach ball). Pasar referencias ya bloqueadas, scope corto.
- **Tests**: `cargo nextest`. Archivos `${name}_tests.rs` o `mod_test.rs` adyacentes con `#[cfg(test)] #[path = "..."] mod tests;`.
- **Match exhaustivo**: evitar wildcard `_`.
- **Estilo**: sin anotaciones de tipo redundantes, imports arriba, `ctx` siempre último param (excepto closures), inline format args (`"{x}"` no `"{}", x`).

## Comandos dev

```bash
cargo run                          # build + run
cargo bundle --bin warp            # bundle app
./script/presubmit                 # fmt + clippy + tests
cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2
cargo fmt
cargo clippy --workspace --all-targets --all-features --tests -- -D warnings
```

Conexión a `warp-server` local:

```bash
cargo run --features with_local_server
SERVER_ROOT_URL=http://localhost:8082 WS_SERVER_URL=ws://localhost:8082/graphql/v2 cargo run --features with_local_server
```

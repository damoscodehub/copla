# Copla

A local desktop workspace for comparing and organizing AI-generated image variants.

## Development

Requirements: Node.js 24+, Rust stable, Windows C++ Build Tools, and WebView2. Use `npm ci` to install locked frontend dependencies. Run `npm run desktop` for the desktop development app, `npm run build` for the frontend, `npm test` for domain tests, and `cargo test --manifest-path src-tauri/Cargo.toml` for native tests. `npm run desktop:build` creates a Windows installer.

The project is under active implementation. See `MASTER_PLAN.md` for agreed scope, `docs/research/open-source-landscape.md` for reuse research, and `docs/testing/` for actual validation evidence. Unchecked release criteria are not claims of completed behavior.

The browser development server is a UI test harness; native filesystem capabilities require the desktop application. No cloud account is required.

## Security and license

Security reports are handled according to `SECURITY.md`. Copla is distributed
under the MIT License; see `LICENSE`.

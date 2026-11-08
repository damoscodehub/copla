# Copla

A local desktop workspace for comparing and organizing AI-generated image variants.

## Development

Requirements: Node.js 24+, Rust stable, and the native prerequisites for the
target platform. On Windows, install Windows C++ Build Tools and WebView2. On
Linux, follow [Linux support](docs/linux.md). Use `npm ci` to install locked
frontend dependencies. Run `npm run desktop` for the desktop development app,
`npm run build` for the frontend, `npm test` for domain tests, and `cargo test
--manifest-path src-tauri/Cargo.toml` for native tests. `npm run desktop:build`
creates a Windows installer; `npm run desktop:build:linux` creates Linux
packages.

The project is under active implementation. See `MASTER_PLAN.md` for agreed scope, `docs/research/open-source-landscape.md` for reuse research, and `docs/testing/` for actual validation evidence. Unchecked release criteria are not claims of completed behavior.

The browser development server is a UI test harness; native filesystem capabilities require the desktop application. No cloud account is required.

## Launching a local release build

After running `npm run desktop:build`, double-click `Launch Copla.cmd` in the
repository root to start the built application. This is a workspace launcher;
it does not create a portable edition or change where Copla stores its local
preferences and recovery data.

## Linux

Linux builds produce AppImage, Debian, and RPM artifacts. See
[Linux support](docs/linux.md) for prerequisites, installation, launch, and
compatibility notes.

## Security and license

Security reports are handled according to `SECURITY.md`. Copla is distributed
under the MIT License; see `LICENSE`.

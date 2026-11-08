# Linux verification gate

Date: November 8, 2026  
Target: x86_64 Ubuntu 22.04.5 LTS under WSL2

## Environment

- Node.js 24.22.2
- Rust stable 1.99.1
- WebKitGTK 4.1 development and runtime dependencies
- Tauri v2 bundle configuration

## Verified checks

- `npm ci` completed with zero known npm vulnerabilities.
- `npm run build` completed successfully.
- `npm test` completed with 7 passing tests.
- `cargo test --manifest-path src-tauri/Cargo.toml` completed with 9 passing tests, including the Linux filename-rule test.
- `npm run desktop:build:linux` completed successfully.
- The release binary started under WSLg and was stopped after its process was confirmed.
- The AppImage started through `APPIMAGE_EXTRACT_AND_RUN=1` under WSLg and was stopped after its process was confirmed.
- The AppImage is executable.
- The Debian package includes a desktop entry and a Debian-format MIT copyright notice.
- The Debian package declares `libwebkit2gtk-4.1-0` and `libgtk-3-0` runtime dependencies.
- The RPM package declares `webkit2gtk4.1`, uses the MIT license metadata, and identifies as x86_64.

## Produced artifacts

- `Copla_0.1.0_amd64.AppImage` (78.06 MiB)
- `Copla_0.1.0_amd64.deb` (3.38 MiB)
- `Copla-0.1.0-1.x86_64.rpm` (3.38 MiB)

## Scope boundary

This verifies x86_64 Linux on the Ubuntu 22.04 baseline. ARM64, distribution
store publishing, package signing, and testing on a non-WSL physical Linux
desktop remain separate release work.

# Contributing to Copla

Thank you for contributing. Copla is a local desktop application, so changes
must preserve offline behavior, user-file safety, and cross-platform design.

## Before opening a pull request

1. Keep code, comments, filenames, commits, and documentation in English.
2. Do not add real user images, session files, portable packages, credentials,
   local settings, or build outputs to the repository.
3. Add or update tests when behavior changes, especially around persistence,
   archive handling, and filesystem operations.
4. Run `npm run build`, `npm test`, and `cargo test --manifest-path src-tauri/Cargo.toml`.
5. Describe user-visible behavior and testing in the pull request.

## Security-sensitive changes

Changes involving Tauri permissions, filesystem access, archive extraction,
session recovery, external input, or dependencies require a focused security
review. Follow `SECURITY.md` for vulnerability reports.

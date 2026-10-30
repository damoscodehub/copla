# ADR 0001: Custom session domain with reusable infrastructure

Status: Accepted for prototype validation. Date: October 29, 2026.

Research in `docs/research/open-source-landscape.md` found relevant viewers and organizers, but no inspected whole application matched Copla's session, history, native file, and independent synchronization requirements without substantial adaptation.

Use Tauri 2, React, TypeScript, Rust, TanStack Virtual, Lucide, i18next, and explicit native image codecs. Keep session commands and viewport coordinates independent from framework state. Prefer native accessible HTML controls and add headless primitives only where needed. Do not copy code from restrictive or copyleft applications into the initial codebase. This preserves freedom to decide Copla's distribution license later.

Use a small controlled viewport engine initially, with react-zoom-pan-pinch as an evaluated fallback. A/B rendering shares that engine rather than using a second incompatible transform implementation. Confirm through M1 measurements; revisit if gesture quality or memory behavior fails.

Consequences: Copla owns domain correctness, history, file transactions, and integration tests. Existing libraries supply codecs, UI infrastructure, and virtualization. No claims of completed performance or non-Windows validation yet.

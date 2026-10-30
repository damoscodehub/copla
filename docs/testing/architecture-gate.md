# Architecture gate

October 29, 2026. Local Windows prototype.

- Native Rust/Tauri build succeeded using Rust 1.95 and installed C++ Build Tools.
- Automated native WebView launch confirmed the React interface and native bridge load. See `native-shell-gate.json`.
- Eight synthetic 4096px PNGs decoded in 341ms. After a 60-frame warm-up, 120 animated frames averaged 16.67ms, maximum 17.5ms in headless Edge. See `browser-render-gate.json`.
- The initial un-warmed run included a 283ms frame. Loading/preview state is required; this is not evidence of stall-free cold loading.
- Normalized transform tests cover aspect preservation, selected-target rules, pointer anchoring, and bounds.

Decision: proceed with the Tauri/component architecture. No evidence currently justifies changing shells. Continue image-native loading and integrated viewport checks in the first session slice.

Limitations: synthetic solid images are less demanding than representative artwork. Browser results are not a native packaged performance certification. Working-set, ICC examples, high-DPI visual review, and long-session performance remain release gates. Non-Windows validation requires those platforms. These outstanding checks do not prevent implementing the first usable slice.

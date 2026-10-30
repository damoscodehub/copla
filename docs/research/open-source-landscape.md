# Open-source landscape and reuse decision

Research date: October 29, 2026. Repository observations are point-in-time evidence, not guarantees of quality or continued maintenance.

## Recommendation

Build Copla's session and comparison domain with maintained reusable infrastructure rather than fork a whole application. Keep Tauri 2, React, TypeScript, and Rust. Reuse TanStack Virtual for thumbnail virtualization, Lucide for vector icons, and Rust image decoders. Evaluate a controlled viewport implementation against react-zoom-pan-pinch: Copla needs independently selected pan and zoom targets, normalized coordinates, and shared A/B clipping, which require application-specific orchestration either way.

No third-party application code has been copied. No downloaded application or repository script was executed during research. Dependency license notices must be retained in distributions; the project's own publication license remains undecided and does not block private development.

## Search coverage

Searched public repositories and official documentation for synchronized image comparison, split sliders, desktop viewers, AI image organization, generation metadata, React pan/zoom, virtualization, native image decoding, accessible primitives, and cross-platform shells. Read repository metadata, licenses, manifests, and selected implementation files. Followed canonical repository redirects. Examined whole applications and libraries separately; additional broad searches repeated these architectural categories rather than revealing a complete fit.

This is a bounded investigation, not a claim to enumerate every available project. Performance claims from project descriptions remain unverified until measured locally. GitHub push dates are maintenance signals only; dependency audit results and real platform tests remain implementation/release checks.

## Comparable applications

### MediaSyncView — design reference, not a fork

[Repository](https://github.com/Rogala/MediaSyncView), inspected commit `b8496385bdefcbc2e9cb40814cbb38a927adbf29`; last push observed May 11, 2026. AGPL-3.0. Repository contains a single-page HTML application and bundled p5. Its synchronized comparison and split-view interaction are relevant, but Copla's persistent annotations, transactional history, native paths, and portable documents require different foundations. Adopting it would also introduce distribution obligations not selected for Copla. Use publicly described interaction concepts only, not copied implementation.

### Infinite Image Browsing — strongest workflow reference

[Repository](https://github.com/zanllp/infinite-image-browsing), commit `2732c5522f5b52a3d07e31f73200ace4a4b6c1bf`; push observed October 29, 2026. MIT. Inspected the Vue package structure, Tauri manifest location, and `vue/src/util/stable-diffusion-image-metadata.ts`. It combines broader image organization and generation metadata with a Vue/Python-oriented ecosystem. A fork would inherit substantial library/indexing behavior unrelated to session comparison. Prefer studying metadata formats and retaining independent adapters over carrying its whole application. Any later source adaptation must preserve its MIT notice and cite the exact source revision.

### Stable Diffusion Prompt Reader — metadata reference

[Repository](https://github.com/receyuki/stable-diffusion-prompt-reader), commit `32b2eea715b5f7816222cef23af3deed7df04a30`; last push observed May 29, 2024. MIT. Inspected `sd_prompt_reader/image_data_reader.py`: dispatches format-specific interpretation using Pillow, piexif, and specialized parsers. Useful evidence that prompt extraction must be format-aware and conservative. Its Python UI/runtime does not justify a second runtime inside Copla. Maintenance is relatively old; do not assume coverage of current generator payloads. Reimplement minimal known format handling with synthetic tests rather than port the whole parser suite.

### LAP — desktop architecture reference

[Repository](https://github.com/julyx10/lap), commit `a3443e710c5c278a98af3c453f69523b789e6774`; push observed October 29, 2026. GPL-3.0. The repository has Tauri/Rust and Vue frontend directories, demonstrating a relevant cross-platform local photo-management approach. Its broad library, search, and AI features are outside Copla's first release. No code adoption: substantial product mismatch and unnecessary distribution/license decisions. Large-library claims need independent measurement.

### Image MetaHub — reference only; not permissive open source

[Repository](https://github.com/LuqP2/Image-MetaHub), commit `dee8bc8ebaa2285d2154b4ff67ed4fcad5c291c2`; push observed October 29, 2026. Inspected its actual LICENSE and package.json. The license is PolyForm Perimeter 1.0.1 with competition restrictions; GitHub's `NOASSERTION` was not treated as permission. The manifest shows Electron/React, licensing services, and a much larger AI/media dependency surface. Do not copy its source into Copla. Its publicly described metadata-oriented workflow is useful context only.

### TwinPixels — exclude from code reuse

[Repository](https://github.com/Eberload/TwinPixels), commit `86c0a687bcfd04eb9b185617569702e99b046efc`; push observed September 28, 2026. The actual LICENSE.md describes closed-source donationware and prohibits modifications. It is not an open-source foundation despite appearing in repository searches. Paired comparison is a product reference only.

## Component shortlist

### TanStack Virtual — adopt

[Repository](https://github.com/TanStack/virtual), commit `51b06cd5e078a289fe1cbf25a8d2e647aa98bfff`; October 29, 2026 push; MIT. Inspected the React adapter in `packages/react-virtual/src/index.tsx`. Headless virtualization fits a custom thumbnail browser without imposing visual design. Integration effort: low to moderate; test selection outside the rendered range and resizing. Install a released version, not unreleased main-branch APIs.

### react-zoom-pan-pinch — evaluated alternative

[Repository](https://github.com/BetterTyped/react-zoom-pan-pinch), commit `df6a7c6187a41bb0c7d294d0aedd7cd632369671`; October 24, 2026 push; MIT. Inspected its manifest, `src/core/instance.core.ts`, and test directory listing. It has transform callbacks and substantial gesture/bounds testing. Integration effort is moderate because eight controlled viewports and separate pan/zoom target rules still require Copla's coordinate engine. Avoid adopting its internal instance API or coupling sessions to its transform state. Keep as fallback if custom pointer handling does not pass M1 tests.

### img-comparison-slider — reference / optional

[Repository](https://github.com/sneas/img-comparison-slider), commit `c73939d61c8fb010342c9910f1de420edbf0b067`; October 12, 2026 push; MIT. Provides an existing accessible comparison component across frameworks. Copla also needs independent transformed layers and gesture ownership; a native range control plus clipping is a smaller integration surface for the first prototype. A guessed source path returned 404, so no source-level claims about its internals are made.

### OpenSeadragon — defer

[Repository](https://github.com/openseadragon/openseadragon), commit `36b92d228b121f5fa4c933aa5bb3d5eb10859e05`; October 29, 2026 push; BSD-3-Clause. Strong candidate if huge-image tiling becomes necessary. Adds an unnecessary tiled-viewer model for ordinary generated images initially. Reconsider if native preview plus full-image rendering cannot meet measured memory requirements.

### image-rs/image — adopt native codec boundary

[Repository](https://github.com/image-rs/image), commit `075e65d618a323e5cb48c1d8710a2eca25d690c6`; September 15, 2026 push. Cargo.toml specifies `MIT OR Apache-2.0`, correcting the API's single-license classification. Its feature-gated decoders cover the initial formats. Use explicit PNG/JPEG/WebP features, dimension/allocation limits, and bounded background work. Retain native full-resolution originals for display; previews are disposable, never replacements for originals.

### exifr — defer unless native metadata coverage needs it

[Repository](https://github.com/MikeKovarik/exifr), commit `6cbf6e921688faf7723e1f2e0b9e672d1f0aa21c`; March 29, 2024 push; MIT. Relevant general EXIF/XMP library but not a universal generation-prompt decoder. Prefer bounded native extraction and format-specific parsers initially; avoid passing every full image into JavaScript solely for metadata. If adopted later, test malformed input and review maintenance status again.

### Radix Primitives and Lucide — reusable UI infrastructure

[Radix](https://github.com/radix-ui/primitives), commit `eb3a2b7091293e27df8f9ba906632d8daa3d81a3`; October 14, 2026 push; MIT. Use individual primitives when required instead of importing a whole styled system. Accessibility still requires application-level focus and keyboard tests.

[Lucide](https://github.com/lucide-icons/lucide), commit `81875c8c42d2320fa5123b37fee3554f4ca2bddc`; October 29, 2026 push. Actual LICENSE contains ISC and MIT notices for inherited Feather icons. Preserve both. Adopt vector icons; no emoji controls or remote icon delivery.

## Decision and remaining evidence

Whole-app adaptation has high domain mismatch and upstream divergence costs. Building every primitive independently has needless accessibility, codec, and virtualization risk. Component reuse plus a custom session/viewport domain is the best fit based on inspected evidence.

M1 must establish native build viability, source-image loading, eight-view behavior, normalized pan/zoom correctness, memory constraints, high-DPI behavior, and packaged rendering. macOS/Linux support remains architectural until tested on those platforms. Metadata completeness and professional color fidelity are not implied by selecting a codec library.

Retain dependency license notices from exact installed releases, lock dependencies, and run package/native advisory checks before distribution. No project is declared vulnerability-free based on this search.

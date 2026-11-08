# Copla — Master Development Plan

Version: 1.3
Date: November 5, 2026
Status: Implementation is active and an internal desktop build exists. Research, foundation, architecture, and multiple product slices have been completed in working form; their formal release evidence remains incomplete. Public publication is not authorized by this plan until the release-readiness gates in Section 18 have passed and the owner explicitly authorizes publication. See `docs/research/open-source-landscape.md` and `docs/decisions/0001-component-reuse.md`.

## 1. Purpose and planning authority

Copla is a local desktop application for comparing, evaluating, and organizing images. Its initial focus is variants generated with AI. The name derives from “compare and play,” with “display” as an additional association.

The first complete release must support the entire agreed image workflow, not merely an image-viewing prototype. Development proceeds through usable increments so the owner can correct interaction and visual decisions before they become expensive to change.

This document defines the implementation and release plan. It does not by itself authorize publishing, purchasing services, creating remote repositories, or building cloud services. The repository now contains application code; this plan remains the governing source for scope, verification, and release decisions.

Requirements labeled **agreed** reflect explicit user decisions. Other implementation details are **planning defaults** selected to make the project actionable. Defaults may change after evidence from prototypes or user feedback; changes must preserve agreed requirements or be explicitly discussed.

## 2. Product outcomes

Users should be able to:

1. Start a session and add a batch of generated images without moving the originals.
2. Browse the batch and compare one to eight images in a focused workspace.
3. Inspect differences through grid, carousel, and A/B slider views.
4. Control pan and zoom synchronization independently.
5. Rate, tag, name, and annotate images individually or through supported selection actions.
6. Undo mistakes and recover work after an unexpected exit.
7. Save a lightweight local session or export a self-contained portable package.

Success means the comparison workflow feels immediate, preserves data reliably, and remains understandable without reading documentation.

## 3. Agreed scope

### First complete release

- Windows first, with macOS and Linux portability treated as an architecture requirement.
- Fully local operation without accounts, mandatory connectivity, uploads, or telemetry.
- PNG, JPEG, and WebP support, with an extensible format boundary.
- Many images per session; no more than eight displayed in the comparison stage at once.
- Grid, carousel with buttons and keyboard navigation, and A/B slider comparison.
- Separate pan and zoom synchronization settings: all, none, or selection.
- Integer ratings from 1 to 10, plus an unrated state.
- Applying a rating to a selection updates every selected image individually.
- Internal image names and a separate explicit operation to rename a physical file.
- A shared tag catalog with searchable, multi-select, creatable combobox controls.
- Per-image notes.
- Basic read-only generation metadata when recognized.
- Manual save, Save As, and automatic recovery, including never-saved sessions.
- Local session references plus portable export containing the images.
- A 100-operation undo/redo history per open session.
- Dark theme by default and a complete light theme.
- Responsive desktop layout, modern sans-serif typography, vector icons, visual interaction feedback, tooltips, and appropriate notifications.
- English source code, comments, paths, identifiers, technical documentation, commits, and initial interface text. User-entered content and external filenames may naturally use any language.
- Translation-ready interface architecture from the beginning.

### Explicitly outside the first release

- Video playback or video comparison.
- Cloud sync, accounts, collaboration, or an online image library.
- Image generation, editing, retouching, cropping originals, or transcoding exports.
- Professional RAW/HDR workflows and guarantees of reference-grade color accuracy.
- Ratings attached to persistent groups as independent entities.
- Advanced generation-metadata search, model integrations, or full workflow graph interpretation.
- Persistent undo history across session closure.
- Global library management across all sessions.
- External plugin execution or a public extension SDK.

Extensibility means clear internal interfaces and versioned data, not building these features early.

## 4. Primary user journey

1. Launch Copla and choose New Session.
2. Add images through a file dialog or drag and drop.
3. Browse virtualized thumbnails and choose a comparison set of up to eight images.
4. Switch between grid, carousel, and A/B while retaining image identity and useful view settings.
5. Adjust pan and zoom, optionally synchronizing their targets independently.
6. Assign ratings, add tags and notes, and inspect generation details.
7. Save the session as a `.copla` file.
8. Reopen it later with its annotations, order, comparison set, and saved view settings intact.
9. Export a `.coplapack` when the session needs to move to another computer.

## 5. Information architecture and interaction rules

### 5.1 Start screen and session lifecycle

The start screen provides New Session, Open Session, Recent Sessions, a session drop target, and Exit. Text labels are appropriate here because these are primary onboarding choices; icon-first controls apply mainly to the working interface.

- New Session opens an empty workspace with an explicit image import action.
- Open Session uses a native dialog accepting `.copla` and `.coplapack`.
- Recent Sessions contains at most ten existing session files, ordered by successful opening or saving. Deduplicate equivalent paths. Revalidate on launch and when presenting the list; omit missing files. Distinguish missing files from temporarily inaccessible locations where the platform permits it.
- Dropping one session file opens it through the same validation and unsaved-change flow as the dialog.
- Dropping images on the start screen creates an unsaved session and imports them.
- One session is open per application window; the first release uses a single working window to avoid simultaneous-write ambiguity.
- Opening another session or exiting offers Save, Discard, and Cancel when document data has changed. An empty untouched session does not prompt.
- File association and launch-with-file support are included in packaging work.
- Concurrent opening of the same session must be detected through an application lock. A second writer is not allowed; offer opening a copy where appropriate.

### 5.2 Workspace structure

- Top toolbar: session controls, view mode, pan sync, zoom sync, undo, redo, and settings.
- Center: comparison stage, occupying the largest available area.
- Bottom: collapsible thumbnail browser with selection, ratings, and missing-file indicators.
- Side inspector: image details, rating, tags, notes, generation metadata, and file actions.
- Compact status area: image count, selected count, save/recovery state, and background operation progress.

Planning default: a 960 × 640 logical-pixel minimum window, validated against desktop scaling. On narrow windows the inspector becomes a drawer, secondary actions enter an overflow menu, and the thumbnail strip can collapse. No mobile interface is promised.

### 5.3 Distinguish the four image concepts

- **Session collection:** all image records in the session.
- **Selection:** image records targeted by batch actions; may exceed eight.
- **Active image:** the image currently receiving focus and single-image inspection.
- **Comparison set:** one to eight image records pinned to the stage.

Click selects and activates; Ctrl/Cmd-click toggles selection; Shift-click selects a range in the current browser order. “Compare Selected” replaces the comparison set when one to eight images are selected. Selecting more than eight remains valid for batch ratings and tags, but comparison requires reducing the set; do not silently discard selections.

First import fills available comparison slots with the first images, up to eight. Subsequent imports do not unexpectedly replace the comparison set. Removing an image updates selection, active image, and comparison references coherently.

### 5.4 Grid

Grid displays every image in the comparison set. Defaults: one image fills the stage; two use two columns; three or four use two columns; five through eight use up to four columns according to available space. Tiles use consistent dimensions and preserve image aspect ratios.

Each tile shows a compact name, rating, active/selected indicators, and loading or error state. Reordering the comparison set is separate from reordering the session collection. Keyboard alternatives accompany drag reordering.

### 5.5 Carousel

Carousel displays one image at a time from the comparison set. Previous/next vector buttons and Left/Right keys navigate it. The thumbnail browser provides access to the whole session; “Replace Active Slot” lets the user audition another image without rebuilding the set.

Navigation stops at the ends by default; controls are disabled visibly. Show position and total. Editing a note, combobox, slider, or other text/control field takes priority over global navigation shortcuts.

### 5.6 A/B slider

A/B displays exactly two distinct images from the comparison set, with clear A and B labels and a swap action. When more than two images are pinned, explicit A/B pickers choose the pair. With fewer than two, explain the requirement and offer a selection action.

- Default divider: vertical at 50%, with an optional horizontal orientation.
- Divider dragging changes the reveal boundary, not either image's pan.
- Keyboard focus on the divider supports arrow adjustment, Home/End, and a readable percentage.
- Fit is the default: show complete, centered images without distortion.
- Fill is optional: preserve proportions while visually cropping overflow.
- Transparent areas use a configurable neutral/checkerboard background.
- Changing the A/B pair resets the divider to its saved/default position but preserves each image's viewport state.
- A/B offers a clearly labeled “Align Views” action that fits and centers both images and enables all-target pan/zoom synchronization. It does not silently override a user's existing sync choices.
- Different compositions are not automatically registered or aligned by image recognition.

### 5.7 Pan and zoom

Pan uses pointer dragging inside the image; slider handles and selection controls have separate hit targets. Wheel zoom is anchored at the pointer. Provide visible zoom controls, Fit, Fill, Actual Size, and Reset View. Trackpad behavior and high-DPI scaling must be validated in the rendering milestone.

Pan and zoom each have their own independent mode:

- None: only the directly manipulated image changes.
- All: all images in the current comparison set change, including carousel items temporarily not visible.
- Selection: selected members of the comparison set change together when the gesture originates on a selected member. A gesture on an unselected image affects only that image. With no selected comparison images, behavior is independent and the control indicates that no targets are selected.

Target membership is captured at gesture start. A gesture generates one shared viewport update, avoiding feedback loops between tiles.

Represent center positions in normalized image coordinates and zoom as a multiplier relative to the chosen Fit/Fill baseline. This aligns corresponding image regions across different resolutions. Clamp each image independently at its boundaries; different aspect ratios can therefore produce different visible edges. Actual Size is a per-image explicit command with high-DPI-aware semantics, not an assertion that equal relative zoom means equal source-pixel scale.

Planning defaults: pan and zoom sync start at None; reset returns to Fit and center. View changes do not consume edit-history steps or trigger discard prompts, but are captured on explicit save and in recovery snapshots. Document annotations and structure do trigger dirty state.

## 6. Evaluation and organization

### Ratings

- Store an integer from 1 through 10 or null for unrated; zero is not a rating.
- Use a compact numeric control with ten selectable values and a clear action.
- Mixed selections show “Mixed” until a rating is applied.
- Keyboard defaults: 1–9 assign those values, 0 assigns 10, and an explicit menu action clears the rating. Shortcuts act only when the workspace owns focus.
- A batch rating is atomic and one undo step, restoring each prior value independently.

### Tags

- Store the global catalog in application data; store the tags actually used by each session inside that session.
- The combobox supports searching existing tags, selecting multiple tags, and creating a tag by typing.
- Trim outer whitespace and normalize Unicode; match labels case-insensitively for duplicate prevention while preserving a display label.
- Batch editing supports adding or removing a tag without replacing unrelated tags. Indicate tags present on only some selected images.
- Import catalog suggestions from successfully opened sessions by normalized label; do not rewrite unrelated session files.
- Removing an unused suggestion from the global catalog does not remove tags from saved sessions. Global rename propagation across sessions is deferred.
- Session-local stable tag IDs and label snapshots keep sessions self-contained and avoid cross-machine ID assumptions.

### Notes and names

- Notes are plain text, attached to one image at a time. No automatic batch overwrite of notes in version one.
- Group contiguous typing into one history step until blur or a 750 ms idle pause; enforce a maximum group duration to avoid excessively large edits.
- “Rename in Session” edits a display label without modifying the source path.
- “Rename File on Disk” is a separate single-file action with destination preview. Preserve the extension in version one, validate platform-specific names, and never overwrite an existing file.
- Update the active session reference after a successful disk rename. If the internal label was manually customized, preserve it.
- Explain that other saved sessions referencing the old path may need relinking; do not silently edit those sessions.

### Browsing aids

Planning defaults for version one: filter by internal/file name, tag, and rating/unrated; sort by import order, name, or rating. Filtering does not remove images from the comparison set or alter saved annotations. Hidden selections remain visible through a selection-count indicator and a clear-selection action.

## 7. Undo and redo

Use a command-based history with up to 100 completed operations per open session. Store inverse data, not decoded image copies. When the limit is reached, discard the oldest operation.

Included: import/remove image records, session and comparison ordering, internal renaming, ratings, tag assignment, note edits, relinking, and supported disk renames. Removal means removal from the session only; there is no original-file deletion action in version one.

- Batch actions count as one operation and undo to the previous values of all affected records.
- Save does not clear history. Close does. Recovery opens a recovered document with a fresh history.
- New document edits after undo clear the redo branch.
- Selection, focus, pan, zoom, filters, mode switches, global settings, save, and export do not enter document edit history.
- Buttons expose the next operation label, such as “Undo Rate 6 Images.”
- Windows/Linux: Ctrl+Z, Ctrl+Shift+Z, and Ctrl+Y. macOS: Cmd+Z and Cmd+Shift+Z.
- Text fields use the appropriate editor undo behavior while focused; commit note groups at focus transitions so global undo remains predictable.
- Dirty state compares the document with the saved baseline rather than trusting a history index, including when old entries are evicted.

Disk rename undo/redo is conditional: verify source identity, destination availability, and permission before changing anything. On failure, keep the history cursor unchanged and explain the issue. Allow the user to explicitly discard that history entry if they need access to older operations. A session discard does not reverse completed disk operations; communicate this distinction in the file action and close flow where relevant.

## 8. Session format and persistence

### 8.1 Local format

Use `.copla`, containing UTF-8 JSON with a versioned schema. Keep large image binaries and thumbnail caches outside it. Validate on both read and write.

The schema will contain:

- `schemaVersion`, `sessionId`, `title`, `createdAt`, and `updatedAt`.
- Ordered image records with stable IDs, display labels, ratings, tag IDs, and notes.
- Source locators: a path relative to the session directory where possible, an absolute fallback, original basename, byte size, modification time, and an optional content hash calculated when needed.
- A session tag dictionary containing used labels.
- Comparison IDs, active image, A/B pair, mode, synchronization preferences, and lightweight viewport state.
- Cached metadata summaries with parser/version and file-fingerprint information so stale data can be invalidated.

Store timestamps in UTC and format them using the active locale. Resolve paths through native platform APIs; never concatenate Windows-specific separators in domain logic. Stable IDs survive names and path changes.

Never use filenames alone as image identity. File size and modification time are hints, not proof of identical content. Detect duplicate imports of the same resolved file; different files with identical content may be retained intentionally.

### 8.2 Save and Save As

Serialize to a temporary sibling file, flush as supported, validate, then replace the destination using the platform's safe replacement mechanism. Preserve the previous valid file if writing fails. Verify behavior on Windows rather than assuming POSIX replacement semantics.

Save As rebases relative source paths, preserves the in-memory session identity, and switches the active location only after a successful write. Detect external session changes before overwriting; offer reload or Save As instead of silently replacing another writer's changes.

Schema migrations are explicit and tested. Preserve a backup before migrating on disk. Reject unsupported future schemas with a useful message; do not silently drop unknown data and save over the file.

### 8.3 Missing or changed images

Open the session even if some sources are unavailable. Keep annotations and show actionable placeholders. Offer Locate File and Locate Folder with a review of proposed matches. Never silently bind an ambiguous basename match.

When an external file changes, invalidate cached previews and metadata. Preserve annotations, disclose the change, and refresh safely. If the original path resolves to a different file, require review before treating it as the same image.

### 8.4 Recovery

- Maintain recovery snapshots in application data, keyed by session ID and source location.
- Write after a two-second idle debounce, with a maximum 15-second interval during ongoing edits.
- Use a safe replacement strategy and retain the last two valid snapshots per active session.
- Include committed annotation edits and current note drafts; never require an initial manual save.
- After an unexpected exit, offer recover, inspect saved version, or discard recovery.
- Recovery never silently overwrites the manually saved session.
- On explicit Discard, clear the relevant recovery data so discarded edits do not reappear.
- After successful save and a clean close, remove obsolete recovery snapshots. Do not automatically delete unresolved crash recovery solely because it is old.

The interval is a target loss window, not a guarantee against storage failure or process termination during an unfinished write.

### 8.5 Portable packages

Use `.coplapack`, a ZIP container with a versioned `manifest.json`, `session.copla`, and an `assets/` directory. Asset paths use stable IDs to avoid filename collisions. Copy original bytes, preserving embedded metadata; display names are preserved in session records.

Export runs as a cancellable background task, checks available space, calculates checksums, and writes to a temporary package before final replacement. Missing/unreadable assets prevent a “complete” export; offer locating them or an explicitly labeled partial export listing omissions.

Opening a package validates its contents, then asks for an extraction destination and creates a normal local session with relative asset references. Edits and disk renames affect those extracted copies. The source package remains unchanged unless explicitly exported again.

Reject path traversal, absolute archive paths, symlink escape routes, suspicious compression ratios, excessive entry counts, and oversized expanded content. Do not include original absolute source paths, global settings, unused global tags, or recovery history in portable output. Note that original image metadata itself may contain private information; metadata stripping is not part of version one.

## 9. Technical architecture

### 9.1 Provisional implementation baseline

M0 research selected a custom Copla domain using reusable infrastructure. Tauri/React/Rust remains the baseline for M1 validation. Adopt TanStack Virtual and Lucide, use explicit native image codecs, and implement shared normalized viewport controls; react-zoom-pan-pinch remains an evaluated fallback. See ADR 0001 for rationale and the research report for inspected revisions and licenses.

- **Desktop shell:** Tauri 2 with Rust for native services.
- **Interface:** React and TypeScript, with Vite and pnpm for development/build tooling.
- **Styling:** CSS variables for design tokens and CSS Modules for component styling; accessible headless primitives for menus, dialogs, tooltips, and comboboxes after a small compatibility check.
- **Icons and fonts:** bundled SVG icon set, such as Lucide, and a locally bundled sans-serif font with a system fallback. Verify redistribution licenses.
- **Client state:** Zustand for workspace orchestration; pure TypeScript domain commands for mutations and history. Keep transient gesture updates out of broad React renders.
- **Validation:** typed JSON contracts validated in TypeScript and Rust; contract fixtures ensure both sides agree.
- **Localization:** i18next/react-i18next with bundled English resources and named interpolation.
- **Tests:** Vitest and React Testing Library for domain/interface behavior; Rust tests for native services; browser interaction tests plus a small real packaged-app smoke suite.

Exact dependency versions will be selected from compatible stable releases and locked after the research gate, when implementation bootstrapping begins. No backend web server, database server, or cloud infrastructure is required. JSON is sufficient for version-one sessions and the tag catalog.

Tauri's system WebView approach and native dialog/filesystem facilities fit this local desktop workflow. The tradeoff is platform-dependent rendering behavior, which must be tested rather than assumed identical. References: [Tauri architecture](https://v2.tauri.app/concept/architecture/), [dialogs](https://v2.tauri.app/plugin/dialog/), and [filesystem](https://v2.tauri.app/plugin/file-system/).

### 9.2 Required early architecture gate

Before committing to the full shell, validate eight-image rendering, pointer gestures, PNG/JPEG/WebP loading, color/profile behavior, high-DPI presentation, native file access, and packaged performance on Windows. Conduct a small macOS/Linux portability check when those systems are available.

If a material WebView issue cannot be resolved with bounded work, compare Electron using the same fixtures before proceeding. Preserve frontend/domain boundaries so this is a shell decision rather than a full rewrite. Electron embeds Chromium and Node.js, offering a different runtime-consistency tradeoff; it is a fallback, not a second implementation. [Electron introduction](https://www.electronjs.org/docs/latest/)

### 9.3 Module boundaries

1. Presentation: components, focus, accessible controls, themes, localized text.
2. Domain: image records, session rules, commands, history, selection semantics, schema transformations.
3. Viewport engine: coordinate transforms, gesture targeting, synchronization, clipping, and render scheduling.
4. Application services: imports, saves, recovery, export jobs, tag suggestions, and native bridge orchestration.
5. Native infrastructure: filesystem operations, image probing/previews, metadata parsing, archive handling, locks, and OS integration.

Frontend code never receives unrestricted arbitrary filesystem execution. Expose narrow, validated native operations and file access only for authorized user-selected sources and application-owned data. Disable remote page navigation and avoid executing scripts or URLs found in image metadata.

### 9.4 Proposed project layout

```text
copla/
  MASTER_PLAN.md
  README.md
  docs/
    architecture/
    decisions/
    product/
    testing/
  src/
    app/
    components/
    domain/
    features/
      comparison/
      library/
      metadata/
      sessions/
      settings/
    services/
    viewport/
    styles/
    locales/en/
  src-tauri/
    capabilities/
    src/
      commands/
      imaging/
      metadata/
      persistence/
      portability/
  tests/
    fixtures/
    integration/
    e2e/
```

These are planned paths, not directories to scaffold during the planning phase.

### 9.5 Extension boundaries

- Image format adapters expose capability checks, dimension probing, decoding/preview generation, and metadata extraction.
- Metadata readers are independent from image decoders. Initially support recognized PNG text-based parameters and conservative extraction of readable JPEG/WebP EXIF/XMP/comment fields. Recognize common AI parameter blocks when present; provide escaped raw text for unrecognized textual payloads. No universal prompt extraction is promised.
- A native service adapter isolates dialogs, paths, reveal-in-folder, and modifier conventions.
- The media record can later gain a different media kind through a schema migration. Do not add video playback abstractions before video requirements exist.
- Future sync would require conflict, identity, privacy, and account design. Stable IDs help, but this plan does not claim that adding cloud sync will be trivial.

## 10. Rendering, quality, and performance

Virtualize the thumbnail browser. Generate previews in background workers with bounded concurrency. Load stage images on demand, prefetch only nearby carousel items, cancel obsolete work, and release decoded surfaces when evicted from a memory-budgeted cache.

Prefer image elements with shared transforms and clipping for the initial renderer. Introduce canvas or a tiled rendering approach only if benchmark evidence requires it. Large-image inspection may use progressive previews and higher-resolution content, but must clearly indicate preview/loading status; never present an upscaled thumbnail as verified full detail.

Test EXIF orientation, transparency, sRGB and embedded ICC examples, grayscale, malformed files, and animated WebP/PNG. Version one treats supported animated containers as still previews of their first frame and labels that limitation. Do not claim HDR or professional color-managed proofing.

Planning acceptance targets, to be measured and revised explicitly after the architecture gate:

- Reference workload: 500 image records; eight 4096 × 4096 comparison images; a 16 GB Windows machine with SSD and a documented GPU/display scale.
- At least 50 displayed frames per second during steady warmed pan/zoom on that workload, with no repeated stalls above 100 ms.
- Visible thumbnail page within two seconds of starting a local SSD import; the entire batch may continue processing in the background.
- Warm carousel navigation within 150 ms for prefetched images.
- Ordinary 500-record session save within one second on the reference SSD, excluding original image copying.
- Target working set below 1.5 GB on the reference workload, with bounded cache growth over a 30-minute browse cycle.

These are release targets, not measured results. Add a stress case with 2,000 records and oversized images to verify graceful degradation rather than treating it as a guaranteed operating limit. Reject unsafe dimensions before allocating excessive memory. Make limits and actionable errors visible.

## 11. Visual design, accessibility, and localization

Use a restrained flat visual system with neutral surfaces and one primary accent. Dark is the initial theme; Light and optional System are available. Accent colors distinguish interaction state without tinting the comparison background. Keep the image surround neutral and configurable.

Define tokens for surface/background, text hierarchy, borders, focus, selection, success, warning, and error. Bundle resources locally; no font CDN or remote icon loading.

Every interactive control needs hover, pressed, keyboard focus, disabled, and busy states where applicable. Drop zones show accepted/rejected drag feedback before the drop. Selection must be distinguishable from active focus without color alone.

Use accessible names for every icon-only button. Tooltips include action names and shortcuts. Prioritize WCAG 2.2 AA contrast and keyboard behavior for the web interface. Respect reduced motion, test 200% scaling, trap focus in dialogs, restore it on close, and avoid keyboard traps in the stage.

Use toasts for completed exports, recoverable errors, and file-operation results; use inline status for saving/loading and persistent actionable errors. Do not create a toast for every rating or tag change. Announce important changes to assistive technology without flooding its live region.

All interface strings, accessibility labels, errors, menus, and notifications belong to locale resources. Native errors return stable codes and parameters for localized presentation. Avoid sentence concatenation, reserve space for longer translations, use locale-aware numbers/dates, and use logical layout properties for eventual right-to-left support. The initial release ships English only. [react-i18next documentation](https://react.i18next.com/latest)

## 12. Delivery milestones and dependencies

### M0 — Open-source research, reuse decisions, and project foundation

Status: **Completed.** The research report and component-reuse decision are recorded in `docs/research/open-source-landscape.md` and `docs/decisions/0001-component-reuse.md`. Copla uses selected permissively licensed components and a purpose-built domain layer rather than adapting a whole application.

Before application implementation or final architecture selection, conduct a thorough, bounded internet investigation of current open-source projects and comparable applications. Research is a required deliverable, not an optional background activity.

Research three complementary areas:

- Comparable desktop applications for image comparison, A/B sliders, synchronized viewing, batch evaluation, and AI image organization.
- Reusable components for pan/zoom, image decoding and previews, virtualized browsing, generation metadata extraction, accessible controls, and local persistence.
- Product interaction patterns worth learning from, including limitations and usability problems. Non-open-source applications may inform product analysis, but their code and assets are not reuse candidates without an appropriate license.

Start with broad discovery through search engines, repository hosting platforms, and official project documentation. Shortlist candidates by relevance, then inspect the actual repositories of promising candidates. Check current release activity, issue handling, maintenance signals, documentation, tests, dependency burden, supported platforms, and architectural fit. Do not use popularity or recent commit counts as substitutes for technical evaluation.

For each serious reuse candidate, record:

- Canonical repository and documentation links, research date, and inspected release or commit.
- The precise Copla requirement it could satisfy and whether it is a whole-app foundation, reusable component, or design reference only.
- License and attribution requirements, redistribution and modification constraints, relevant dependency licenses, and compatibility with the intended distribution model. Public source visibility alone does not establish reuse rights; unresolved licensing questions block adoption, not the rest of the research.
- Evidence of maintenance and platform support, known limitations, relevant security concerns, and performance evidence or missing measurements.
- Estimated integration or adaptation effort, upstream divergence and maintenance costs, and reasons to accept or reject the candidate.

Compare three implementation paths explicitly: adapting an existing application, building Copla around selected components, and implementing specific capabilities ourselves. Evaluate fit against the agreed session model, offline operation, file safety, undo behavior, eight-image viewing, portability, and localization. Do not reduce agreed product requirements merely to fit an existing project.

Keep the search comprehensive but bounded: cover every research area, inspect the strongest candidates, and stop when additional searches yield no materially different viable approach. Document search coverage and remaining uncertainties rather than claiming to have found every project on the internet. Separate verified repository facts from inferences and identify claims that require M1 benchmarking.

Deliverables: an English research report at `docs/research/open-source-landscape.md`, a concise shortlist with source links and rejection reasons, a reuse/adapt/build recommendation, and an architecture decision record. Update this master plan to reflect the findings before bootstrapping. Do not copy project code or execute unfamiliar repository scripts merely to prepare the report; any later prototype evaluation should use isolated fixtures and reviewed setup steps.

Research exit: every proposed adopted project has a documented purpose, a verified license, and a maintenance assessment; unresolved technical claims have explicit M1 checks; the architecture baseline is confirmed or revised with evidence.

After the research gate, establish the repository and specification foundation:

Create the Git repository, local identity, ignore rules, architecture decision records, issue-sized backlog, fixture policy, and development setup documentation. Record the toolchain and lockfiles when bootstrapping begins.

Exit: research deliverables and the reuse decision are complete; requirements map to milestones and acceptance checks; local identity is verified; no originals, caches, credentials, or build outputs are tracked.

### M1 — Architecture and rendering validation

Status: **Working validation completed; release evidence incomplete.** The Tauri/React shell, native bridge, and an eight-image synthetic rendering exercise have been built and tested. The recorded architecture gate includes warmed-frame measurements near 16.67 ms. Fresh-install and real desktop automation coverage remain release work.

Create only the technical prototype needed to validate the shell, renderer, and shortlisted components selected through M0. Exercise eight images, high-DPI, native reads, gesture synchronization, format handling, memory behavior, and packaged execution. Resolve research uncertainties with focused comparisons rather than implementing multiple complete applications.

Exit: benchmark evidence supports the chosen shell or a documented alternative is selected. No full interface investment before this gate.

### M2 — First complete session slice

Status: **Implemented for internal use; needs release verification.** The repository includes session persistence, image import/load support in native and browser modes, and the interface foundation. The specific recovery, interruption, and fresh-install exercises in the exit criteria must still be demonstrated.

Deliver start screen, new/open, import, single-image viewing, basic session schema, Save/Save As, dirty-state prompts, and the recovery foundation. Establish themes, locale resources, and the command-history architecture now rather than retrofitting them.

Exit: a user can create, save, close, reopen, and recover an unsaved session; files remain unchanged. Request the owner's first hands-on feedback.

### M3 — Comparison workspace

Status: **Implemented and iterated with owner feedback; needs formal acceptance testing.** Grid, carousel, A/B comparison, viewport controls, and responsive UI refinements have been implemented. All mode combinations, high-DPI behavior, and native interaction automation remain to be proven for a release candidate.

Deliver virtualized browsing, comparison-set selection, grid, carousel, A/B slider, Fit/Fill, pan/zoom synchronization, keyboard behavior, and responsive workspace layout.

Exit: one through eight images work in applicable modes; all nine pan/zoom mode combinations behave correctly; mismatched dimensions and transparency are covered. Request hands-on comparison feedback.

### M4 — Evaluation and organization

Status: **Partially implemented; verification and scope audit required.** The release gate must compare the actual behavior against every agreed organization, metadata, physical rename, and 100-step history requirement before it can be claimed complete.

Deliver ratings, batch rating, shared tag suggestions, notes, names, basic filters/sorting, metadata inspection, and complete edit-history coverage. Add disk rename with conditional undo and recovery journaling.

Exit: batch operations restore individual prior values; history caps at 100; disk rename conflicts cannot overwrite files; focus prevents accidental shortcuts while typing.

### M5 — Portable sessions and resilience

Status: **Partially implemented; release evidence incomplete.** Native storage and portable-package code exist. The native test suite requires execution in a normal Windows environment because restricted temporary-directory permissions prevented several file-system tests from completing in the current environment.

Deliver portable export/import, missing-file relinking, recent-session validation, dropped-session opening, migrations, concurrent-write detection, and interruption-safe persistence.

Exit: a package opens correctly without original paths; corrupt or malicious archives are rejected; missing files preserve annotations; save/export failures preserve the last valid output.

### M6 — Release hardening and Windows distribution

Status: **Not started as a formal release gate.** This is the current controlling milestone. No public source publication, release tag, installer publication, or remote push is a substitute for the checks below.

Complete keyboard/accessibility review, both themes, performance measurements, fresh-install testing, file associations, dependency/license review, and an installer with an offline runtime strategy. Document signing status and whether any installation prerequisite requires downloading; runtime app use must remain offline.

Exit: all release acceptance checks pass; measured limits and known issues are documented; no developer tools are required to use the installed app. Deliver a release candidate for owner evaluation before public distribution.

### M7 — Subsequent platform enablement

Status: **Linux x86_64 implementation and local verification complete.** AppImage, Debian, and RPM packages build successfully on the Ubuntu 22.04 baseline; the release binary and AppImage start under WSLg. The repository contains Linux setup and launcher documentation, a platform-safe filename policy, Ubuntu 22.04 CI coverage, and verification evidence at `docs/testing/linux-gate.md`. ARM64, package signing, store publishing, and validation on a non-WSL physical Linux desktop remain separate release work.

Package and validate macOS and Linux separately. Test paths, permissions, filenames, shortcuts, WebView rendering, color, dialogs, drop behavior, and installers. Handle signing/notarization and distribution channels as explicit release tasks.

This milestone is planned future work, not part of the first Windows release's completion criteria. Cross-platform architecture is required from M0 onward; cross-platform certification is not implied until tested.

Dependency order: M0 research and reuse gate → M0 project foundation → M1 → M2 → M3 → M4 → M5 → M6; M7 follows the Windows release. No application bootstrapping precedes the research gate. Accessibility, localization, testing, and safe persistence run through every milestone. Calendar estimates should be established after M1, when the major rendering risk is measured; avoid invented delivery dates now.

## 13. Verification strategy

### Domain tests

Cover selection and comparison limits, rating validation, batch inverse commands, note grouping, redo invalidation, history eviction, dirty-baseline detection, tag normalization, path-independent identities, schema migrations, and sync target calculation. Test combinations and invariants rather than mirroring component implementation.

### Native and persistence tests

Use temporary directories and synthetic fixtures to exercise safe replacement failure, read-only locations, Unicode/long/case-sensitive paths, rename conflicts, missing assets, modified sources, package checksums, extraction confinement, disk-full simulations, and recovery interruption.

### Interface and integration tests

Cover keyboard focus, mixed-value controls, import cancellation, mode transitions, slider dragging versus panning, all sync combinations, filtered selections, restore-on-reopen, metadata absence, and user-visible error recovery. Use accessible roles/selectors where possible.

Browser tests with mocked native adapters are useful but insufficient. Maintain a small real application test suite for dialogs, filesystem integration, packaging, and launch behavior. Select the supported desktop automation route during M1 based on the current Tauri tooling; do not assume browser automation proves native integration. [Tauri testing](https://v2.tauri.app/develop/tests/)

### Manual release exercises

Run a fresh-machine offline launch, a 30-minute comparison session, a forced-termination recovery, an export/import on another path, high-DPI resizing, keyboard-only operation, and a visual pass in both themes. Use owned or synthetic fixtures with known metadata; never commit private user images.

## 14. Release acceptance checklist

- [ ] All five startup choices work, including a maximum of ten valid recent sessions.
- [ ] Session and image drag-and-drop routes are distinct and preserve unsaved-change handling.
- [ ] PNG, JPEG, and WebP open with correct orientation and transparency behavior.
- [ ] Sessions contain many images while the comparison set never exceeds eight.
- [ ] Grid, carousel buttons/keys, and A/B slider work with clear focus and selection semantics.
- [ ] Pan and zoom support independent All, None, and Selection modes.
- [ ] Fit and Fill never distort or modify original image content.
- [ ] Ratings accept 1–10/unrated and apply atomically to selections.
- [ ] Tags are searchable, creatable, reusable across sessions, and portable.
- [ ] Notes and internal names survive save/open and undo/redo.
- [ ] Physical rename is explicit, collision-safe, and conditionally undoable.
- [ ] Basic supported generation metadata is readable; missing metadata is handled honestly.
- [ ] Manual save, Save As, dirty prompts, and recovery protect documents under tested failures.
- [ ] Portable packages open without original sources and do not expose local absolute paths in their manifest.
- [ ] History supports 100 operations, grouped edits, keyboard shortcuts, and correct branch invalidation.
- [ ] Original removal from session never deletes files from disk.
- [ ] Missing and changed sources are actionable without losing annotations.
- [ ] Dark/light themes, vector icons, feedback, tooltips, and keyboard accessibility are complete.
- [ ] Interface resources are translation-ready and all authored project material is English.
- [ ] Performance targets are measured or revised explicitly with the owner before release.
- [ ] Windows installation and runtime behavior are documented and verified.

## 15. Git and project workflow

Initialize Git at implementation kickoff, using repository-local identity:

```text
user.name = damo
user.email = damoscodehub@gmail.com
```

Use `main` as the stable branch and short-lived `feat/`, `fix/`, or `docs/` branches as useful. Keep commits small and use English messages such as `feat: add atomic session saving`. A remote hosting provider/repository is not assumed or created by this plan. Before a remote is added, inspect the final tracked file list, select a publication license, and obtain explicit owner authorization for the target repository and visibility.

Track source, tests, small synthetic fixtures, lockfiles, and architecture decisions. Ignore private images, portable packages, user sessions, recovery files, local settings, generated thumbnails, dependency directories, and build outputs. Store sample schemas as deliberately named test fixtures so ignoring user sessions does not hide intentional test data.

For each feature: identify acceptance criteria, implement the smallest complete behavior, run relevant checks, review the diff, and record user-visible behavior. Use formatting, linting, type checks, targeted tests, and native checks in local automation. Add hosted CI only when a remote is selected; keep the checks runnable locally.

Version releases semantically. Keep session schema versions independent from application versions. Record shell decisions, persistence guarantees, viewport semantics, and portability tradeoffs in concise English architecture decision records.

## 16. Principal risks and mitigations

- **Image memory pressure:** virtualized lists, bounded decode work/cache, previews, cancellation, and an early eight-image benchmark.
- **WebView differences across platforms:** an explicit rendering gate, isolated shell services, and real platform validation before claims of support.
- **Confusing selection and synchronization:** distinct visual states, explicit target rules, focused usability exercises, and no silent truncation.
- **Physical rename breaking other sessions:** separate action, clear scope, source identity checks, relinking tools, and no background edits to other sessions.
- **Interrupted persistence or external changes:** safe replacement, recovery snapshots, write conflict checks, and fault-injection tests.
- **Portable export size and safety:** progress, cancellation, checksums, bounded extraction, and explicit handling of missing assets.
- **Inconsistent generation metadata:** independent parsers, bounded input handling, source attribution, raw text fallback, and no execution of metadata content.
- **Scope expansion:** keep the full agreed first release intact while deferring video, cloud, professional color workflows, and global library features.

## 17. Definition of done and next action

A milestone is done when its behavior is implemented, relevant checks pass, failure cases are handled, documentation matches actual behavior, and its exit criteria are demonstrated. A passing build alone is insufficient.

The next project action is the M6 release-readiness gate in Section 18. It is intentionally verification-only: it does not change product scope or publish the repository. Any defect, unimplemented agreed requirement, failed check, or license uncertainty returns the project to the relevant milestone before public publication is considered.

This master plan is the baseline for implementation. It intentionally leaves exact visual spacing, minor shortcut refinements, and dependency patch versions adjustable while preserving the agreed product behavior.

## 18. Current release-readiness state and publication gate

### 18.1 Repository snapshot — November 5, 2026

- The current branch is `main`; the most recent commit is `20b5518` (`fix: persist browser image imports across session reloads`).
- The generated inventory file `filetree.txt` is not part of the product and is now ignored. It must remain untracked; no user file was removed to clean the repository.
- No Git remote is configured. The proposed destination is `https://github.com/damoscodehub/copla.git`, but no connection or push has occurred.
- `npm run build`, `npm test`, `cargo test --manifest-path src-tauri/Cargo.toml`, and `cargo build --release --manifest-path src-tauri/Cargo.toml` have passed locally. The frontend suite has seven passing tests and the native suite has seven passing tests.
- `npm run desktop:build` has produced the NSIS installer successfully. The current unsigned installer is a verification artifact, not a public release artifact.
- The browser UI smoke test has passed with generated fixtures, covering import, grid, ratings, notes, tags, A/B, themes, carousel, batch undo, and compact layout.
- Both production-only and full `npm audit` checks reported zero known vulnerabilities at the time of the check.
- Git integrity inspection found a dangling tree containing an older untracked/generated inventory. This does not indicate branch corruption, but it should be cleaned up or allowed to expire through normal Git maintenance only after confirming that it contains no needed user work.
- No credentials, private keys, private image/session data, build outputs, or generated inventory files were found in the tracked source or commit history during the publication review.
- The security review found and the repository corrected one high-severity archive-extraction accounting issue. Portable-package import now caps actual decompressed bytes as well as ZIP-declared sizes, with a native regression test.
- The repository now includes an MIT License, security reporting policy, contribution guidance, continuous integration workflow, and Dependabot configuration.

### 18.2 Publication decision

**Current decision: the source repository is ready for public publication as an active pre-release development project.** It is not yet ready to claim a finished product release: the full Section 14 product acceptance audit, fresh-install exercise, signing decision, and owner evaluation remain release work. A remote is still absent, and pushing remains an external action that requires explicit owner authorization.

### 18.3 Required, ordered release-readiness work

1. **Perform the requirement-to-implementation audit for a product release.** Mark each item in Section 14 as pass, fail, or deferred with concrete evidence. Any deferred item changes the release scope and requires explicit owner approval; do not quietly label a partial implementation as complete.
2. **Prepare a signed release candidate.** Set a deliberate pre-release version, verify a fresh offline install, document signing status and known limitations, generate or verify third-party notices, and tag only after all acceptance evidence passes.
3. **Request explicit publication authorization.** Before adding `origin` or pushing, present the final commit range, target URL, repository visibility, license, release-candidate status, and any known limitations. Push only after the owner explicitly authorizes that exact external action.

### 18.4 Conditions for a safe push

A source push to the proposed GitHub repository is permitted only when all of the following are true:

- The security-focused review reports no unresolved high- or critical-severity publication issue. **Complete:** the one reported high-severity issue was fixed and tested.
- The tracked-file and Git-history review finds no secrets, private assets, user data, generated build outputs, or unwanted inventory files.
- A license and any required attribution notices are present and compatible with the dependencies and intended distribution. **Complete for source publication:** MIT License and repository policies are present; distribution notices remain a release-candidate task.
- The full native and browser/domain checks pass in an environment capable of running them; blocked tests have been rerun rather than assumed successful. **Complete for the current automated suite.**
- Section 14 has been audited, with any approved scope changes written down. **Required only before representing Copla as a finished product release.**
- The owner explicitly authorizes adding the proposed remote and pushing the reviewed commit set.

If the goal is merely private backup rather than a public release, the same secret/data/license review is still required, but installer and every end-user acceptance exercise may be scheduled after the private push. The repository visibility must still be explicitly chosen before the remote is created.

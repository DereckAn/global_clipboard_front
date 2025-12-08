# Laboratory Features Plan

Detailed implementation plan for the “Laboratory” tab with downloadable/optional features and a gated screenshot button in the header.

## Goals
- Keep the base app lean: users install/uninstall optional features on demand.
- Gate UI affordances (e.g., screenshot split-button) on feature availability + enabled state.
- Use OS-specific binaries or native APIs per platform, downloaded only when needed.
- Persist feature state per user, per machine.

## UX Outline
- Settings → Laboratory tab: list of cards similar to the provided mock.
  - Left: icon, title, description.
  - Right CTA: “Install” → “Installing {x}%” (disabled) → red trash icon for uninstall when installed.
  - Optional inline error if prerequisites missing (e.g., Wayland tools).
- Header: screenshot split-button appears only when `screenshot` feature is installed and enabled.
  - Primary: full-screen capture.
  - Dropdown: region capture.
  - Disabled state while capture in progress.

## Data Model (frontend reflection of backend state)
- `LabFeatureId = "screenshot" | "ocr" | "translator" | "pickColor" | ...`
- `LabFeatureUIState`: `{ id, status: "available" | "installed", progress?: number, error?: string, enabled: boolean, installedVersion?: string }`
- `LabFeatureMeta` (static or fetched once): `{ id, title, description, icon, needsDownload: boolean }`
- Source of truth for install/enabled lives in Rust; frontend mirrors it to render CTA/progress/error.

## Storage Locations
- Artifacts: `AppDir/lab_features/<feature>/<version>/...`
  - macOS: `~/Library/Application Support/global_clipboard/lab_features/...`
  - Windows: `%AppData%\\global_clipboard\\lab_features\\...`
  - Linux: `~/.local/share/global_clipboard/lab_features/...`
- Feature state JSON: same settings area already used by Tauri for app settings.

## Backend (Rust) Structures
- `LabFeatureId` enum.
- `Artifact { url: String, sha256: String, version: String, size_bytes: Option<u64> }`
- `FeatureRegistryItem { id: LabFeatureId, needs_download: bool, artifact: Option<Artifact> }` (artifact chosen per-OS; no platform branching in TS).
- `FeatureState { id: LabFeatureId, installed_version: Option<String>, enabled: bool }`
- Derived status: `installed` if `installed_version.is_some()`, else `available`.
- Progress kept in-memory during install and emitted via events.

## Backend Commands (Tauri)
- `get_lab_features() -> Vec<FeatureWithState>` (registry + state merged).
- `install_feature(id: LabFeatureId) -> FeatureWithState`
- `uninstall_feature(id: LabFeatureId) -> FeatureWithState`
- `enable_feature(id: LabFeatureId, enabled: bool) -> FeatureWithState`
- Events:
  - `lab://install-progress` payload `{ id, progress }` (0–100).
  - `lab://install-error` payload `{ id, message }` (optional if command returns error).

## Install Flow (Rust)
1) Validate registry entry exists and is supported on current OS.
2) Create feature dir under `AppDir/lab_features/<id>/<version>/`.
3) Download artifact to temp; stream bytes, emit progress percent.
4) Verify SHA-256; abort and delete temp on mismatch.
5) Unpack/copy into feature dir (if archive).
6) Update state: `installed_version = Some(version)`, `enabled = true` (or leave disabled if you prefer explicit toggle).
7) Return updated state; emit final progress 100.

## Uninstall Flow
1) Remove feature dir for the installed version.
2) Update state: `installed_version = None`, `enabled = false`.
3) Return updated state.

## Enable/Disable
- For features that need download: require `installed_version.is_some()` before enabling.
- For built-in (no download) features: no artifact required; just flip `enabled`.
- Persist in state JSON.

## Frontend Integration
- On settings load: call `get_lab_features()` and map to UI cards.
- CTA logic:
  - If `status === "installed"` → show red trash icon (click = uninstall).
  - Else if `progress` exists → show “Installing {progress}%” (disabled).
  - Else → show “Install”.
- Subscribe to `lab://install-progress` to update the visible percentage live.
- Show inline error if `error` present.

## Header Screenshot Gating
- Header renders the screenshot split-button only when:
  - `screenshot` feature `status === "installed"` AND `enabled === true`.
- Button states:
  - Default: active.
  - While capture in progress: disabled + spinner.
  - Dropdown entries: “Full screen (⌘+⇧+3 / Win+PrtSc)” and “Selection (⌘+⇧+4 / Snip)”.

## Screenshot Capture Implementation (per OS)
- macOS: `screencapture -x <file>` (full), `screencapture -i -x <file>` (interactive region).
- Windows: BitBlt full-screen capture to PNG; for region, use Snipping Tool (`snippingtool /clip`) or a small custom region UI. Save to temp PNG.
- Linux:
  - Wayland: `grim` for full, `grim -g "$(slurp)"` for region. If missing, return a clear error.
  - X11: `gnome-screenshot -f` or ImageMagick `import`. If missing, return error.
- After capture: run through existing `image_handler` to normalize to PNG + thumbnail, insert into DB and system clipboard so it appears in history.

## Other Features (future)
- OCR: ship/download a helper (e.g., Tesseract-based) per OS.
- Translator: likely online API; keep config/keys in settings; download only if a helper is needed.
- PickColor: small native helper or built-in OS color picker (varies by OS).

## Error Handling
- Missing tooling (e.g., `grim`): return structured error; UI shows inline message.
- Download failures: propagate error; keep state as `available`; clear partial files.
- Hash mismatch: abort, delete temp, surface “Integrity check failed”.

## Security Notes
- Always verify SHA-256; consider signature verification if distributing binaries.
- Store helpers under user-specific app data, not system-wide.
- Do not auto-run unknown binaries; only run the downloaded helper for its feature.

## Testing Checklist
- macOS: full + interactive capture flows; header gating on/off; uninstall removes button.
- Windows: full + region capture; Snipping Tool availability; uninstall cleanup.
- Linux: Wayland (grim/slurp) and X11 (gnome-screenshot/import) paths; missing-tool errors.
- Progress events: verify percent updates; UI disables during install.
- State persistence: restart app, ensure installed/enabled status survives.

## Rollout Steps
1) Add Rust registry/state structs + JSON persistence.
2) Add Tauri commands + progress events.
3) Implement screenshot feature backend (per-OS capture) using native tools first (no download).
4) Implement install/uninstall flows (no-op for built-ins; real download for future helpers).
5) Update frontend settings tab to new card list with Install/Installing/Trash CTA.
6) Gate header button on screenshot installed+enabled; add split-button behavior.
7) QA per-platform; document prerequisites (grim/slurp, gnome-screenshot, etc.).

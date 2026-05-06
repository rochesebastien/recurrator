# CLAUDE.md

Local-first, RAM-only notes spotlight. Tauri 2 + Svelte 5 + TypeScript.
No DB, no cloud, no persistent index — every search rescans loaded notes.

## Stack

- **Tauri 2.x** (MSVC toolchain on Windows). `crate-type = ["staticlib", "cdylib", "rlib"]`
  is mobile-ready by default; we only ship desktop for now.
- **Frontend**: Svelte 5 (runes mode), TypeScript strict + `verbatimModuleSyntax`, Vite 5.
- **Search core**: `memchr::memmem` for literal, `nucleo-matcher` for fuzzy.
- **Walker**: `ignore` crate with all ignore filters disabled — the notes folder is not a repo,
  hidden files and `.gitignore` rules must NOT exclude content.
- **No DB / no SQLite / no persistent index.** Notes load to RAM at startup.
- **No HTTP server in-process** until slice 7 (Chrome capture). Until then, IPC only.

## Where we are

Slices in order. Done unless flagged.

- [x] **0** — load notes, literal search via memmem, IPC plumbing, basic UI.
- [x] **1a** — highlight literal matches (UTF-16 offsets across IPC).
- [x] **1b** — fuzzy via nucleo (filename + content, name boost ×1.5), Ctrl+L toggle.
- [x] **UI redesign** — Geist + warm ambient gradient + frosted-glass palette. See DESIGN.md.
- [ ] **2** — global hotkey, tray icon, fs watcher (`notify`) with debounce.
- [ ] **3** — YAML frontmatter, filterable search (`tag:`, `code:`, `id:`).
- [ ] **4** — CodeMirror 6 editor + debounced autosave. Beware echo loop with the watcher.
- [ ] **5** — markdown-it preview (Mermaid, GFM tables, inline HTML).
- [ ] **6** — split view 2 notes.
- [ ] **7** — local HTTP endpoint for Chrome capture extension.

## Constraints

- Volume target: **500–2000 notes × ~5 KB**.
- Latency target: **< 20 ms** per search at full volume.
- Notes are **plain `.md` / `.txt`**. Never invent custom on-disk formats.
- Notes folder: **`<repo>/notes`** (resolved at compile time via
  `env!("CARGO_MANIFEST_DIR")` parent in `lib.rs`). Regenerable via
  `py generate_test_notes.py` at the repo root.

## Architecture

```
src-tauri/src/
  main.rs       — windows_subsystem flag + gdidiot_lib::run()
  lib.rs        — load store, manage State, generate_handler, run
  notes.rs      — Note { path: PathBuf, content: String }, NotesStore (RwLock)
  search.rs     — SearchMode { Literal, Fuzzy }, search() + utf16 helpers
  commands.rs   — #[tauri::command] surface (`search`)
src/
  App.svelte    — palette layout, debounce 50ms, Ctrl+L toggle, highlight()
  app.css       — design tokens + component styles (see DESIGN.md)
  lib/api.ts    — invoke wrappers + Match/SearchMode types (mirror Rust)
```

`src/lib/api.ts` is the only boundary between front and Rust commands.
Types here mirror the Rust structs field-for-field.

## Conventions

- **No comments unless they explain *why*.** Names are enough for *what*.
- **No `--no-verify`, no `--force` to main.** Fix the underlying issue.
- **No backwards-compat shims** during a slice. Just edit the code.
- **Match offsets across IPC are UTF-16** (JS strings are UTF-16). Conversion in Rust.
- **Commit messages**: `<type>(<scope>): <subject>` with a body explaining the *why*
  and any non-obvious decisions. See `git log` for the established pattern.
- **One slice = one or more focused commits.** Split sub-slices (1a/1b) when one
  half ships independently.

## Git policy

- The harness blocks direct push to `main`. Either branch (`feat/slice-N`) or the
  user pushes manually after reviewing local commits.
- Lockfiles (`Cargo.lock`, `package-lock.json`) are committed for reproducibility.

## Common commands

```pwsh
npm install                    # first time only
npm run tauri dev              # dev (cold Rust build ~5 min, then incremental)
npm run check                  # svelte-check
cd src-tauri && cargo check    # Rust-only fast check
py generate_test_notes.py      # regenerate <repo>/notes fixtures
```

## Gotchas

- **`<repo>/notes`** is gitignored — fixtures are regenerable. Don't commit the folder.
- **Icons**: `src-tauri/icons/icon.ico` is required by `tauri-build` on Windows
  (compile-time, not packaging). To replace the placeholder, swap `_source.png`
  with a real logo and rerun `npx tauri icon src-tauri/icons/_source.png`.
- **`generate_context!()` does NOT need `dist/`** in dev (uses `devUrl`). It does at bundle time.
- **`cargo` not on PATH** in some shells right after rustup install — restart the
  terminal or prepend `~/.cargo/bin` once.
- **Windows `~`** is not expanded by Rust. We use `CARGO_MANIFEST_DIR`; do not
  reintroduce `dirs::home_dir()` for the notes path.

## Don'ts

- Don't add a database, persistent index, or migration layer.
- Don't add cloud sync, login, or telemetry.
- Don't add a config file before slice 3 (frontmatter ships first).
- Don't generalize `search` behind a trait until there are 3+ modes.
- Don't introduce error-handling layers for slice-0 invariants — `unwrap()` on
  the lock is fine; mutations land at slice 2 with the watcher.

# Architecture

Time Warp Studio is a Cargo workspace of four library crates plus one binary.

```
src/main.rs
  └─ tw_ui::TimeWarpApp  (eframe::App)
       ├─ tw_core::Interpreter
       │    ├─ tw_languages::ExecContext
       │    │    └─ tw_graphics::TurtleState
       │    ├─ tw_languages::execute_basic / execute_logo / …
       │    └─ tw_core::ExecutionTimeline
       ├─ tw_ui::CodeEditor
       ├─ tw_ui::TurtleCanvas
       ├─ tw_ui::OutputPanel
       ├─ tw_ui::DebugPanel
       ├─ tw_ui::FeaturePanels
       └─ tw_ui::ThemeManager
```

---

## Crate Details

### `tw_graphics`

Pure data — no UI, no interpreter logic.

| Type | Purpose |
|------|---------|
| `TurtleState` | Full turtle machine state: position, heading, pen, lines, shapes |
| `TurtleLine` | One drawn line segment |
| `TurtleShape` | Polygon, arc, dot, or text shape |
| `ShapeType` | Enum discriminating shape kind |
| `Rgb` | `(u8, u8, u8)` colour tuple |
| `parse_color()` | Parse named / hex / palette-index colours |

All types derive `Serialize` / `Deserialize`.

---

### `tw_languages`

Contains the `ExecContext` (shared mutable state) and one executor per language.

**`ExecContext`** holds everything a language executor can read or write:

- numeric variables, string variables, arrays
- the program line list and current line cursor
- for-loop, gosub, while, do-loop stacks
- user subroutine definitions
- PILOT match state
- Pascal block stack / var-section flag
- Prolog fact and rule databases
- Forth executor (optional boxed value, popped before each call)
- `TurtleState`
- output buffer
- input request/response queues
- iteration counter + max limit (default 100 000)

**`ControlFlow`** is the return type of every executor call:

```rust
Continue | Jump(usize) | JumpLabel(String) | Gosub(usize) |
Return | End | WaitInput | Error(String)
```

**`eval.rs`** implements a shunting-yard → RPN expression evaluator used by BASIC, Logo, PILOT and Pascal.  It never calls `eval()` on a string — all evaluation is token-by-token.

---

### `tw_core`

| Type | Purpose |
|------|---------|
| `Language` | Enum of 7 supported languages + metadata |
| `Interpreter` | Wraps `ExecContext`, drives the main execution loop |
| `RunState` | `Idle / Running / WaitingInput / Finished / Error` |
| `ExecutionTimeline` | Records `ExecutionFrame` snapshots for the time-travel debugger |
| `ExecutionFrame` | One step's: line number, source, all variable values, output-so-far |

**Execution loop** (`Interpreter::step_batch`):

1. Get current `(line_no, source_line)` from `ctx.program_lines[ctx.line_idx]`.
2. Optionally record a debugger snapshot.
3. Dispatch to the language executor.
4. Apply the returned `ControlFlow` to advance `line_idx`.
5. Return early on `WaitInput`, `End`, or `Error`.

The loop runs up to `batch_size` (default 200) statements per call, so the egui frame stays responsive while long programs execute across multiple frames.

---

### `tw_ui`

Built with [egui 0.31](https://github.com/emilk/egui) / [eframe 0.31](https://github.com/emilk/egui).  Immediate-mode: the entire UI is re-evaluated every frame.

| Module | Responsibility |
|--------|----------------|
| `app.rs` | `TimeWarpApp` (eframe `App` impl), layout, toolbar, menus, keyboard shortcuts |
| `editor.rs` | `CodeEditor` — wraps `TextEdit::multiline` with font control |
| `canvas.rs` | `TurtleCanvas` — renders `TurtleState` via `egui::Painter` (zoom/pan) |
| `output_panel.rs` | `OutputPanel` — scrolling text console |
| `debug_panel.rs` | `DebugPanel` — timeline slider, variable grid, breakpoint list |
| `feature_panels.rs` | `FeaturePanels` — lessons, examples browser, theme picker, about |
| `themes.rs` | `Theme` struct + `ThemeManager` with 8 built-in colour themes |

**Layout modes** (selectable via View menu):

- *Horizontal split* — editor top, canvas + output bottom
- *Vertical split* — editor left, canvas + output right
- *Editor only*
- *Canvas only*

---

## Data Flow

```
User types source code
       │
  CodeEditor (stores String)
       │ Run pressed
  Interpreter::load() — parses lines, builds label map
       │
  eframe frame timer calls
  Interpreter::step_batch() ×N per frame
       │ dispatches to
  execute_basic / execute_logo / … (operate on ExecContext)
       │
  ExecContext.output ──────────────────► OutputPanel
  ExecContext.turtle ──────────────────► TurtleCanvas (renders each frame)
  ExecutionTimeline  ──────────────────► DebugPanel
```

---

## Threading

The application is single-threaded.  The execution loop runs on the UI thread, batched across frames.  This is intentional — turtle graphics and variable state must be observable frame-by-frame without synchronization overhead.

---

## Adding a Dependency

All shared dependencies are declared once in `[workspace.dependencies]` in the root `Cargo.toml` and referenced with `{ workspace = true }` in each crate.  Do not add crate-specific version pins unless absolutely necessary.

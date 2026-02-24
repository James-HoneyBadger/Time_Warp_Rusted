# Architecture

Technical overview of the Time Warp Rusted codebase.

---

## Workspace Structure

```
Time_Warp_Rusted/
├── Cargo.toml              # Workspace manifest
├── src/
│   └── main.rs             # Binary entry point
├── crates/
│   ├── tw_core/            # Language interpreters & runtime
│   ├── tw_ui/              # GUI (egui/eframe)
│   ├── tw_graphics/        # Turtle graphics engine
│   ├── tw_languages/       # Parsers, ASTs, evaluators
│   └── tw_iot/             # Raspberry Pi / GPIO / serial
├── Examples/               # 74+ example programs (7 languages)
├── docs/                   # Documentation
└── target/                 # Build output (git-ignored)
```

---

## Crate Dependency Graph

```
src/main.rs (binary)
  └── tw_ui
        ├── tw_core
        │     ├── tw_languages
        │     └── tw_graphics
        └── tw_iot
```

### tw_languages — Parsers & Evaluators

Contains parsers and execution engines for all seven languages:

| Module | Language | Approach |
|--------|----------|----------|
| `basic.rs` | BASIC | Line-by-line interpreter with line numbers |
| `logo.rs` | Logo | Recursive-descent parser + tree walker |
| `c_lang.rs` | C (subset) | Statement-level interpreter |
| `pascal.rs` | Pascal | Statement-level interpreter |
| `forth.rs` | Forth | Stack machine with word dictionary |
| `pilot.rs` | PILOT | Command-per-line interpreter |
| `prolog.rs` | Prolog | Unification engine with backtracking |

Shared infrastructure:

- `expression.rs` — Common expression evaluator (arithmetic, comparison,
  functions, string operations) used by BASIC, C, Pascal, and PILOT.
- Each language module exposes `parse()` and `step()` (or equivalent)
  functions consumed by `tw_core`.

### tw_core — Runtime & Interpreter

The central orchestrator:

- **`interpreter.rs`** — `Interpreter` struct with `RunState` enum
  (`Idle`, `Running`, `WaitingInput`, `Finished`, `Error`). Manages
  `load()`, `parse_lines()`, `step()`, execute-batch loop (batch size 200
  statements per frame), and GPIO integration.
- **`language.rs`** — `Language` enum with seven variants, provides
  `friendly_name()`, `extension()`, `from_extension()`, `all()`,
  `sample_program()`.
- **`context.rs`** — `ExecutionContext` shared between interpreters,
  carrying output buffer, error log, turtle command queue, variable state.
- **Timeline** — Execution timeline for time-travel debugging (step
  forward/backward through program states).

### tw_graphics — Turtle Engine

- **`turtle.rs`** — `Turtle` struct: position (x, y), heading, pen state
  (up/down), pen color, pen width, visibility. Processes movement commands
  and produces line segments.
- **`canvas.rs`** — Rendering primitives: lines, circles, arcs, dots,
  text. Maintains a draw list that the UI canvas renders.
- **`color.rs`** — 60+ named colors, CGA palette (0–15), hex parsing,
  RGB Color struct.

### tw_ui — User Interface

Built with **egui 0.31** / **eframe 0.31** (immediate-mode GUI):

- **`app.rs`** — `TimeWarpApp` (implements `eframe::App`). The main
  application struct holding all state: interpreter, theme manager, editor,
  output panel, canvas, feature panels, debug panel, active tab, input
  buffer, current language/file, IoT panel state. Contains:
  - Menu bar rendering (File, Edit, View, Appearance, Language, Raspberry Pi)
  - Keyboard shortcut handling (25+ shortcuts)
  - Left panel and tab switching logic
  - Program execution loop (run/stop/step)

- **`editor.rs`** — `CodeEditor` struct: code buffer, cursor, selection,
  undo/redo stacks (max 200 entries), clipboard, find/replace, goto line,
  auto-indent, line numbers, word wrap, font size/family. Renders the code
  editing area with syntax-aware toolbar.

- **`themes.rs`** — `ThemeManager` with 20 themes in 3 categories
  (`ThemeCategory`: Retro, Dark, Light). Each `Theme` struct has ~30 color
  fields. `FONT_SIZE_PRESETS` (6 sizes). `themes_by_category()` organizer.

- **`output_panel.rs`** — `OutputPanel` with three tabs:
  `OutputTab::Output`, `OutputTab::Errors`, `OutputTab::Console`.

- **`canvas.rs`** — `TurtleCanvas`: zoom/pan viewport, renders turtle
  draw list, reset view.

- **`feature_panels.rs`** — Left sidebar `FeaturePanels` with four panels:
  `Lessons`, `Examples`, `Themes`, `About`. `builtin_examples()` provides
  categorized example programs for each language.

### tw_iot — IoT & GPIO

- **`gpio.rs`** — `GpioManager`: pin state tracking with `PinMode`
  (Input, Output, Pwm, I2c, Spi, Unset) and `PinState` (High, Low,
  Unknown). Methods: `connect`, `pin_mode`, `digital_write`/`read`,
  `pwm_write`, `analog_read`, `sim_toggle`, `reset`.
- **`board.rs`** — `Board` enum: Pico, PicoW, PiZero, PiZero2W, Pi4,
  Pi5, Simulator. Each board reports `gpio_count`, `has_wifi`, `has_adc`.
- **`serial_port.rs`** — `SerialConnection`: port name, baud rate
  (default 115200), connect/disconnect, send/receive, available ports.

### Binary (src/main.rs)

Entry point: initializes `eframe::NativeOptions`, creates
`TimeWarpApp::default()`, launches the native window.

---

## Data Flow

### Program Execution

```
User clicks Run (F5)
  → app.rs: interpreter.load(source, language)
  → tw_core: parse source into AST / line table
  → app.rs: main loop calls interpreter.step() in batches (200/frame)
    → tw_languages: execute statement, produce output/turtle commands
    → tw_core: context accumulates output, errors, draw commands
  → tw_ui: output_panel displays text, canvas renders turtle graphics
```

### Input Handling

```
Interpreter reaches INPUT/scanf/ReadLn/A:
  → RunState changes to WaitingInput
  → UI shows input field (docked or floating)
  → User types and presses Enter
  → Input passed to interpreter, RunState returns to Running
```

### Theme Application

```
User selects theme from Appearance menu
  → ThemeManager.set_theme(name)
  → All UI components read colors from active theme each frame
  → Immediate visual update (no restart)
```

---

## Key Design Decisions

1. **Immediate-mode GUI (egui)** — The entire UI is redrawn every frame.
   No widget state persistence or event callbacks — just Rust structs
   rendered directly.

2. **Batched execution** — Interpreters execute 200 statements per frame
   to keep the UI responsive. Long-running programs yield to the render
   loop.

3. **Shared expression evaluator** — BASIC, C, Pascal, and PILOT all
   share the same expression parser and evaluator, ensuring consistent
   math behavior.

4. **Language-agnostic turtle** — Turtle graphics commands are
   language-independent; each language translates its drawing commands
   into a common `TurtleCommand` enum.

5. **Simulated GPIO** — The `Simulator` board allows GPIO code to run
   without hardware, with `sim_toggle` for interactive testing.

6. **Undo/redo with limits** — Editor maintains separate undo and redo
   stacks capped at 200 entries to bound memory usage.

---

## Build System

- **Cargo workspace** with 5 member crates
- **Rust edition**: 2021
- **Key dependencies**:
  - `eframe` 0.31 / `egui` 0.31 — GUI framework
  - `rfd` — Native file dialogs
  - `serde` + `serde_json` — Serialization
  - `rand` — Random number generation
  - `serialport` — Serial communication (IoT)

Build: `cargo build --release`
Test: `cargo test` (239 integration tests)
Run: `cargo run`

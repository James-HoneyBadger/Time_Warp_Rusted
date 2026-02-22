# Time Warp Studio

**Educational Multi-Language Programming Environment — Rust Edition**

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange)](https://www.rust-lang.org/)
[![egui 0.31](https://img.shields.io/badge/egui-0.31-blue)](https://github.com/emilk/egui)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-7.0.0-green)](#)

Time Warp Studio is a native desktop IDE for learning programming across **7 classic languages** — BASIC, PILOT, Logo, C, Pascal, Prolog, and Forth — in a single unified environment. It features an integrated code editor, turtle graphics canvas, step-through debugger with timeline replay, and built-in lessons.

Built entirely in Rust using [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe).

---

## Features

- **7 language executors** running directly in-process — no compiler or runtime required
- **Turtle graphics** canvas with zoom, pan, and OpenGL rendering
- **Step debugger** with variable inspector and timeline scrubbing
- **8 built-in themes** — Dracula, Monokai, VS Code Dark, Solarized Dark, Ocean Blue, Spring, Sunset, Candy
- **Examples browser** with 30+ ready-to-run programs
- **Lessons panel** for each language
- Immediate-mode UI — zero widget toolkit dependencies beyond egui

---

## Quick Start

```bash
# Debug build (~5 s, ~156 MB)
cargo run

# Release build (~60 s, ~10 MB stripped)
cargo build --release
./target/release/time-warp-studio
```

**Minimum Rust:** 1.80 (stable).  No other system dependencies required on Fedora/RHEL.

On Debian/Ubuntu you may need:

```bash
sudo apt install libxkbcommon-dev libwayland-dev libgl1-mesa-dev
```

---

## Building

```bash
git clone https://github.com/James-HoneyBadger/Time_Warp_Studio
cd Time_Warp_Studio
cargo build --release
```

---

## Workspace Layout

```
Cargo.toml                  workspace root + binary
src/
  main.rs                   entry point (eframe::run_native)
crates/
  tw_graphics/              TurtleState, TurtleLine, TurtleShape, colour utils
  tw_languages/             all 7 language executors + ExecContext + expression evaluator
  tw_core/                  Interpreter loop, Language enum, ExecutionTimeline debugger
  tw_ui/                    egui App, editor, canvas, themes, panels
Examples/                   .bas / .c / .logo / .pas / .pro / .fth / .pilot programs
docs/                       documentation
```

---

## Languages

| Language | Extension | Sample |
|----------|-----------|--------|
| BASIC    | `.bas`    | `PRINT "Hello"` / turtle graphics |
| PILOT    | `.pilot`  | `T:Hello` / `A:NAME` / `M:pattern` |
| Logo     | `.logo`   | `REPEAT 4 [FD 100 RT 90]` |
| C        | `.c`      | `printf("Hello\n");` |
| Pascal   | `.pas`    | `writeln('Hello');` |
| Prolog   | `.pro`    | `?- ancestor(tom, ann).` |
| Forth    | `.fth`    | `: SQUARE DUP * ; 5 SQUARE .` |

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| F5 | Run program |
| F6 | Stop |
| F7 | Step (one statement) |
| Ctrl+L | Clear output |
| Ctrl+= | Zoom in editor |
| Ctrl+- | Zoom out editor |

---

## Documentation

- [Architecture](ARCHITECTURE.md)
- [User Guide](docs/USER_GUIDE.md)
- [Language Guide](docs/LANGUAGE_GUIDE.md)
- [Turtle Graphics](docs/TURTLE_GRAPHICS.md)
- [Debugger](docs/DEBUGGING.md)
- [FAQ](docs/FAQ.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)

---

## License

MIT — see [LICENSE](LICENSE).

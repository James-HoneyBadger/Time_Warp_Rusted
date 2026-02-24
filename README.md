# ⏱ Time Warp Studio

**An educational multi-language programming environment built in Rust.**

Time Warp Studio is a retro-inspired IDE that lets you write, run, and debug
programs in seven classic programming languages — all with built-in turtle
graphics, a step-through debugger, and Raspberry Pi GPIO support.

![Version](https://img.shields.io/badge/version-1.0.0-blue)
![Rust](https://img.shields.io/badge/rust-1.80%2B-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![Languages](https://img.shields.io/badge/languages-7-purple)
![Themes](https://img.shields.io/badge/themes-20-teal)

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| **7 Language Executors** | BASIC, Logo, C, Pascal, Forth, PILOT, Prolog |
| **Turtle Graphics** | Full 2D drawing canvas with zoom, pan, colors, fills |
| **Step Debugger** | Time-travel debugger with execution timeline and variable inspector |
| **20 Themes** | 6 retro + 10 dark + 4 light — with color-swatch previews |
| **Tabbed Interface** | Code Editor, Text Output, and Graphics Canvas on separate tabs |
| **74+ Example Programs** | Tutorials, demos, and showcases for every language |
| **Raspberry Pi GPIO** | Simulate and control real GPIO pins on Pico, Zero, Pi 4/5 |
| **Code Editor** | Syntax highlighting, find/replace, undo/redo, line numbers, 7 fonts |

## 🖥 Supported Languages

| Language | Extension | Example |
|----------|-----------|---------|
| **Turbo BASIC** | `.bas` | `PRINT "Hello, World!"` |
| **Logo** | `.logo` | `REPEAT 4 [FD 100 RT 90]` |
| **C (subset)** | `.c` | `printf("Hello!\n");` |
| **Pascal** | `.pas` | `WriteLn('Hello!');` |
| **Forth** | `.f` | `." Hello!" CR` |
| **PILOT** | `.pilot` | `T:Hello, World!` |
| **Prolog** | `.pl` | `?- write('Hello!'), nl.` |

## 🚀 Quick Start

```bash
# Clone
git clone https://github.com/James-HoneyBadger/Time_Warp_Rusted.git
cd Time_Warp_Rusted

# Run (debug build)
cargo run

# Or build optimized release
cargo build --release
./target/release/time-warp-studio
```

### System Requirements

- **Rust** 1.80 or newer
- **Linux** (Ubuntu/Debian): `sudo apt install libxkbcommon-dev libwayland-dev libgl1-mesa-dev`
- **Fedora**: `sudo dnf install libxkbcommon-devel wayland-devel mesa-libGL-devel`
- **macOS / Windows**: No extra dependencies

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [Installation Guide](docs/INSTALL.md) | Setup, building, platform notes |
| [User Guide](docs/USER_GUIDE.md) | Complete IDE walkthrough |
| [Language Reference](docs/LANGUAGE_GUIDE.md) | Syntax and commands for all 7 languages |
| [Turtle Graphics](docs/TURTLE_GRAPHICS.md) | Drawing commands, coordinates, colors |
| [Keyboard Shortcuts](docs/KEYBOARD_SHORTCUTS.md) | Every shortcut at a glance |
| [Themes & Appearance](docs/THEMES.md) | All 20 themes, fonts, customization |
| [Debugging Guide](docs/DEBUGGING.md) | Step debugger, timeline, breakpoints |
| [Raspberry Pi & IoT](docs/RASPBERRY_PI.md) | GPIO, boards, wiring, projects |
| [Examples Guide](docs/EXAMPLES.md) | Walkthrough of all 74+ example programs |
| [Architecture](ARCHITECTURE.md) | Codebase structure and design |
| [FAQ & Troubleshooting](docs/FAQ.md) | Common questions and solutions |
| [Contributing](CONTRIBUTING.md) | How to add languages, themes, and features |

## 🎨 Themes

**20 built-in themes** in three categories:

| 🕹 Retro | 🌙 Dark | ☀ Light |
|----------|---------|---------|
| Amber Terminal | Dracula | Spring |
| Green Screen | Monokai | Candy |
| Commodore 64 | VS Code Dark | Solarized Light |
| Borland Turbo | Solarized Dark | GitHub Light |
| CGA | Ocean Blue | |
| Apple ][ | Sunset | |
| | Nord | |
| | Gruvbox Dark | |
| | Tokyo Night | |
| | One Dark | |

Select themes from the **Appearance** menu with live color-swatch previews.

## ⌨ Key Shortcuts

| Shortcut | Action |
|----------|--------|
| `F5` / `Ctrl+R` | Run program |
| `F6` | Stop |
| `F7` | Step (single line) |
| `Ctrl+1` / `2` / `3` | Switch to Editor / Output / Canvas |
| `Ctrl+=` / `Ctrl+-` | Zoom in / out |
| `Ctrl+N` / `O` / `S` | New / Open / Save |
| `Ctrl+Z` / `Shift+Z` | Undo / Redo |
| `Ctrl+F` / `H` | Find / Find & Replace |
| `Ctrl+G` | Go to line |
| `Ctrl+L` | Clear output |

## 🏗 Project Structure

```
Time_Warp_Rusted/
├── src/main.rs              # Application entry point
├── crates/
│   ├── tw_graphics/         # TurtleState, shapes, colors
│   ├── tw_languages/        # Language executors + evaluator
│   ├── tw_core/             # Interpreter, timeline, Language enum
│   ├── tw_ui/               # egui IDE (editor, canvas, panels, themes)
│   └── tw_iot/              # GPIO, boards, serial
├── docs/                    # Full documentation suite
├── Examples/                # 74+ example programs
│   ├── basic/  logo/  c/  pascal/  forth/  pilot/  prolog/
│   ├── demo/                # Showcase per language
│   └── projects/            # 31 Raspberry Pi GPIO projects
└── Cargo.toml               # Workspace root
```

## 📄 License

[MIT](LICENSE)

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on adding languages,
themes, examples, and features. All contributions welcome!

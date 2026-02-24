# Installation Guide

Complete instructions for building and running Time Warp Studio on every
supported platform.

---

## Prerequisites

| Requirement | Version | Notes |
|-------------|---------|-------|
| **Rust** | 1.80+ (stable) | Install via [rustup.rs](https://rustup.rs/) |
| **Git** | any | To clone the repository |
| **C compiler** | gcc / clang | Needed by some Rust crates for linking |

### Platform-Specific Libraries

Time Warp Studio uses **egui / eframe** with OpenGL (glow) rendering.
Some platforms require extra system libraries:

#### Linux — Ubuntu / Debian

```bash
sudo apt update
sudo apt install -y \
    libxkbcommon-dev \
    libwayland-dev \
    libgl1-mesa-dev \
    build-essential
```

#### Linux — Fedora / RHEL

```bash
sudo dnf install -y \
    libxkbcommon-devel \
    wayland-devel \
    mesa-libGL-devel \
    gcc
```

#### Linux — Arch

```bash
sudo pacman -S libxkbcommon wayland mesa base-devel
```

#### macOS

No extra libraries needed. Xcode Command Line Tools are sufficient:

```bash
xcode-select --install
```

#### Windows

No extra libraries needed. Visual Studio Build Tools (C++ workload) are
required for the MSVC toolchain.

---

## Building from Source

### 1. Clone the Repository

```bash
git clone https://github.com/James-HoneyBadger/Time_Warp_Rusted.git
cd Time_Warp_Rusted
```

### 2. Debug Build (fast compile, larger binary)

```bash
cargo run
```

This compiles in ~5–15 seconds and produces a ~160 MB debug binary.
The application launches immediately after compilation.

### 3. Release Build (optimized, smaller binary)

```bash
cargo build --release
./target/release/time-warp-studio
```

Release builds take ~60 seconds but produce a ~10 MB binary with
full optimizations (LTO enabled, single codegen unit).

### 4. Run Tests

```bash
# Run all 239+ integration tests
cargo test -p tw_core

# Run just the language integration tests
cargo test --package tw_core --test language_tests

# Run tests for a specific language
cargo test --package tw_core --test language_tests -- basic_
cargo test --package tw_core --test language_tests -- logo_
cargo test --package tw_core --test language_tests -- pascal_
```

---

## Workspace Structure

Time Warp Studio is organized as a Cargo workspace with six crates:

```
Cargo.toml                     ← workspace root + binary entry point
src/main.rs                    ← eframe::run_native() launch

crates/
  tw_graphics/                 ← turtle state, shapes, color parsing
  tw_languages/                ← 7 language executors + expression evaluator
  tw_core/                     ← interpreter loop, Language enum, debugger timeline
  tw_ui/                       ← egui IDE: editor, canvas, panels, themes
  tw_iot/                      ← GPIO manager, board definitions, serial port
```

### Dependency Graph

```
tw_graphics    (no deps)
    ↓
tw_languages   (depends on tw_graphics)
    ↓
tw_core        (depends on tw_languages, tw_graphics)
    ↓
tw_ui          (depends on tw_core, tw_iot)
    ↓
binary         (depends on tw_ui, eframe, egui)
```

---

## Configuration

### Window Size

The default window is 1400 × 900 pixels (minimum 800 × 600). This is
configured in `src/main.rs`:

```rust
.with_inner_size([1400.0, 900.0])
.with_min_inner_size([800.0, 600.0])
```

### Logging

Time Warp Studio uses `env_logger`. Enable debug logging with:

```bash
RUST_LOG=debug cargo run
```

Log levels: `error`, `warn`, `info`, `debug`, `trace`.

---

## Troubleshooting

### "error: linker `cc` not found"

Install a C compiler:
- Ubuntu: `sudo apt install build-essential`
- Fedora: `sudo dnf install gcc`

### "error: could not find native static library `GL`"

Install OpenGL development libraries (see platform instructions above).

### "error[E0658]: use of unstable library feature"

Update Rust: `rustup update stable`

### Large binary size in debug mode

This is normal — debug builds include symbols and debug info. Use
`cargo build --release` for an optimized binary.

### Push rejected due to large files

Add `target/` to `.gitignore` (it should already be there). If build
artifacts were accidentally committed:

```bash
echo "target/" >> .gitignore
git rm -r --cached target/
git commit -m "Remove target from tracking"
```

# Contributing to Time Warp Studio

Thank you for your interest in contributing to this Rust project.

## Prerequisites

- Rust 1.80+ (stable toolchain)
- `cargo` (included with Rust)
- On Debian/Ubuntu: `libxkbcommon-dev libwayland-dev libgl1-mesa-dev`

## Development Setup

```bash
git clone https://github.com/James-HoneyBadger/Time_Warp_Studio
cd Time_Warp_Studio
cargo build
cargo run
```

## Project Structure

Each concern lives in its own crate:

| Crate | Responsibility |
|-------|----------------|
| `tw_graphics` | Pure turtle-state data structures, no UI |
| `tw_languages` | Language executors + `ExecContext` |
| `tw_core` | `Interpreter` execution loop, `Language` enum, `ExecutionTimeline` |
| `tw_ui` | egui widgets, the main `App`, themes, panels |

Keep crate dependencies flowing in one direction: `tw_graphics` → `tw_languages` → `tw_core` → `tw_ui` → binary.

## Making Changes

### Adding a new Language executor

1. Add a new module in `crates/tw_languages/src/<name>.rs`.
2. Expose a public `execute_<name>(ctx: &mut ExecContext, line: &str) -> ControlFlow` function.
3. Declare the module in `crates/tw_languages/src/lib.rs`.
4. Add the variant to the `Language` enum in `crates/tw_core/src/language.rs`.
5. Add dispatch to `Interpreter::dispatch()` in `crates/tw_core/src/interpreter.rs`.
6. Add example programs under `Examples/<name>/`.

### Adding a Theme

Add a named constructor in `crates/tw_ui/src/themes.rs` returning a `Theme` struct, then register it in `ThemeManager::default()`.

### Adding Example Programs

Drop source files in `Examples/<language>/` using the correct extension.  The examples browser in `feature_panels.rs` reads from `builtin_examples()` — add an entry there to surface it in the UI.

## Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` and fix all lints.
- Keep `unsafe` code to zero.
- Prefer `f64` for turtle/math and `f32` only where egui APIs require it.

## Testing

```bash
cargo test
```

Unit tests live alongside their modules.  Integration tests go in `crates/<name>/tests/`.

## Pull Requests

- Fork the repository and create a feature branch.
- Ensure `cargo build`, `cargo test`, and `cargo clippy` all pass.
- Describe *what* changed and *why* in the PR description.
- One logical change per PR.

## Commit Messages

Use the conventional-commits format:

```
feat(languages): add Smalltalk executor
fix(canvas): correct arc start-angle calculation
docs: rewrite user guide for Rust edition
```

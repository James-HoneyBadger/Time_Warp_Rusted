# Contributing to Time Warp Rusted

*Release: v1.0.0 (February 2026)*

Thank you for your interest in contributing! This guide covers everything
you need to get started.

---

## Quick Start

```bash
# Clone the repository
git clone https://github.com/James-HoneyBadger/Time_Warp_Rusted.git
cd Time_Warp_Rusted

# Install system dependencies (Ubuntu/Debian)
sudo apt install libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev \
  libxcb-xfixes0-dev libxkbcommon-dev libssl-dev

# Build and run
cargo build
cargo run

# Run tests
cargo test
```

See [docs/INSTALL.md](docs/INSTALL.md) for other platforms.

---

## Project Structure

```
crates/
├── tw_languages/   # Parsers & evaluators for all 7 languages
├── tw_core/        # Interpreter runtime, language enum, context
├── tw_graphics/    # Turtle graphics engine, colors
├── tw_ui/          # egui/eframe GUI (app, editor, themes, panels)
└── tw_iot/         # GPIO, board definitions, serial communication
src/
└── main.rs         # Binary entry point
Examples/           # 74+ example programs
docs/               # Documentation
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed technical documentation.

---

## Development Workflow

### 1. Pick an Issue or Feature

- Check the issue tracker for open items
- For new features, open an issue first to discuss the approach
- Small fixes can go directly to a pull request

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 3. Make Changes

- Follow the existing code style
- Add tests for new functionality
- Update documentation if behavior changes

### 4. Test

```bash
# Run all 239 integration tests
cargo test

# Run a specific test
cargo test test_name

# Check for compiler warnings
cargo clippy

# Format code
cargo fmt
```

### 5. Commit and Push

```bash
git add -A
git commit -m "Brief description of changes"
git push origin feature/your-feature-name
```

### 6. Open a Pull Request

- Describe what changed and why
- Reference any related issues
- Ensure CI passes

---

## Code Style

- **Rust edition**: 2021
- **Formatting**: Run `cargo fmt` before committing
- **Linting**: Run `cargo clippy` and address warnings
- **Naming**: Follow Rust conventions (snake_case for functions/variables,
  CamelCase for types)
- **Comments**: Document public APIs with `///` doc comments
- **Error handling**: Use `Result<T, E>` for fallible operations; avoid
  `unwrap()` in library code

---

## Adding a New Language

1. Create a new module in `crates/tw_languages/src/` (e.g., `lua.rs`)
2. Implement a parser that produces an AST or line table
3. Implement a step function for incremental execution
4. Add a variant to the `Language` enum in `crates/tw_core/src/language.rs`
5. Register the language in the interpreter dispatch in
   `crates/tw_core/src/interpreter.rs`
6. Add a sample program via `sample_program()` in the `Language` enum
7. Create example programs in `Examples/your_language/`
8. Add integration tests in `crates/tw_languages/src/tests/`
9. Update documentation

---

## Adding a New Theme

1. Open `crates/tw_ui/src/themes.rs`
2. Add a new `theme_your_name() -> Theme` function defining all ~30 color
   fields
3. Register it in the appropriate category (Retro, Dark, or Light) in the
   `ThemeManager` initialization
4. The theme will automatically appear in the Appearance menu and Themes
   panel

---

## Writing Tests

Integration tests live in `crates/tw_languages/src/tests/`. Each language
has its own test file:

```rust
#[test]
fn test_basic_hello_world() {
    let source = r#"PRINT "Hello, World!""#;
    let output = run_basic(source);
    assert_eq!(output.trim(), "Hello, World!");
}
```

Test categories:
- **Output tests** — verify printed output
- **Error tests** — verify error detection and messages
- **Graphics tests** — verify turtle commands are generated
- **Expression tests** — verify math evaluation

---

## Writing Examples

When adding examples to `Examples/`:

- Follow the sequential numbering: `01_`, `02_`, etc.
- Include comments explaining each concept
- Start simple and build incrementally
- End each tutorial series with a showcase file
- Test every example runs without errors

---

## Writing Documentation

Documentation lives in `docs/` (guides) and root (README, ARCHITECTURE,
CONTRIBUTING):

- Use Markdown with clear headings
- Include code examples for every feature
- Keep tables for command references
- Update the doc index in README.md when adding new docs
- Cross-reference related documents with relative links

---

## Reporting Bugs

When filing a bug report, include:

1. **What you expected** to happen
2. **What actually happened** (include error messages)
3. **Steps to reproduce** (minimal code example preferred)
4. **Platform** (OS, Rust version, build mode)
5. **Theme and language** being used

---

## Code of Conduct

This project follows a [Code of Conduct](CODE_OF_CONDUCT.md). Please be
respectful and constructive in all interactions.

---

## License

By contributing, you agree that your contributions will be licensed under
the same license as the project (see [LICENSE](LICENSE)).

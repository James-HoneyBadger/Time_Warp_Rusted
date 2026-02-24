# FAQ & Troubleshooting

Answers to common questions and solutions to typical problems.

---

## General

### What is Time Warp Rusted?

Time Warp Rusted is a retro-inspired programming IDE written in Rust. It
supports seven classic languages — BASIC, Logo, C, Pascal, Forth, PILOT,
and Prolog — with a built-in editor, turtle graphics canvas, 20 visual
themes, and Raspberry Pi GPIO simulation.

### What platforms does it run on?

Linux, macOS, and Windows. See the [Installation Guide](INSTALL.md) for
platform-specific build instructions.

### Is it an emulator?

No. Time Warp Rusted is a native desktop application with custom
interpreters for each language. It recreates the *feel* of classic
programming environments but is not emulating historical hardware or
software.

### Can I use it for teaching?

Absolutely — that is its primary design goal. The progressive example
suite (74+ programs), built-in lessons panel, retro themes, and simple
GPIO simulation make it ideal for education.

---

## Languages

### Which BASIC dialect is this?

It is closest to **Turbo BASIC** / **QBasic** — supporting structured
programming with `SUB`/`FUNCTION`, `SELECT CASE`, `DO`/`LOOP`,
`WHILE`/`WEND`, multi-line `IF`/`ELSE`/`END IF`, and string functions
like `LEFT$`, `MID$`, `INSTR`. Line numbers are optional.

### Does the C interpreter support the full C standard?

No. It implements a teaching subset: variables, `printf`/`scanf`,
`if`/`else`, `while`/`for`/`do-while`/`switch`, functions with
recursion, and basic math. Pointers, structs, dynamic memory, and the
preprocessor are not supported.

### Does Logo support all standard Logo commands?

It supports most common commands: turtle movement, pen control, color,
`REPEAT`/`FOREVER`/`IF`/`IFELSE`, procedures with parameters, variables,
recursion, `ARC`, `DOT`, `LABEL`, `BEGINFILL`/`ENDFILL`, and GPIO.
Advanced features like object Logo, macros, and threading are not
available.

### How does the Prolog interpreter work?

It implements core Prolog: facts, rules, queries with unification and
backtracking, arithmetic (`is`), list operations (`member`, `append`,
`length`, `findall`), and type-checking predicates. Maximum recursion
depth is enforced.

### Can I add my own language?

There is no plugin system yet. Adding a language requires implementing a
parser and evaluator in Rust and registering it in the `Language` enum.
See `crates/tw_languages/src/` for examples.

---

## Editor

### How do I change the font?

**Appearance → Font Family** in the menu bar, or use the font dropdown
in the editor toolbar.

### How do I change font size?

**Appearance → Font Size** in the menu bar, **Ctrl + =** / **Ctrl + −**,
or the +/− buttons in the editor toolbar. Six presets are available:
Tiny (10pt) through Huge (24pt).

### Is there syntax highlighting?

The editor provides keyword-aware coloring through themes. Each theme
defines colors for keywords, strings, comments, and numbers.

### Can I have multiple files open?

The editor has a tabbed interface — you can work on multiple files
simultaneously. Unsaved changes are indicated on tabs.

### How big is the undo history?

The undo stack holds up to **200 actions**. A separate redo stack (also
200 entries) tracks undone changes.

---

## Execution

### My program runs but I see no output

Make sure you are on the **Output** tab (Ctrl + 2). If your program uses
turtle graphics, switch to the **Canvas** tab (Ctrl + 3).

### How do I provide input to my program?

When a program reaches an input statement (`INPUT`, `scanf`, `ReadLn`,
`A:`, etc.), the interpreter enters **WaitingInput** state and an input
field appears. Type your response and press **Enter**.

The input field can be docked to the bottom of the output panel or shown
as a floating dialog — toggle with **View → Dock Input to Output**.

### My program seems stuck / runs forever

Press **F6** (Stop) to halt execution. Check for infinite loops in your
code. Prolog programs have a recursion depth limit to prevent genuine
infinite recursion.

### What does "batch size 200" mean?

The interpreter executes up to 200 statements per UI frame to keep the
interface responsive. Long-running programs continue across multiple
frames. This is invisible to the user — it just prevents the UI from
freezing.

---

## Turtle Graphics

### The canvas is blank after running my program

1. Make sure the program actually uses turtle/drawing commands.
2. Switch to the **Canvas** tab (Ctrl + 3).
3. Try **zooming out** (scroll wheel) — your drawing may be very large
   or very small.
4. Click **Reset View** to re-center.

### The drawing is too small / too large

Use the **scroll wheel** to zoom in or out. Click and drag to pan.
Click **Reset View** to return to the default viewport.

### Can I save my drawing?

Canvas export is not currently supported. You can take a screenshot of
the application window.

---

## Themes

### How do I switch themes?

**Appearance** menu → pick a category (🕹 Retro, 🌙 Dark, ☀ Light) →
click a theme. Or use the **Themes** panel in the left sidebar.

### Do themes affect the output and canvas?

Yes. Each theme defines colors for the editor, output panel, canvas
background, menus, buttons, and tabs. Switching themes changes the
entire application appearance.

### Can I create custom themes?

Not through the UI. Custom themes require adding a theme function in
`crates/tw_ui/src/themes.rs` and registering it in the theme manager.

---

## GPIO / Raspberry Pi

### Do I need a Raspberry Pi to use GPIO features?

No. The **Simulator** board lets you test GPIO code without any hardware.
Pin states are tracked in software and visible in the IoT panel.

### How do I connect to a real board?

1. Open the **🔌 Raspberry Pi** menu
2. Select **Setup Board** and choose your board model
3. Connect via serial port (the app detects available ports)
4. Use GPIO commands in your program

### Which boards are supported?

Raspberry Pi Pico, Pico W, Pi Zero, Pi Zero 2W, Pi 4, Pi 5, and a
software Simulator.

---

## Building & Development

### Build fails with missing system libraries

Install the required graphics libraries for your platform:

- **Ubuntu/Debian**: `sudo apt install libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`
- **Fedora**: `sudo dnf install gtk3-devel libxcb-devel libxkbcommon-devel openssl-devel`
- **Arch**: `sudo pacman -S gtk3 libxcb libxkbcommon openssl`
- **macOS**: `xcode-select --install` (Xcode command line tools)

See [INSTALL.md](INSTALL.md) for complete instructions.

### Tests are failing

Run `cargo test` and check error messages. All 239 integration tests
should pass on a clean build. If tests fail after your changes, review
the test expectations in `crates/tw_languages/src/tests/`.

### How do I run in release mode?

```bash
cargo run --release
```

This enables optimizations and is noticeably faster.

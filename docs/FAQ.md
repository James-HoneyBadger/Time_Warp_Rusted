# Frequently Asked Questions

## Build & Install

**Q: What Rust version do I need?**  
Rust 1.80 stable or newer.  Check with `rustc --version`.

**Q: Does it need any system libraries?**  
On Fedora/RHEL: none beyond the default Rust toolchain.  
On Debian/Ubuntu:
```bash
sudo apt install libxkbcommon-dev libwayland-dev libgl1-mesa-dev
```

**Q: How do I build a release binary?**  
```bash
cargo build --release
# binary is at target/release/time-warp-studio
```

**Q: The window doesn't open. What's wrong?**  
Ensure you have a display server running (X11 or Wayland).  Try `RUST_LOG=debug cargo run` to see startup logs.

---

## Usage

**Q: How do I switch languages?**  
Use the language drop-down in the toolbar, or **Language** menu.

**Q: My BASIC program gives wrong line numbers.**  
Line numbers in BASIC must be numeric prefixes on each line (`10 PRINT ...`).  Free-format (unnumbered) BASIC is also supported — in that case, execution is sequential.

**Q: Why does Logo `REPEAT` end at a `]` on the same line?**  
Logo blocks are inline square-bracket lists.  The entire `REPEAT n [...]` must be on one line.  For multi-statement bodies, separate commands with spaces inside the brackets: `REPEAT 4 [FD 100 RT 90]`.

**Q: Prolog doesn't find my fact. What's wrong?**  
Every clause must end with a `.` period, even on the last line.  Multi-line clauses are accumulated until the period is seen.

**Q: Forth words I define aren't persisting between runs.**  
The Forth executor is recreated fresh for each `Run`.  Put all `: definitions ;` at the top of your program.

**Q: My program runs forever and the UI freezes.**  
The interpreter has a `max_iterations` limit of 100 000 by default.  It automatically stops at that point.  Press **⏹ Stop** at any time to halt immediately.

---

## Turtle Graphics

**Q: The canvas is blank after running my program.**  
Make sure `PENDOWN` (or `PD`) is called before movement commands.  The turtle starts with pen down, but `CLEARSCREEN` resets it.

**Q: How do I reset the canvas without restarting?**  
Call `HOME` (returns turtle to origin) or `CLEARSCREEN` (clears and homes) from your program, or click **🗑 Clear** in the toolbar.

**Q: Can I save the canvas as an image?**  
Not yet — planned for a future release.

---

## Debugger

**Q: The debugger panel is empty.**  
Make sure the **Debug** checkbox is checked *before* pressing Run.  The timeline is only populated during a debug run.

**Q: Step backward shows old variable values — is that correct?**  
Yes.  The variable inspector shows the exact state *at that recorded step*, which is the intended time-travel behaviour.

---

## Contributing

**Q: How do I add a new language?**  
See [CONTRIBUTING.md](../CONTRIBUTING.md) — there's a step-by-step guide for adding a new language executor.

**Q: Where are the example programs stored?**  
Under `Examples/<language>/`.  Add `.bas` / `.c` / `.logo` / `.pas` / `.pro` / `.fth` / `.pilot` files there, then register them in `crates/tw_ui/src/feature_panels.rs` → `builtin_examples()`.

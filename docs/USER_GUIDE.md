# User Guide

## Starting the Application

```bash
cargo run --release
```

The IDE opens with a BASIC "Hello World" program loaded.

---

## Interface Overview

```
┌─────────────────────────────────────────────────────┐
│  Menu bar                                           │
│  Toolbar: [Language ▾] [▶ Run] [⏭ Step] [⏹ Stop]  │
├──────────────┬──────────────────────────────────────┤
│              │                                      │
│  Left Panel  │  Code Editor                        │
│  (Lessons /  │                                      │
│   Examples / ├──────────────┬───────────────────────┤
│   Themes /   │              │                       │
│   About)     │  Canvas      │  Output               │
│              │              │                       │
│  [Debugger]  │              │                       │
└──────────────┴──────────────┴───────────────────────┘
```

---

## Running a Program

1. Select a language from the **Language** drop-down in the toolbar, or via the **Language** menu.
2. Type (or paste) your program in the editor.
3. Press **▶ Run** or **F5**.
4. Output appears in the Output panel; turtle graphics appear in the Canvas.
5. Press **⏹ Stop** or **F6** to halt a running program.

## Input

When a program requests user input (e.g. BASIC `INPUT`, PILOT `A:`, Pascal `readln`), an input bar appears at the bottom of the window. Type your response and press **Enter** or **Submit**.

---

## Layout Modes

Choose **View → Horizontal split / Vertical split / Editor only / Canvas only**.

---

## Themes

Choose **View → [theme name]** or open the **Themes** tab in the left panel.

Eight built-in themes:

| Theme | Style |
|-------|-------|
| Dracula | Dark purple |
| Monokai | Dark warm |
| VS Code Dark | Dark neutral |
| Solarized Dark | Dark teal |
| Ocean Blue | Dark blue |
| Spring | Light green |
| Sunset | Dark amber |
| Candy | Light pink |

---

## Loading Examples

Open the **Examples** tab in the left panel.  Use the filter box to search by language or title.  Click any example to load and immediately run it.

---

## The Canvas

The turtle graphics canvas supports:

- **Scroll wheel** — zoom in/out
- **Click + drag** — pan
- **🔁 Reset view** button in toolbar — return to 1:1 centre

---

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| F5 | Run |
| F6 | Stop |
| F7 | Step one statement |
| Ctrl+L | Clear output |
| Ctrl+= | Editor font larger |
| Ctrl+- | Editor font smaller |

---

## Debugger

Enable the debugger by checking **Debug** in the toolbar before pressing Run.

The debugger panel (in the left panel, below the feature tabs) shows:

- A **step slider** to scrub through the execution history
- **⏮ ⏪ ⏩ ⏭** navigation buttons
- The **source line** at the current step
- A **variable table** showing all values at that step

To set a breakpoint, right-click a line number (breakpoints are toggled via `ExecutionTimeline::toggle_breakpoint(line_no)`).

See [DEBUGGING.md](DEBUGGING.md) for details.

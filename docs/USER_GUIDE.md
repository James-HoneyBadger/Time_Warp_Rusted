# User Guide

A complete walkthrough of the Time Warp Rusted interface and workflow.

---

## Table of Contents

1. [Launching the IDE](#launching-the-ide)
2. [Interface Overview](#interface-overview)
3. [Writing Code](#writing-code)
4. [Running Programs](#running-programs)
5. [Tabs: Editor, Output, Canvas](#tabs)
6. [File Management](#file-management)
7. [Left Panel](#left-panel)
8. [Menus](#menus)
9. [Input Handling](#input-handling)
10. [Canvas Interaction](#canvas-interaction)
11. [Themes and Fonts](#themes-and-fonts)
12. [Keyboard Shortcuts](#keyboard-shortcuts)

---

## Launching the IDE

```bash
cargo run            # debug build
# or
cargo build --release && ./target/release/time-warp-studio
```

The window opens at 1400 × 900 pixels with the title **⏱ Time Warp Rusted**.

---

## Interface Overview

```
┌──────────────────────────────────────────────────────────┐
│  Menu Bar:  File  Edit  View  Appearance  Language  ...  │
├──────────────────────────────────────────────────────────┤
│  Toolbar:  [Language ▼] [▶ Run] [⏹ Stop] [⏭ Step] ...  │
├──────────┬───────────────────────────────────────────────┤
│          │  Tab Bar:  📝 Editor │ 📄 Output │ 🖼 Canvas  │
│  Left    ├───────────────────────────────────────────────┤
│  Panel   │                                               │
│          │              Active Tab Content                │
│ Lessons  │                                               │
│ Examples │    (Code Editor / Text Output / Graphics)      │
│ Themes   │                                               │
│ About    │                                               │
├──────────┴───────────────────────────────────────────────┤
│  Status Bar:  Language │ File │ Line:Col │ Run State      │
├──────────────────────────────────────────────────────────┤
│  Input Bar:  ⌨ Input: [____________] [Send]               │
└──────────────────────────────────────────────────────────┘
```

### Key Areas

| Area | Purpose |
|------|---------|
| **Menu Bar** | File operations, edit commands, view toggles, appearance, language selection |
| **Toolbar** | Quick-access buttons: language selector, Run/Stop/Step, zoom, reset |
| **Tab Bar** | Switch between the three main content areas |
| **Left Panel** | Lessons, examples browser, theme selector, about info |
| **Editor Tab** | Full code editor with syntax highlighting and line numbers |
| **Output Tab** | Text output, error messages, and console log |
| **Canvas Tab** | Turtle graphics rendering area |
| **Status Bar** | Current language, filename, cursor position, run state |
| **Input Bar** | Appears when a program requests user input |

---

## Writing Code

### The Code Editor

The editor is a full-featured code editing area with:

- **Line numbers** — toggle on/off in the editor toolbar
- **Syntax highlighting** — keywords, strings, numbers, comments highlighted per language
- **Find & Replace** — `Ctrl+F` opens the search bar; `Ctrl+H` opens search + replace
- **Go to Line** — `Ctrl+G` opens the line number input
- **Undo / Redo** — `Ctrl+Z` / `Ctrl+Shift+Z` (up to 200 steps)
- **Word wrap** — toggle in the editor toolbar
- **Auto-indent** — toggle in the editor toolbar
- **Comment toggle** — 💬 button in the editor toolbar
- **Indent / Unindent** — → and ← buttons in the editor toolbar

### Editor Toolbar

The toolbar strip directly above the editing area provides:

```
[↩ Undo] [↪ Redo]  |  [✂ Cut] [📋 Copy] [📌 Paste]  |
[🔍 Find] [↕ Goto]  |  [💬 Comment] [→ Indent] [← Unindent]  |
Font: [−] 14pt [+] [Monospace ▼]  |  ☑ Lines  ☑ Wrap  ☑ Indent  Tab: [4 ▼]
```

### Changing Font

- Use the **font family** dropdown in the editor toolbar (7 monospace options)
- Use **−** / **+** buttons for fine-grained size control
- Use `Ctrl+=` / `Ctrl+-` keyboard shortcuts
- Or use the **Appearance** menu for named presets (Tiny 10pt → Huge 24pt)

---

## Running Programs

### Basic Workflow

1. **Select a language** from the toolbar dropdown or the Language menu
2. **Write or load** a program
3. **Press F5** (or click ▶ Run) to execute
4. View results in the **Output** tab (text) or **Canvas** tab (graphics)
5. **Press F6** (or click ⏹ Stop) to halt execution

### Run States

| State | Indicator | Meaning |
|-------|-----------|---------|
| Idle | (no indicator) | No program running |
| Running | 🟢 ● Running | Program is executing |
| Input | 🟡 ● Input… | Waiting for user input |
| Finished | 🔵 ✔ Done | Program completed successfully |
| Error | 🔴 ✗ (message) | Runtime error occurred |

### Step Execution

- **F7** executes one statement at a time
- Enable **Debug mode** (View menu) for the full timeline debugger
- The debugger records every statement, showing variables at each step

---

## Tabs

Time Warp Rusted uses a tabbed interface with three main content areas.

### 📝 Code Editor (Ctrl+1)

The source code editing area. This is where you write programs.

### 📄 Text Output (Ctrl+2)

Text output from your program (PRINT, printf, writeln, etc.). Has three
sub-tabs:

| Sub-tab | Content |
|---------|---------|
| **Output** | Standard program output |
| **Errors** | Error messages and warnings |
| **Console** | Full input/output transcript |

### 🖼 Graphics Canvas (Ctrl+3)

The turtle graphics rendering area. All drawing commands (FORWARD, LINE,
CIRCLE, ARC, DOT, etc.) are displayed here.

### Tab Switching

- Click the tab labels in the tab bar
- Use `Ctrl+1`, `Ctrl+2`, `Ctrl+3` keyboard shortcuts
- The tab bar shows activity indicators:
  - 🟡 dot on **Output** when new text appears
  - 🟡 dot on **Canvas** when new graphics are drawn

### Auto-Switch Behavior

When you run a program:
- If it produces **text output**, the Output tab activates automatically
- If it produces **graphics**, the Canvas tab activates automatically

---

## File Management

### New File — `Ctrl+N`

Creates a new empty file. If you have unsaved changes, you'll be prompted
to save first.

### Open File — `Ctrl+O`

Opens a file dialog to load a source file. The language is automatically
detected from the file extension:

| Extensions | Language |
|------------|----------|
| `.bas`, `.basic` | BASIC |
| `.logo` | Logo |
| `.c`, `.h`, `.cpp`, `.hpp` | C |
| `.pas`, `.pascal`, `.pp`, `.dpr` | Pascal |
| `.f`, `.fth`, `.4th`, `.fs`, `.forth` | Forth |
| `.pilot`, `.pil` | PILOT |
| `.pl`, `.pro`, `.prolog` | Prolog |

### Save — `Ctrl+S`

Saves the current file. If no filename is set, opens a Save As dialog.

### Save As — `Ctrl+Shift+S`

Always opens a dialog to choose a filename and location.

---

## Left Panel

Toggle with **View → Left panel**. Contains four tabs:

### 📚 Lessons

Interactive lesson outlines organized by language. Each lesson includes:
- A description of the concept
- A "▶ Load starter program" button that populates the editor

**Lesson topics per language:**
- **BASIC**: Hello World, Variables, Input, Loops, Graphics
- **Logo**: Moving, Squares, Procedures, Colors, Recursion
- **C**: Hello World, Variables, Loops, Functions
- **Pascal**: Hello World, Variables, Control Flow, Procedures
- **Forth**: Stack Basics, Arithmetic, Word Definitions, Loops
- **PILOT**: Hello World, Input/Match, Jumps, Subroutines
- **Prolog**: Facts, Rules, Queries, Recursion

### 📁 Examples

A searchable catalog of 74+ built-in example programs. Type in the filter
box to search by name. Click any example to load it into the editor.

### 🎨 Themes

Quick-access theme browser (also available via the Appearance menu).

### ℹ About

Version information, credits, and links.

---

## Menus

### File Menu

| Command | Shortcut | Action |
|---------|----------|--------|
| 📄 New | `Ctrl+N` | New empty file |
| 📂 Open… | `Ctrl+O` | Open file dialog |
| 💾 Save | `Ctrl+S` | Save current file |
| 💾 Save As… | `Ctrl+Shift+S` | Save with new name |
| Quit | — | Exit application |

### Edit Menu

| Command | Shortcut | Action |
|---------|----------|--------|
| Undo | `Ctrl+Z` | Undo last edit |
| Redo | `Ctrl+Shift+Z` | Redo last undo |
| Find | `Ctrl+F` | Open find bar |
| Replace | `Ctrl+H` | Open find & replace |
| Go to Line | `Ctrl+G` | Jump to line number |

### View Menu

| Command | Action |
|---------|--------|
| Left panel | Toggle the left sidebar |
| Debug mode | Enable/disable step debugger |
| Layout Preferences… | Open layout settings |
| Input bar docked | Dock input at bottom vs. floating window |
| IoT Panel | Toggle Raspberry Pi GPIO panel |
| Active Tab | Switch between Editor / Output / Canvas |

### Appearance Menu

| Section | Options |
|---------|---------|
| 🎨 Theme | 20 themes in 3 categories with color swatches |
| 🔤 Font Family | Monospace, Hack, Fira Code, JetBrains Mono, Source Code Pro, Consolas, Courier New |
| 🔠 Font Size | Presets (Tiny 10pt – Huge 24pt) + fine −/+ control |

### Language Menu

Select between BASIC, Logo, C, Pascal, Forth, PILOT, and Prolog.
Changing the language updates syntax highlighting and the executor.

### 🔌 Raspberry Pi Menu

| Option | Action |
|--------|--------|
| 📋 GPIO Projects | Browse 31 pre-built GPIO projects |
| ⚙ Pi Setup Guide | Setup instructions for each board |
| 🔌 IoT Panel | Open the GPIO simulator/controller |

---

## Input Handling

When a program requires input (`INPUT` in BASIC, `A:` in PILOT,
`scanf` in C, `readln` in Pascal), the run state changes to **Input**.

### Docked Input (default)

An input bar appears at the bottom of the IDE:
```
⌨ Input: [type here___________] [Send]
```

Type your response and press **Enter** or click **Send**.

### Floating Input

Uncheck **View → Input bar docked** to use a floating input window instead.

---

## Canvas Interaction

The graphics canvas supports:

| Action | Effect |
|--------|--------|
| **Scroll wheel** | Zoom in/out (0.05× to 20×) |
| **Click + drag** | Pan the canvas |
| **🔁 Reset view** | Reset zoom to 1× and center the view |

### Grid

At high zoom levels (>3×), a grid automatically appears to help with
precise positioning.

### Coordinate System

- Origin (0, 0) is at the **center** of the canvas
- **X** increases to the right
- **Y** increases upward
- Heading 0° = North, 90° = East, 180° = South, 270° = West

---

## Themes and Fonts

See the dedicated [Themes & Appearance Guide](THEMES.md) for full details.

Quick summary:
- **20 themes** organized into Retro (6), Dark (10), and Light (4) categories
- Open the **Appearance** menu to browse themes with color swatches
- Theme descriptions appear on hover
- **7 monospace fonts** available in the font family selector
- **6 font size presets** plus manual +/− control

---

## Keyboard Shortcuts

See the dedicated [Keyboard Shortcuts Reference](KEYBOARD_SHORTCUTS.md)
for the complete list.

### Most-Used Shortcuts

| Shortcut | Action |
|----------|--------|
| `F5` | Run program |
| `F6` | Stop |
| `F7` | Step |
| `Ctrl+1` / `2` / `3` | Switch tab |
| `Ctrl+S` | Save |
| `Ctrl+Z` | Undo |
| `Ctrl+F` | Find |
| `Ctrl+=` / `-` | Zoom in/out |
| `Escape` | Close search/goto bars |

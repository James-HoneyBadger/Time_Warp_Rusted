# Debugger Guide

Time Warp Studio includes a **time-travel debugger** that records every execution step and lets you scrub backwards and forwards through the history.

---

## Enabling Debug Mode

Check the **Debug** checkbox in the toolbar before pressing **▶ Run**.

When debug mode is active, the interpreter records an `ExecutionFrame` at each statement:

- Current line number and source text
- Snapshot of all numeric variables, string variables, and arrays
- Accumulated output up to that point

---

## Debugger Panel

The debugger panel appears in the left sidebar below the feature tabs.

```
⏱ Debugger
────────────────────────
Step: [────●──────] 42 / 150
[⏮] [⏪] [⏩] [⏭]
────────────────────────
Line 30: FOR I = 1 TO 10
────────────────────────
Variables
I     │ 3
TOTAL │ 6
NAME$ │ "Alice"
ARR[] │ [1, 2, 4, 8, …]
────────────────────────
Breakpoints
🔴 Line 50   [✕]
🔴 Line 80   [✕]
```

### Navigation

| Button | Action |
|--------|--------|
| ⏮ | Jump to step 0 |
| ⏪ | Step backward |
| ⏩ | Step forward |
| ⏭ | Jump to last step |
| Slider | Scrub to any step |

### Variable Inspector

The variable table shows the state **at the selected step**, not the current live state.  There are three types of entries:

| Display | Type |
|---------|------|
| `I = 3` | Numeric variable |
| `NAME$ = "Alice"` | String variable |
| `ARR[] = [1, 2, …]` | Array (first 8 elements shown) |

---

## Single Stepping

Press **F7** (or **⏭ Step** in the toolbar) to execute exactly one statement at a time.  The debug checkbox does not need to be enabled for single-stepping; the debugger panel will show variables if it is.

---

## Breakpoints

Breakpoints are set programmatically via `ExecutionTimeline::toggle_breakpoint(line_number)`.  When the interpreter reaches a breakpointed line it pauses (entering `WaitingInput` state) after completing the previous batch.  Resume by pressing **▶ Run** again.

To remove a breakpoint click the **✕** button next to it in the panel.

---

## Performance Note

Recording every frame has a cost proportional to the number of variables.  For programs with very tight inner loops, disable debug mode and use single-stepping or strategic breakpoints instead.

---

## Limitations

- The timeline is held in memory; very long programs (>100 000 steps) will stop recording due to the `max_iterations` limit in `ExecContext`.
- Variable snapshots do not include Prolog databases or the Forth dictionary (those are shown as-is in the live inspector but are not time-travelled).

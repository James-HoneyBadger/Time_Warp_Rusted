# Debugging Guide

How to debug programs and troubleshoot issues in Time Warp Rusted.

---

## Built-in Debugging Tools

### Step Execution

Press **F7** to execute your program one statement at a time. Each press
advances by a single line or instruction. The current line is highlighted
in the editor.

### Run / Stop

| Action | Shortcut | Description |
|--------|----------|-------------|
| **Run** | F5 | Execute the entire program |
| **Stop** | F6 | Halt execution immediately |
| **Step** | F7 | Execute one statement |

### Output Panel

The output panel has three tabs:

| Tab | Shows |
|-----|-------|
| **Output** | Program `PRINT`/`printf`/`WriteLn` output |
| **Errors** | Compile and runtime error messages |
| **Console** | Interpreter diagnostics and debug info |

Switch between tabs or check **Errors** first when a program misbehaves.

### Debug Mode

Enable debug mode from **View → Debug Panel**. This shows:

- Current interpreter state (`Idle`, `Running`, `WaitingInput`,
  `Finished`, `Error`)
- Current line number and statement being executed
- Variable values (when available)
- Execution timeline position

---

## Common Errors by Language

### BASIC

| Error | Cause | Fix |
|-------|-------|-----|
| "Undefined variable" | Using a variable before assigning it | Add `LET X = 0` before use |
| "NEXT without FOR" | Mismatched loop | Check FOR/NEXT pairing |
| "Type mismatch" | String where number expected | Check `$` suffix on variables |
| "RETURN without GOSUB" | Stray RETURN | Ensure GOSUB precedes RETURN |
| "END IF without IF" | Unmatched block | Check IF/END IF pairing |

### Logo

| Error | Cause | Fix |
|-------|-------|-----|
| "Unknown command" | Typo in command name | Check spelling: `FD`, `RT`, etc. |
| "Not enough inputs" | Missing argument | Provide required parameter |
| "Undefined procedure" | Calling before defining | Define with `TO`/`END` first |
| "Too many inputs" | Extra arguments | Check command syntax |

### C

| Error | Cause | Fix |
|-------|-------|-----|
| "Undefined function" | Calling unknown function | Define function before calling |
| "Type error" | Wrong argument type | Check printf format specifiers |
| "Missing semicolon" | Forgotten `;` | Add semicolons after statements |
| "Unmatched brace" | Missing `{` or `}` | Check brace pairs |

### Pascal

| Error | Cause | Fix |
|-------|-------|-----|
| "Undeclared identifier" | Missing `var` declaration | Declare in `var` section |
| "Type mismatch" | Incompatible assignment | Check types match |
| "Expected semicolon" | Missing `;` | Add between statements |
| "Expected period" | Missing final `.` | Add `.` after final `end` |

### Forth

| Error | Cause | Fix |
|-------|-------|-----|
| "Stack underflow" | Popping from empty stack | Ensure enough values pushed |
| "Unknown word" | Typo or undefined word | Check spelling, define with `:` |
| "Division by zero" | Dividing by 0 | Check divisor before `/` |

### PILOT

| Error | Cause | Fix |
|-------|-------|-----|
| "Unknown command" | Invalid command letter | Use T:, A:, M:, etc. |
| "Label not found" | Jump to undefined label | Define `*LABEL` in code |
| "Missing colon" | Command without `:` | Format as `T:text` |

### Prolog

| Error | Cause | Fix |
|-------|-------|-----|
| "Syntax error" | Missing period | End every clause with `.` |
| "Undefined predicate" | Missing fact or rule | Define before querying |
| "Max recursion depth" | Infinite recursion | Add base case to rules |

---

## Debugging Strategies

### 1. Read the Error Message

Error messages appear in the **Errors** tab of the output panel. They
usually include:
- The line number where the error occurred
- A description of what went wrong
- Sometimes a suggestion for fixing it

### 2. Use Step Execution

Press **F7** repeatedly to walk through your program line by line. Watch
the output and variable states change. This is the most effective way to
find logic bugs.

### 3. Add Print Statements

Insert `PRINT` (or equivalent) statements to show variable values at key
points:

```basic
PRINT "DEBUG: X = "; X
PRINT "DEBUG: entering loop"
```

Remove or comment them out once the bug is found.

### 4. Simplify the Problem

If a large program has a bug:
1. Comment out sections until the bug disappears
2. The last section you commented out contains the bug
3. Focus debugging on that section

### 5. Check Common Pitfalls

- **Off-by-one errors**: `FOR I = 1 TO 10` vs `FOR I = 0 TO 9`
- **Missing initialization**: Variables default to 0 but may need
  specific starting values
- **Infinite loops**: Always ensure loop conditions will eventually
  become false
- **Case sensitivity**: BASIC and Logo are case-insensitive; C and
  Pascal are case-sensitive for identifiers

---

## GPIO Debugging

When working with GPIO code:

1. **Use the Simulator first** — verify logic before connecting hardware
2. **Check the IoT panel log** — every GPIO operation is recorded
3. **Verify pin modes** — always call `PINMODE` before read/write
4. **Check pin numbers** — valid range depends on your board model
5. **Watch for timing** — use `SLEEP`/`WAIT` between rapid pin changes

---

## Performance Issues

### Program runs slowly

- The interpreter executes 200 statements per UI frame — this is normal
  for long-running programs
- Tight loops with no output may appear frozen but are still running
- Complex turtle graphics with thousands of segments may slow canvas
  rendering

### UI is laggy

- Close the debug panel if not needed
- Reduce font size for faster text rendering
- Stop any running program (F6) before making edits

---

## Getting Help

If you are stuck:

1. Check the [FAQ](FAQ.md) for common questions
2. Review the [Language Reference](LANGUAGE_GUIDE.md) for correct syntax
3. Look at similar [example programs](EXAMPLES.md) for working patterns
4. Open an issue on GitHub with a minimal reproducing example

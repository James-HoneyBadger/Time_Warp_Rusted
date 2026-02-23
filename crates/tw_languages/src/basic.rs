//! BASIC language executor — port of `languages/basic.py`.
//!
//! Supports Turbo BASIC with graphics extensions.
//! Entry point: `execute_basic(ctx, command)`.

use crate::context::{ControlFlow, ExecContext};
use tw_graphics::turtle::parse_color;

// ── entry point ───────────────────────────────────────────────────────────────

/// Execute a single BASIC statement.  Returns `ControlFlow` indicating what
/// the interpreter loop should do next.
pub fn execute_basic(ctx: &mut ExecContext, command: &str) -> ControlFlow {
    let command = command.trim();
    if command.is_empty() {
        return ControlFlow::Continue;
    }

    let cmd_up = command.to_uppercase();
    let cmd_up_str = cmd_up.as_str();

    // ── Multi-line IF block tracking ──────────────────────────────────────
    // Structural keywords (IF/ELSE/END IF/ELSEIF) must always be processed
    // so the block stack stays balanced, even inside skipped regions.
    // Each stack entry = (currently_executing, any_branch_taken_in_this_chain).
    let skipping = !ctx.if_block_stack.is_empty()
        && !ctx.if_block_stack.iter().all(|(exec, _)| *exec);

    if cmd_up_str == "END IF" || cmd_up_str == "ENDIF" {
        ctx.if_block_stack.pop();
        return ControlFlow::Continue;
    }
    if cmd_up_str == "ELSE" || cmd_up_str.starts_with("ELSE ") || cmd_up_str.starts_with("ELSEIF") {
        // Only act if the parent (all stack entries below the top) is executing.
        let parent_ok = ctx.if_block_stack.len() <= 1
            || ctx.if_block_stack[..ctx.if_block_stack.len() - 1].iter().all(|(exec, _)| *exec);

        // Handle ELSEIF / ELSE IF
        if cmd_up_str.starts_with("ELSE IF ")
            || cmd_up_str.starts_with("ELSEIF ")
        {
            let any_taken = ctx.if_block_stack.last().map_or(false, |e| e.1);
            ctx.if_block_stack.pop();
            if any_taken {
                // A previous branch already executed — skip this ELSEIF
                ctx.if_block_stack.push((false, true));
            } else {
                // No branch taken yet — evaluate the new condition
                let if_part = if cmd_up_str.starts_with("ELSEIF ") {
                    &command[7..]
                } else {
                    &command[8..]
                };
                return basic_if_block(ctx, if_part.trim());
            }
            return ControlFlow::Continue;
        }

        // Plain ELSE
        if parent_ok {
            if let Some(top) = ctx.if_block_stack.last_mut() {
                if !top.1 {
                    // No branch has executed yet — execute ELSE
                    top.0 = true;
                    top.1 = true;
                } else {
                    // A branch already ran — skip ELSE
                    top.0 = false;
                }
            }
        }
        return ControlFlow::Continue;
    }

    // If we're inside a skipped block, only count nested IFs for depth tracking
    if skipping {
        if cmd_up_str.starts_with("IF ") && is_block_if(&cmd_up) {
            ctx.if_block_stack.push((false, false)); // nested skip
        }
        return ControlFlow::Continue;
    }

    // ── Normal execution (not skipping) ──────────────────────────────────

    // Ignore label-only lines (e.g. `100:` or `MYLOOP:`)
    if command.ends_with(':') && !command.contains(' ') {
        return ControlFlow::Continue;
    }

    // Handle multi-statement lines joined by `:`
    if has_multi_statement(command) {
        return exec_multi(ctx, command);
    }

    // ── Comments ──────────────────────────────────────────────────────────
    // Re-alias for the rest of the function (formerly a local `let cmd_up`).
    let cmd_up = cmd_up_str;

    if cmd_up.starts_with("REM") || cmd_up.starts_with("'") {
        return ControlFlow::Continue;
    }

    // ── PRINT ─────────────────────────────────────────────────────────────
    if cmd_up.starts_with("PRINT") {
        let args = if command.len() > 5 { &command[5..].trim_start() } else { "" };
        return basic_print(ctx, args);
    }

    // ── INPUT ─────────────────────────────────────────────────────────────
    if cmd_up.starts_with("INPUT ") {
        return basic_input(ctx, command[6..].trim());
    }

    // ── IF ────────────────────────────────────────────────────────────────
    if cmd_up.starts_with("IF ") {
        let rest = command[3..].trim();
        // Check for multi-line block IF (nothing after THEN)
        if is_block_if(&cmd_up) {
            return basic_if_block(ctx, rest);
        }
        return basic_if(ctx, rest);
    }

    // ── GOTO / GOSUB / RETURN ────────────────────────────────────────────
    if cmd_up.starts_with("GOTO ") {
        let target = command[5..].trim();
        return resolve_jump(ctx, target);
    }
    if cmd_up.starts_with("GOSUB ") {
        let target = command[6..].trim();
        return basic_gosub(ctx, target);
    }
    if cmd_up == "RETURN" {
        return basic_return(ctx);
    }

    // ── END ───────────────────────────────────────────────────────────────
    // Pure END (program termination) — do not match END IF / END SUB / etc.
    if cmd_up == "END" {
        return ControlFlow::End;
    }

    // ── LET / assignment ──────────────────────────────────────────────────
    if cmd_up.starts_with("LET ") {
        return basic_let(ctx, command[4..].trim());
    }

    // ── FOR / NEXT ────────────────────────────────────────────────────────
    if cmd_up.starts_with("FOR ") {
        return basic_for(ctx, command[4..].trim());
    }
    if cmd_up.starts_with("NEXT") {
        let rest = command[4..].trim();
        let var = if rest.is_empty() { None } else { Some(rest) };
        return basic_next(ctx, var);
    }

    // ── WHILE / WEND ──────────────────────────────────────────────────────
    if cmd_up.starts_with("WHILE ") {
        return basic_while(ctx, command[6..].trim());
    }
    if cmd_up == "WEND" {
        return basic_wend(ctx);
    }

    // ── DO / LOOP ─────────────────────────────────────────────────────────
    if cmd_up == "DO" || cmd_up.starts_with("DO ") {
        ctx.do_stack.push(ctx.line_idx);
        return ControlFlow::Continue;
    }
    if cmd_up.starts_with("LOOP") {
        return basic_loop(ctx, command[4..].trim());
    }

    // ── SELECT CASE ───────────────────────────────────────────────────────
    if cmd_up.starts_with("SELECT ") {
        let expr = command[6..].trim();
        let expr = expr.strip_prefix("CASE").unwrap_or(expr).trim();
        let val = ctx.eval_f64(expr);
        ctx.select_val = Some(val);
        ctx.in_select = true;
        ctx.select_matched = false;
        return ControlFlow::Continue;
    }
    if cmd_up.starts_with("CASE ") {
        return basic_case(ctx, command[5..].trim());
    }
    if cmd_up == "END SELECT" {
        ctx.in_select = false;
        ctx.select_val = None;
        return ControlFlow::Continue;
    }

    // ── SUB / FUNCTION ────────────────────────────────────────────────────
    if cmd_up.starts_with("SUB ") {
        return basic_sub(ctx, command[4..].trim());
    }
    if cmd_up == "END SUB" {
        return basic_end_sub(ctx);
    }
    if cmd_up.starts_with("FUNCTION ") {
        return basic_sub(ctx, command[9..].trim());
    }
    if cmd_up == "END FUNCTION" {
        return basic_end_sub(ctx);
    }
    if cmd_up.starts_with("CALL ") {
        return basic_call(ctx, command[5..].trim());
    }

    // ── DIM ───────────────────────────────────────────────────────────────
    if cmd_up.starts_with("DIM ") {
        return basic_dim(ctx, command[4..].trim());
    }

    // ── CLS ───────────────────────────────────────────────────────────────
    if cmd_up == "CLS" {
        ctx.turtle.clear_screen();
        ctx.text_lines.clear();
        ctx.emit("🎨 Screen cleared\n");
        return ControlFlow::Continue;
    }

    // ── SCREEN ────────────────────────────────────────────────────────────
    if cmd_up.starts_with("SCREEN ") {
        // Accept screen mode changes as a no-op in Rust edition
        return ControlFlow::Continue;
    }

    // ── LOCATE ────────────────────────────────────────────────────────────
    if cmd_up.starts_with("LOCATE ") {
        return ControlFlow::Continue; // no-op in graphical mode
    }

    // ── COLOR ─────────────────────────────────────────────────────────────
    if cmd_up.starts_with("COLOR ") {
        return basic_color(ctx, command[6..].trim());
    }

    // ── WIDTH ─────────────────────────────────────────────────────────────
    if cmd_up.starts_with("WIDTH ") {
        if let Ok(w) = command[6..].trim().parse::<f64>() {
            ctx.turtle.set_pen_width(w);
        }
        return ControlFlow::Continue;
    }

    // ── Graphics ─────────────────────────────────────────────────────────
    if cmd_up.starts_with("LINE ") {
        return basic_line(ctx, command[5..].trim());
    }
    if cmd_up.starts_with("CIRCLE ") {
        return basic_circle(ctx, command[7..].trim());
    }
    if cmd_up.starts_with("PSET ") {
        return basic_pset(ctx, command[5..].trim());
    }
    if cmd_up.starts_with("DRAW ") {
        return basic_draw(ctx, command[5..].trim());
    }
    if cmd_up.starts_with("PAINT ") {
        return ControlFlow::Continue; // flood-fill stub
    }

    // ── Turtle commands ───────────────────────────────────────────────────
    if cmd_up.starts_with("FORWARD ") || cmd_up.starts_with("FD ") {
        let dist = ctx.eval_f64(after_first_word(command));
        ctx.turtle.forward(dist);
        return ControlFlow::Continue;
    }
    if cmd_up.starts_with("BACKWARD ") || cmd_up.starts_with("BK ") {
        let dist = ctx.eval_f64(after_first_word(command));
        ctx.turtle.backward(dist);
        return ControlFlow::Continue;
    }
    if cmd_up.starts_with("LEFT ") || cmd_up.starts_with("LT ") {
        let angle = ctx.eval_f64(after_first_word(command));
        ctx.turtle.left(angle);
        return ControlFlow::Continue;
    }
    if cmd_up.starts_with("RIGHT ") || cmd_up.starts_with("RT ") {
        let angle = ctx.eval_f64(after_first_word(command));
        ctx.turtle.right(angle);
        return ControlFlow::Continue;
    }
    if cmd_up == "PENUP" || cmd_up == "PU" {
        ctx.turtle.pen_up();
        return ControlFlow::Continue;
    }
    if cmd_up == "PENDOWN" || cmd_up == "PD" {
        ctx.turtle.pen_down_cmd();
        return ControlFlow::Continue;
    }
    if cmd_up == "HOME" {
        ctx.turtle.home();
        return ControlFlow::Continue;
    }
    if cmd_up == "CLEARSCREEN" || cmd_up == "CS" {
        ctx.turtle.clear_screen();
        return ControlFlow::Continue;
    }

    // ── SLEEP / WAIT ──────────────────────────────────────────────────────
    if cmd_up.starts_with("SLEEP") || cmd_up.starts_with("WAIT") {
        return ControlFlow::Continue; // no-op; we don't block the UI thread
    }

    // ── RANDOMIZE ─────────────────────────────────────────────────────────
    // RANDOMIZE [TIMER] — seed the RNG.  Our RNG is already time-seeded,
    // so this is a no-op.
    if cmd_up.starts_with("RANDOMIZE") {
        return ControlFlow::Continue;
    }

    // ── SOUND / BEEP ─────────────────────────────────────────────────────
    if cmd_up.starts_with("SOUND") || cmd_up == "BEEP" {
        return ControlFlow::Continue;
    }

    // ── GPIO / IoT Commands ─────────────────────────────────────────────
    // These emit special GPIO: prefixed output that the runtime intercepts.
    if cmd_up.starts_with("PINMODE ") || cmd_up.starts_with("PIN MODE ") {
        // PINMODE pin, mode  (mode: INPUT, OUTPUT, PWM)
        let args = after_first_word(command);
        let args = args.strip_prefix("MODE").unwrap_or(args).trim();
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if parts.len() >= 2 {
            let pin = ctx.eval_f64(parts[0]) as u8;
            let mode = parts[1].to_uppercase();
            ctx.emit(&format!("GPIO:PINMODE {} {}\n", pin, mode));
        }
        return ControlFlow::Continue;
    }
    if cmd_up.starts_with("DIGITALWRITE ") {
        // DIGITALWRITE pin, value  (value: HIGH/1 or LOW/0)
        let args = command[13..].trim();
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if parts.len() >= 2 {
            let pin = ctx.eval_f64(parts[0]) as u8;
            let val = parts[1].to_uppercase();
            let high = val == "HIGH" || val == "1" || val == "TRUE" || val == "ON";
            ctx.emit(&format!("GPIO:WRITE {} {}\n", pin, if high { 1 } else { 0 }));
        }
        return ControlFlow::Continue;
    }
    if cmd_up.starts_with("DIGITALREAD ") || cmd_up.starts_with("DIGITALREAD(") {
        // var = DIGITALREAD(pin)  — handled via eval, but standalone also works
        let args = command[11..].trim().trim_matches(|c| c == '(' || c == ')');
        let pin = ctx.eval_f64(args) as u8;
        ctx.emit(&format!("GPIO:READ {}\n", pin));
        return ControlFlow::Continue;
    }
    if cmd_up.starts_with("PWMWRITE ") || cmd_up.starts_with("ANALOGWRITE ") {
        // PWMWRITE pin, duty  (duty: 0-255 or 0.0-1.0)
        let offset = if cmd_up.starts_with("PWM") { 9 } else { 12 };
        let args = command[offset..].trim();
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if parts.len() >= 2 {
            let pin = ctx.eval_f64(parts[0]) as u8;
            let duty = ctx.eval_f64(parts[1]);
            // Normalize: if > 1.0, treat as 0-255 range
            let normalized = if duty > 1.0 { duty / 255.0 } else { duty };
            ctx.emit(&format!("GPIO:PWM {} {:.4}\n", pin, normalized));
        }
        return ControlFlow::Continue;
    }
    if cmd_up.starts_with("GPIORESET") || cmd_up.starts_with("GPIO RESET") {
        ctx.emit("GPIO:RESET\n");
        return ControlFlow::Continue;
    }
    if cmd_up.starts_with("SERVOWRITE ") {
        // SERVOWRITE pin, angle  (0-180)
        let args = command[11..].trim();
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if parts.len() >= 2 {
            let pin = ctx.eval_f64(parts[0]) as u8;
            let angle = ctx.eval_f64(parts[1]);
            // Convert servo angle (0-180) to PWM duty (approx 2.5%-12.5%)
            let duty = 0.025 + (angle / 180.0) * 0.1;
            ctx.emit(&format!("GPIO:PWM {} {:.4}\n", pin, duty));
        }
        return ControlFlow::Continue;
    }

    // ── Generic assignment (has `=` and isn't IF/FOR) ─────────────────────
    if command.contains('=')
        && !cmd_up.starts_with("IF ")
        && !cmd_up.starts_with("FOR ")
        && !cmd_up.starts_with("WHILE ")
    {
        return basic_let(ctx, command);
    }

    // ── Try as a user-defined sub call ────────────────────────────────────
    let name = command.split_whitespace().next().unwrap_or("").to_uppercase();
    if ctx.subs.contains_key(&name) {
        return basic_call(ctx, command);
    }

    // Unknown command — emit a note but continue
    ctx.emit(&format!("⚠️ Unknown BASIC: {command}\n"));
    ControlFlow::Continue
}

// ── multi-statement ────────────────────────────────────────────────────────

fn has_multi_statement(command: &str) -> bool {
    let upper = command.to_uppercase();
    if upper.starts_with("REM") || upper.starts_with("'") {
        return false;
    }
    let mut in_quotes = false;
    for ch in command.chars() {
        if ch == '"'  { in_quotes = !in_quotes; }
        if ch == ':'  && !in_quotes { return true; }
    }
    false
}

fn exec_multi(ctx: &mut ExecContext, command: &str) -> ControlFlow {
    for stmt in split_statements(command) {
        let stmt = stmt.trim();
        if stmt.is_empty() { continue; }
        match execute_basic(ctx, stmt) {
            ControlFlow::Continue => {}
            other => return other,
        }
    }
    ControlFlow::Continue
}

fn split_statements(command: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in command.chars() {
        if ch == '"' { in_quotes = !in_quotes; }
        if ch == ':' && !in_quotes {
            let t = current.trim().to_string();
            if !t.is_empty() { parts.push(t); }
            current.clear();
        } else {
            current.push(ch);
        }
    }
    let t = current.trim().to_string();
    if !t.is_empty() { parts.push(t); }
    parts
}

/// Resolve user-defined function calls in a BASIC expression.
/// Replaces `FUNCNAME(args)` with the evaluated return value.
fn resolve_basic_funcs(ctx: &mut ExecContext, expr: &str) -> String {
    let sub_names: Vec<String> = ctx.subs.keys().cloned().collect();
    if sub_names.is_empty() { return expr.to_string(); }
    let mut result = expr.to_string();
    for name in &sub_names {
        let mut iters = 0;
        loop {
            iters += 1;
            if iters > 20 { break; } // safety
            let upper_result = result.to_uppercase();
            let pattern = format!("{name}(");
            let pos = match upper_result.find(&pattern) {
                Some(p) => p,
                None => break,
            };
            let open = pos + name.len();
            let mut depth = 0i32;
            let mut close = open;
            for (j, ch) in result[open..].char_indices() {
                if ch == '(' { depth += 1; }
                if ch == ')' { depth -= 1; }
                if depth == 0 { close = open + j; break; }
            }
            let args_str = result[open+1..close].to_string();
            // Get sub definition
            let sub = match ctx.subs.get(name).cloned() {
                Some(s) => s,
                None => break,
            };
            // Evaluate arguments
            let arg_vals: Vec<f64> = if args_str.trim().is_empty() {
                vec![]
            } else {
                args_str.split(',').map(|a| ctx.eval_f64(a.trim())).collect()
            };
            // Bind parameters and execute body
            let saved: Vec<(String, f64)> = sub.params.iter().enumerate().map(|(i, p)| {
                let old = ctx.get_var(p);
                ctx.set_var(p, arg_vals.get(i).copied().unwrap_or(0.0));
                (p.clone(), old)
            }).collect();
            // Execute function body
            for line in &sub.body_lines {
                match execute_basic(ctx, line) {
                    ControlFlow::Continue => {}
                    ControlFlow::Return | ControlFlow::End => break,
                    _ => break,
                }
            }
            let ret = ctx.get_var(name);
            // Restore
            for (p, v) in saved { ctx.set_var(&p, v); }
            // Format return value
            let val_str = if ret == ret.floor() && ret.abs() < 1e15 {
                format!("{}", ret as i64)
            } else {
                format!("{ret}")
            };
            result = format!("{}{}{}", &result[..pos], val_str, &result[close+1..]);
        }
    }
    result
}

// ── PRINT ─────────────────────────────────────────────────────────────────────

fn basic_print(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    if args.is_empty() {
        ctx.emit("\n");
        return ControlFlow::Continue;
    }

    // Handle semicolons (same-line) and commas (tab spacing)
    let end_with_newline = !args.trim_end().ends_with(';');
    let parts: Vec<&str> = split_print_args(args);
    let mut out = String::new();

    for part in parts {
        let part = part.trim();
        if part.is_empty() { continue; }
        // String expression: contains " or $ with + (concatenation)
        if (part.contains('"') || part.contains('$')) && part.contains('+') {
            out.push_str(&eval_string(ctx, part));
        } else if part.starts_with('"') && part.ends_with('"') && part.len() >= 2 {
            out.push_str(&ctx.interpolate(&part[1..part.len()-1]));
        } else if part.to_uppercase().ends_with('$') {
            // String variable (NAME$) — resolve as string, not numeric.
            out.push_str(&ctx.get_str(part));
        } else if is_string_func(part) {
            // String function call: STR$(), CHR$(), LEFT$(), RIGHT$(), MID$()
            out.push_str(&eval_string(ctx, part));
        } else {
            // Try numeric expression — resolve user-defined functions first
            let resolved = resolve_basic_funcs(ctx, part);
            match ctx.eval_expr(&resolved) {
                Ok(v) => {
                    if v == v.floor() && v.abs() < 1e15 {
                        out.push_str(&format!("{}", v as i64));
                    } else {
                        out.push_str(&format!("{}", v));
                    }
                }
                Err(_) => {
                    // Try string variable
                    let sv = ctx.get_str(part);
                    if !sv.is_empty() {
                        out.push_str(&sv);
                    } else {
                        out.push_str(&ctx.interpolate(part));
                    }
                }
            }
        }
    }

    if end_with_newline { out.push('\n'); }
    ctx.emit(&out);
    ControlFlow::Continue
}

/// Check if a PRINT part is a BASIC string function call (STR$, CHR$, etc.)
fn is_string_func(part: &str) -> bool {
    let up = part.trim().to_uppercase();
    up.starts_with("STR$(") || up.starts_with("CHR$(")
        || up.starts_with("LEFT$(") || up.starts_with("RIGHT$(")
        || up.starts_with("MID$(")
}

fn split_print_args(args: &str) -> Vec<&str> {
    // Split by `;` or `,` respecting quoted strings and parentheses
    let mut result = Vec::new();
    let mut start = 0;
    let mut in_q = false;
    let mut paren_depth = 0i32;
    for (i, ch) in args.char_indices() {
        if ch == '"' { in_q = !in_q; }
        if !in_q {
            if ch == '(' { paren_depth += 1; }
            if ch == ')' { paren_depth -= 1; }
            if paren_depth == 0 && (ch == ';' || ch == ',') {
                result.push(&args[start..i]);
                start = i + 1;
            }
        }
    }
    result.push(&args[start..]);
    result
}

// ── INPUT ─────────────────────────────────────────────────────────────────────

fn basic_input(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    let (prompt, var) = if args.contains(';') {
        let mut it = args.splitn(2, ';');
        let p = it.next().unwrap_or("").trim().trim_matches('"').to_string();
        let v = it.next().unwrap_or("").trim().to_string();
        (p, v)
    } else if args.contains(',') {
        let mut it = args.splitn(2, ',');
        let p = it.next().unwrap_or("").trim().trim_matches('"').to_string();
        let v = it.next().unwrap_or("").trim().to_string();
        (p, v)
    } else {
        ("? ".to_string(), args.trim().to_string())
    };

    if var.is_empty() {
        return ControlFlow::Error("INPUT: missing variable name".to_string());
    }

    let is_num = !var.ends_with('$');
    ctx.request_input(&prompt, &var, is_num);
    ControlFlow::WaitInput
}

// ── IF ────────────────────────────────────────────────────────────────────────

fn basic_if(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    let upper = args.to_uppercase();

    // Split at THEN
    let (cond_str, then_str) = if let Some(pos) = find_keyword(&upper, "THEN") {
        (&args[..pos].trim(), &args[pos+4..].trim())
    } else {
        return ControlFlow::Error(format!("IF without THEN: {args}"));
    };

    // Split at ELSE (optional)
    let (then_part, else_part) = if let Some(pos) = find_keyword(&then_str.to_uppercase(), "ELSE") {
        (&then_str[..pos].trim(), Some(&then_str[pos+4..].trim()))
    } else {
        (then_str, None)
    };

    let cond_val = ctx.eval_f64(cond_str);
    let condition = cond_val != 0.0;

    if condition {
        if !then_part.is_empty() {
            let tp = then_part.to_string();
            return execute_basic(ctx, &tp);
        }
    } else if let Some(ep) = else_part {
        if !ep.is_empty() {
            let ep = ep.to_string();
            return execute_basic(ctx, &ep);
        }
    }
    ControlFlow::Continue
}

/// Detect multi-line block IF: `IF cond THEN` with nothing meaningful after THEN.
fn is_block_if(cmd_up: &str) -> bool {
    if let Some(pos) = cmd_up.find("THEN") {
        let after = cmd_up[pos + 4..].trim();
        after.is_empty()
    } else {
        false
    }
}

/// Handle a multi-line block IF.  Evaluates the condition and pushes the result
/// onto `if_block_stack`.  The subsequent lines will be executed or skipped
/// depending on the condition; `ELSE` flips the decision, `END IF` pops the stack.
fn basic_if_block(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    let upper = args.to_uppercase();
    let cond_str = if let Some(pos) = find_keyword(&upper, "THEN") {
        &args[..pos]
    } else {
        args
    };
    let cond_val = ctx.eval_f64(cond_str.trim());
    let cond = cond_val != 0.0;
    ctx.if_block_stack.push((cond, cond));
    ControlFlow::Continue
}

fn find_keyword(hay: &str, needle: &str) -> Option<usize> {
    // Simple word-boundary search
    let chars: Vec<char> = hay.chars().collect();
    let nlen = needle.len();
    let haylen = hay.len();
    if haylen < nlen { return None; }
    let mut in_q = false;
    let mut i = 0;
    while i + nlen <= haylen {
        if chars[i] == '"' { in_q = !in_q; }
        if !in_q && hay[i..].starts_with(needle) {
            let before_ok = i == 0 || !chars[i-1].is_alphanumeric();
            let after_ok  = i + nlen >= haylen || !chars[i + nlen].is_alphanumeric();
            if before_ok && after_ok {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

// ── GOTO ──────────────────────────────────────────────────────────────────────

fn resolve_jump(ctx: &ExecContext, target: &str) -> ControlFlow {
    let target = target.trim();
    if let Some(idx) = ctx.resolve_label(target) {
        ControlFlow::Jump(idx)
    } else {
        ControlFlow::Error(format!("GOTO: label not found: {target}"))
    }
}

// ── GOSUB ─────────────────────────────────────────────────────────────────────

fn basic_gosub(ctx: &mut ExecContext, target: &str) -> ControlFlow {
    let target = target.trim();
    if let Some(idx) = ctx.resolve_label(target) {
        ControlFlow::Gosub(idx)
    } else {
        ControlFlow::Error(format!("GOSUB: label not found: {target}"))
    }
}

fn basic_return(_ctx: &mut ExecContext) -> ControlFlow {
    ControlFlow::Return
}

// ── LET / assignment ──────────────────────────────────────────────────────────

fn basic_let(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    // Strip leading LET and whitespace
    let args = args.strip_prefix("LET").unwrap_or(args).trim();

    // Detect string assignment: LET A$ = "hello"
    if let Some(eq) = args.find('=') {
        let lhs = args[..eq].trim();
        let rhs = args[eq+1..].trim();

        // Array: NAME(index) = value
        if lhs.ends_with(')') {
            if let Some(op) = lhs.find('(') {
                let name = lhs[..op].trim().to_uppercase();
                let idx_str = &lhs[op+1..lhs.len()-1];
                let idx = ctx.eval_f64(idx_str) as usize;
                let val = ctx.eval_f64(rhs);
                ctx.set_array(&name, idx, val);
                return ControlFlow::Continue;
            }
        }

        // String variable: NAME$ = "..."
        if lhs.ends_with('$') {
            let name = lhs.to_uppercase();
            let sv = eval_string(ctx, rhs);
            ctx.set_str(&name, sv);
            return ControlFlow::Continue;
        }

        // Numeric: NAME = expr
        match ctx.eval_expr(rhs) {
            Ok(v) => ctx.set_var(lhs, v),
            Err(_) => {
                // Fallback: may be a string expression
                let sv = eval_string(ctx, rhs);
                ctx.set_str(lhs, sv);
            }
        }
    }
    ControlFlow::Continue
}

fn eval_string(ctx: &ExecContext, expr: &str) -> String {
    let expr = expr.trim();
    // String concatenation with + (check before quoted literal to handle "Hello " + N$)
    if expr.contains('+') {
        let parts = split_string_concat(expr);
        if parts.len() > 1 {
            let mut result = String::new();
            for part in parts {
                result.push_str(&eval_string(ctx, part.trim()));
            }
            return result;
        }
    }
    // Quoted literal (complete string)
    if expr.starts_with('"') && expr.ends_with('"') && expr.len() >= 2 {
        return expr[1..expr.len()-1].to_string();
    }

    // ── BASIC string functions ────────────────────────────────────────────
    let upper = expr.to_uppercase();
    // STR$(expr)  — convert number to string
    if upper.starts_with("STR$(") && upper.ends_with(')') {
        let inner = &expr[5..expr.len()-1];
        let v = ctx.eval_f64(inner);
        return if v == v.floor() && v.abs() < 1e15 {
            format!("{}", v as i64)
        } else {
            format!("{v}")
        };
    }
    // CHR$(n)  — ASCII code to character
    if upper.starts_with("CHR$(") && upper.ends_with(')') {
        let inner = &expr[5..expr.len()-1];
        let code = ctx.eval_f64(inner) as u8;
        return String::from(code as char);
    }
    // LEFT$(str, n)
    if upper.starts_with("LEFT$(") && upper.ends_with(')') {
        let inner = &expr[6..expr.len()-1];
        if let Some(comma) = inner.rfind(',') {
            let s = eval_string(ctx, inner[..comma].trim());
            let n = ctx.eval_f64(inner[comma+1..].trim()) as usize;
            return s.chars().take(n).collect();
        }
    }
    // RIGHT$(str, n)
    if upper.starts_with("RIGHT$(") && upper.ends_with(')') {
        let inner = &expr[7..expr.len()-1];
        if let Some(comma) = inner.rfind(',') {
            let s = eval_string(ctx, inner[..comma].trim());
            let n = ctx.eval_f64(inner[comma+1..].trim()) as usize;
            let chars: Vec<char> = s.chars().collect();
            let start = chars.len().saturating_sub(n);
            return chars[start..].iter().collect();
        }
    }
    // MID$(str, start [, length])
    if upper.starts_with("MID$(") && upper.ends_with(')') {
        let inner = &expr[5..expr.len()-1];
        let parts: Vec<&str> = inner.splitn(3, ',').collect();
        if parts.len() >= 2 {
            let s = eval_string(ctx, parts[0].trim());
            let start = (ctx.eval_f64(parts[1].trim()) as usize).saturating_sub(1); // 1-based
            let chars: Vec<char> = s.chars().collect();
            if parts.len() == 3 {
                let len = ctx.eval_f64(parts[2].trim()) as usize;
                return chars.iter().skip(start).take(len).collect();
            } else {
                return chars.iter().skip(start).collect();
            }
        }
    }

    // String variable
    if let Some(sv) = ctx.string_vars.get(&upper) {
        return sv.clone();
    }
    // Numeric as string
    if let Ok(v) = ctx.eval_expr(expr) {
        if v == v.floor() && v.abs() < 1e15 {
            return format!("{}", v as i64);
        }
        return format!("{}", v);
    }
    expr.to_string()
}

/// Split a string expression at `+` operators, respecting quoted strings.
fn split_string_concat(expr: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_q = false;
    for (i, ch) in expr.char_indices() {
        if ch == '"' { in_q = !in_q; }
        if ch == '+' && !in_q {
            parts.push(&expr[start..i]);
            start = i + 1;
        }
    }
    parts.push(&expr[start..]);
    parts
}

// ── FOR / NEXT ────────────────────────────────────────────────────────────────

fn basic_for(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    // Format: VAR = start TO end [STEP step]
    let upper = args.to_uppercase();
    let re_to = find_keyword(&upper, "TO");
    let re_step = find_keyword(&upper, "STEP");

    let (var_start, to_pos) = match (args.find('='), re_to) {
        (Some(e), Some(t)) => ((&args[..e], &args[e+1..t]), t),
        _ => return ControlFlow::Error(format!("FOR syntax: {args}")),
    };

    let var_name = var_start.0.trim().to_uppercase();
    let start_expr = var_start.1.trim();

    let (end_expr, step_expr) = if let Some(sp) = re_step {
        (&args[to_pos+2..sp].trim(), &args[sp+4..].trim())
    } else {
        (&args[to_pos+2..].trim(), &"1")
    };

    let start = ctx.eval_f64(start_expr);
    let end   = ctx.eval_f64(end_expr);
    let step  = ctx.eval_f64(step_expr);

    ctx.set_var(&var_name, start);

    // Check if loop should run at all
    if (step > 0.0 && start > end) || (step < 0.0 && start < end) {
        // Skip to matching NEXT
        return skip_to_next(ctx, &var_name);
    }

    ctx.push_for(&var_name, end, step, ctx.line_idx);
    ControlFlow::Continue
}

fn skip_to_next(ctx: &ExecContext, var: &str) -> ControlFlow {
    let upper_var = var.to_uppercase();
    for i in ctx.line_idx + 1..ctx.program_lines.len() {
        let (_, line) = &ctx.program_lines[i];
        let up = line.trim().to_uppercase();
        if up.starts_with("NEXT") {
            let rest = up[4..].trim().to_string();
            if rest.is_empty() || rest == upper_var {
                return ControlFlow::Jump(i + 1);
            }
        }
    }
    ControlFlow::End
}

fn basic_next(ctx: &mut ExecContext, var: Option<&str>) -> ControlFlow {
    match ctx.process_next(var) {
        Some(loop_start) => ControlFlow::Jump(loop_start),
        None => ControlFlow::Continue, // loop done
    }
}

// ── WHILE / WEND ─────────────────────────────────────────────────────────────

fn basic_while(ctx: &mut ExecContext, cond: &str) -> ControlFlow {
    let val = ctx.eval_f64(cond);
    if val == 0.0 {
        // Condition false — pop this frame if we had one, then skip to WEND
        if let Some((top_idx, _)) = ctx.while_stack.last() {
            if *top_idx == ctx.line_idx {
                ctx.while_stack.pop();
            }
        }
        return skip_to_wend(ctx);
    }
    // Only push a new frame if we're not already the top entry for this line
    if let Some((top_idx, _)) = ctx.while_stack.last() {
        if *top_idx == ctx.line_idx {
            return ControlFlow::Continue; // already tracked
        }
    }
    ctx.while_stack.push((ctx.line_idx, cond.to_string()));
    ControlFlow::Continue
}

fn skip_to_wend(ctx: &ExecContext) -> ControlFlow {
    let mut depth = 1usize;
    for i in ctx.line_idx + 1..ctx.program_lines.len() {
        let (_, line) = &ctx.program_lines[i];
        let up = line.trim().to_uppercase();
        if up.starts_with("WHILE ") { depth += 1; }
        if up == "WEND" {
            depth -= 1;
            if depth == 0 { return ControlFlow::Jump(i + 1); }
        }
    }
    ControlFlow::End
}

fn basic_wend(ctx: &mut ExecContext) -> ControlFlow {
    if let Some((while_idx, cond)) = ctx.while_stack.last().cloned() {
        let val = ctx.eval_f64(&cond);
        if val != 0.0 {
            return ControlFlow::Jump(while_idx);
        }
        ctx.while_stack.pop();
    }
    ControlFlow::Continue
}

// ── DO / LOOP ─────────────────────────────────────────────────────────────────

fn basic_loop(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    let upper = args.to_uppercase();
    if let Some(do_idx) = ctx.do_stack.last().copied() {
        if upper.starts_with("WHILE ") {
            let cond = &args[6..].trim();
            let val = ctx.eval_f64(cond);
            if val != 0.0 {
                return ControlFlow::Jump(do_idx + 1);
            }
        } else if upper.starts_with("UNTIL ") {
            let cond = &args[6..].trim();
            let val = ctx.eval_f64(cond);
            if val == 0.0 {
                return ControlFlow::Jump(do_idx + 1);
            }
        } else {
            // Always loop back
            return ControlFlow::Jump(do_idx + 1);
        }
        ctx.do_stack.pop();
    }
    ControlFlow::Continue
}

// ── SELECT CASE ───────────────────────────────────────────────────────────────

fn basic_case(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    if !ctx.in_select { return ControlFlow::Continue; }
    if args.to_uppercase() == "ELSE" {
        if !ctx.select_matched {
            ctx.select_matched = true;
        } else {
            // Skip to END SELECT
            return skip_to_end_select(ctx);
        }
        return ControlFlow::Continue;
    }
    let case_val = ctx.eval_f64(args);
    if ctx.select_matched {
        return skip_to_end_select(ctx);
    }
    if let Some(sel) = ctx.select_val {
        if (sel - case_val).abs() < 1e-9 {
            ctx.select_matched = true;
        } else {
            return skip_to_next_case(ctx);
        }
    }
    ControlFlow::Continue
}

fn skip_to_end_select(ctx: &ExecContext) -> ControlFlow {
    for i in ctx.line_idx + 1..ctx.program_lines.len() {
        let (_, line) = &ctx.program_lines[i];
        if line.trim().to_uppercase() == "END SELECT" {
            return ControlFlow::Jump(i);
        }
    }
    ControlFlow::End
}

fn skip_to_next_case(ctx: &ExecContext) -> ControlFlow {
    for i in ctx.line_idx + 1..ctx.program_lines.len() {
        let (_, line) = &ctx.program_lines[i];
        let up = line.trim().to_uppercase();
        if up.starts_with("CASE ") || up == "END SELECT" {
            return ControlFlow::Jump(i);
        }
    }
    ControlFlow::End
}

// ── SUB / FUNCTION ────────────────────────────────────────────────────────────

fn basic_sub(ctx: &mut ExecContext, header: &str) -> ControlFlow {
    // Collect sub body until END SUB / END FUNCTION
    let name_part = header.split('(').next().unwrap_or(header).trim().to_uppercase();
    let params: Vec<String> = if let Some(op) = header.find('(') {
        if let Some(cp) = header.find(')') {
            header[op+1..cp]
                .split(',')
                .map(|p| p.trim().to_uppercase())
                .collect()
        } else { vec![] }
    } else { vec![] };

    let mut body = Vec::new();
    let start = ctx.line_idx + 1;
    let mut end_idx = ctx.program_lines.len();
    for i in start..ctx.program_lines.len() {
        let (_, line) = &ctx.program_lines[i];
        let up = line.trim().to_uppercase();
        if up == "END SUB" || up == "END FUNCTION" {
            end_idx = i;
            break;
        }
        body.push(line.clone());
    }

    ctx.subs.insert(name_part.clone(), crate::context::SubDef {
        name: name_part,
        params,
        body_lines: body,
    });

    // Skip over the sub body
    ControlFlow::Jump(end_idx + 1)
}

fn basic_end_sub(_ctx: &mut ExecContext) -> ControlFlow {
    // Reached when BASIC falls into END SUB outside of a call — treat as END
    ControlFlow::Return
}

fn basic_call(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    let (name, raw_args) = if let Some(op) = args.find('(') {
        let n = args[..op].trim().to_uppercase();
        let rest = args[op+1..].trim_end_matches(')').trim();
        (n, rest.to_string())
    } else {
        (args.trim().to_uppercase(), String::new())
    };

    let sub_def = match ctx.subs.get(&name).cloned() {
        Some(s) => s,
        None => {
            ctx.emit(&format!("⚠️ Undefined sub: {name}\n"));
            return ControlFlow::Continue;
        }
    };

    // Bind parameters
    let arg_vals: Vec<f64> = raw_args
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|e| ctx.eval_f64(e.trim()))
        .collect();

    let saved_vars: std::collections::HashMap<String, f64> = sub_def.params
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let old = ctx.get_var(p);
            ctx.set_var(p, arg_vals.get(i).copied().unwrap_or(0.0));
            (p.clone(), old)
        })
        .collect();

    // Execute body inline
    let body = sub_def.body_lines.clone();
    for line in &body {
        match execute_basic(ctx, line) {
            ControlFlow::Continue => {}
            ControlFlow::End | ControlFlow::Return => break,
            ControlFlow::Error(e) => {
                ctx.emit(&format!("❌ Error in {}: {e}\n", sub_def.name));
                break;
            }
            _ => {}
        }
    }

    // Restore saved params
    for (p, v) in saved_vars {
        ctx.set_var(&p, v);
    }
    ControlFlow::Continue
}

// ── DIM ───────────────────────────────────────────────────────────────────────

fn basic_dim(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    for decl in args.split(',') {
        let decl = decl.trim();
        if let Some(op) = decl.find('(') {
            let name = decl[..op].trim().to_uppercase();
            let size_str = decl[op+1..].trim_end_matches(')');
            let size = ctx.eval_f64(size_str) as usize;
            ctx.dim_array(&name, size);
        }
    }
    ControlFlow::Continue
}

// ── COLOR ─────────────────────────────────────────────────────────────────────

fn basic_color(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    let parts: Vec<&str> = args.split(',').collect();
    if let Some(fg) = parts.first() {
        if let Ok(v) = fg.trim().parse::<f64>() {
            if let Some(c) = tw_graphics::turtle::default_palette_16(v as u8) {
                ctx.turtle.set_pen_color(c);
            }
        } else if let Some(c) = parse_color(fg.trim()) {
            ctx.turtle.set_pen_color(c);
        }
    }
    if let Some(bg) = parts.get(1) {
        if let Ok(v) = bg.trim().parse::<f64>() {
            if let Some(c) = tw_graphics::turtle::default_palette_16(v as u8) {
                ctx.turtle.set_bg_color(c);
            }
        } else if let Some(c) = parse_color(bg.trim()) {
            ctx.turtle.set_bg_color(c);
        }
    }
    ControlFlow::Continue
}

// ── LINE ──────────────────────────────────────────────────────────────────────

fn basic_line(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    // LINE (x1,y1)-(x2,y2)[,color[,B|BF]]
    let args_upper = args.to_uppercase();
    let has_bf = args_upper.contains(",BF") || args_upper.contains(", BF");
    let has_b  = !has_bf && (args_upper.ends_with(",B") || args_upper.ends_with(", B")
                          || args_upper.contains(",B,") || args_upper.contains(", B,"));

    // Strip B/BF flags, then remove parens and `-(` separator
    let cleaned = args_upper
        .replace(",BF", "").replace(", BF", "")
        .replace(", B", "").replace(",B", "");
    let coord_args = cleaned.replace("-(", ",").replace("(", "").replace(")", "");
    let parts: Vec<&str> = coord_args.split(',').collect();
    let nums: Vec<f64> = parts.iter()
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() { return None; }
            ctx.eval_expr(s).ok().or_else(|| s.parse().ok())
        })
        .collect();

    // Save and optionally apply color from 5th numeric parameter
    let saved_color = ctx.turtle.pen_color;
    if nums.len() >= 5 {
        if let Some(c) = tw_graphics::turtle::default_palette_16(nums[4] as u8) {
            ctx.turtle.set_pen_color(c);
        }
    }

    if nums.len() >= 4 {
        let (x1, y1, x2, y2) = (nums[0], nums[1], nums[2], nums[3]);
        if has_bf {
            // Filled box
            let saved = (ctx.turtle.x, ctx.turtle.y);
            let was_down = ctx.turtle.pen_down;
            ctx.turtle.pen_down = false;
            ctx.turtle.set_pos(x1, y1);
            ctx.turtle.pen_down = true;
            ctx.turtle.begin_fill(ctx.turtle.pen_color);
            ctx.turtle.move_to(x2, y1);
            ctx.turtle.move_to(x2, y2);
            ctx.turtle.move_to(x1, y2);
            ctx.turtle.move_to(x1, y1);
            ctx.turtle.end_fill();
            ctx.turtle.pen_down = was_down;
            ctx.turtle.set_pos(saved.0, saved.1);
        } else if has_b {
            // Box outline (no fill)
            let saved = (ctx.turtle.x, ctx.turtle.y);
            let was_down = ctx.turtle.pen_down;
            ctx.turtle.pen_down = false;
            ctx.turtle.set_pos(x1, y1);
            ctx.turtle.pen_down = true;
            ctx.turtle.move_to(x2, y1);
            ctx.turtle.move_to(x2, y2);
            ctx.turtle.move_to(x1, y2);
            ctx.turtle.move_to(x1, y1);
            ctx.turtle.pen_down = was_down;
            ctx.turtle.set_pos(saved.0, saved.1);
        } else {
            // Plain line
            let saved = (ctx.turtle.x, ctx.turtle.y);
            let was_down = ctx.turtle.pen_down;
            ctx.turtle.pen_down = false;
            ctx.turtle.set_pos(x1, y1);
            ctx.turtle.pen_down = true;
            ctx.turtle.move_to(x2, y2);
            ctx.turtle.pen_down = was_down;
            ctx.turtle.set_pos(saved.0, saved.1);
        }
    }

    // Restore previous pen color
    if nums.len() >= 5 {
        ctx.turtle.set_pen_color(saved_color);
    }
    ControlFlow::Continue
}

// ── CIRCLE ────────────────────────────────────────────────────────────────────

fn basic_circle(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    // CIRCLE (cx,cy), radius [,color]
    let cleaned = args.replace("(", "").replace(")", "");
    let parts: Vec<&str> = cleaned.split(',').map(|s| s.trim()).collect();
    let nums: Vec<f64> = parts.iter()
        .filter_map(|s| ctx.eval_expr(s).ok())
        .collect();
    if nums.len() >= 3 {
        let (cx, cy, r) = (nums[0], nums[1], nums[2]);

        // Optional color as 4th parameter
        let saved_color = ctx.turtle.pen_color;
        if nums.len() >= 4 {
            if let Some(c) = tw_graphics::turtle::default_palette_16(nums[3] as u8) {
                ctx.turtle.set_pen_color(c);
            }
        }

        let steps = ((2.0 * std::f64::consts::PI * r / 2.0) as usize).max(16);
        let step_angle = 2.0 * std::f64::consts::PI / steps as f64;
        let saved = (ctx.turtle.x, ctx.turtle.y, ctx.turtle.heading, ctx.turtle.pen_down);
        ctx.turtle.pen_down = false;
        ctx.turtle.set_pos(cx + r, cy);
        ctx.turtle.pen_down = true;
        for i in 1..=steps {
            let angle = i as f64 * step_angle;
            let nx = cx + r * angle.cos();
            let ny = cy + r * angle.sin();
            ctx.turtle.move_to(nx, ny);
        }
        // Ensure circle is closed by returning to start point
        ctx.turtle.move_to(cx + r, cy);
        ctx.turtle.pen_down = saved.3;
        ctx.turtle.set_pos(saved.0, saved.1);
        ctx.turtle.heading = saved.2;

        // Restore color
        if nums.len() >= 4 {
            ctx.turtle.set_pen_color(saved_color);
        }
    }
    ControlFlow::Continue
}

// ── PSET ─────────────────────────────────────────────────────────────────────

fn basic_pset(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    let cleaned = args.replace("(", "").replace(")", "");
    let nums: Vec<f64> = cleaned
        .split(',')
        .filter_map(|s| ctx.eval_expr(s.trim()).ok())
        .collect();
    if nums.len() >= 2 {
        // Move to the target position first, then draw the dot
        ctx.turtle.set_pos(nums[0], nums[1]);
        ctx.turtle.dot(1.0, None);
    }
    ControlFlow::Continue
}

// ── DRAW ─────────────────────────────────────────────────────────────────────

fn basic_draw(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    // GW-BASIC DRAW macro language: M x,y; Un; Dn; Ln; Rn; etc.
    let s = args.trim().trim_matches('"').to_uppercase();
    let mut i = 0;
    let chars: Vec<char> = s.chars().collect();
    while i < chars.len() {
        let cmd = chars[i];
        i += 1;
        // Read optional number
        let num_start = i;
        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '-') {
            i += 1;
        }
        let n: f64 = if i > num_start {
            chars[num_start..i].iter().collect::<String>().parse().unwrap_or(0.0)
        } else { 10.0 };

        match cmd {
            'U' => { ctx.turtle.set_heading(0.0);   ctx.turtle.forward(n); }
            'D' => { ctx.turtle.set_heading(180.0); ctx.turtle.forward(n); }
            'L' => { ctx.turtle.set_heading(270.0); ctx.turtle.forward(n); }
            'R' => { ctx.turtle.set_heading(90.0);  ctx.turtle.forward(n); }
            'E' => { ctx.turtle.set_heading(45.0);  ctx.turtle.forward(n); }
            'F' => { ctx.turtle.set_heading(135.0); ctx.turtle.forward(n); }
            'G' => { ctx.turtle.set_heading(225.0); ctx.turtle.forward(n); }
            'H' => { ctx.turtle.set_heading(315.0); ctx.turtle.forward(n); }
            'B' => { ctx.turtle.pen_up(); }
            'N' => { ctx.turtle.pen_down_cmd(); }
            _ => {}
        }
    }
    ControlFlow::Continue
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn after_first_word(s: &str) -> &str {
    s.splitn(2, ' ').nth(1).unwrap_or("").trim()
}

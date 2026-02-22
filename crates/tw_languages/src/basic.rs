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

    // Ignore label-only lines (e.g. `100:` or `MYLOOP:`)
    if command.ends_with(':') && !command.contains(' ') {
        return ControlFlow::Continue;
    }

    // Handle multi-statement lines joined by `:`
    if has_multi_statement(command) {
        return exec_multi(ctx, command);
    }

    let cmd_up = command.to_uppercase();
    let cmd_up = cmd_up.as_str();

    // ── Comments ──────────────────────────────────────────────────────────
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
        return basic_if(ctx, command[3..].trim());
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
    if cmd_up == "END" || cmd_up.starts_with("END ") {
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

    // ── SOUND / BEEP ─────────────────────────────────────────────────────
    if cmd_up.starts_with("SOUND") || cmd_up == "BEEP" {
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
            out.push_str(&part[1..part.len()-1]);
        } else {
            // Try numeric expression
            match ctx.eval_expr(part) {
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
    // String variable
    let upper = expr.to_uppercase();
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
    // LINE (x1,y1)-(x2,y2)[,color[,BF]]
    let args = args.replace("-(", ",").replace("(", "").replace(")", "");
    let nums: Vec<f64> = args
        .split(',')
        .filter_map(|s| ctx.eval_expr(s.trim()).ok().or_else(|| s.trim().parse().ok()))
        .collect();

    if nums.len() >= 4 {
        let (x1, y1, x2, y2) = (nums[0], nums[1], nums[2], nums[3]);
        let saved = (ctx.turtle.x, ctx.turtle.y);
        let was_down = ctx.turtle.pen_down;
        ctx.turtle.pen_down = false;
        ctx.turtle.set_pos(x1, y1);
        ctx.turtle.pen_down = true;
        ctx.turtle.move_to(x2, y2);
        ctx.turtle.pen_down = was_down;
        ctx.turtle.set_pos(saved.0, saved.1);
    }
    ControlFlow::Continue
}

// ── CIRCLE ────────────────────────────────────────────────────────────────────

fn basic_circle(ctx: &mut ExecContext, args: &str) -> ControlFlow {
    // CIRCLE (cx,cy), radius [,color]
    let cleaned = args.replace("(", "").replace(")", "");
    let nums: Vec<f64> = cleaned
        .split(',')
        .filter_map(|s| ctx.eval_expr(s.trim()).ok())
        .collect();
    if nums.len() >= 3 {
        let (cx, cy, r) = (nums[0], nums[1], nums[2]);
        let steps = (2.0 * std::f64::consts::PI * r / 2.0) as usize + 16;
        let step_angle = 360.0 / steps as f64;
        let saved = (ctx.turtle.x, ctx.turtle.y, ctx.turtle.heading, ctx.turtle.pen_down);
        ctx.turtle.pen_down = false;
        ctx.turtle.set_pos(cx + r, cy);
        ctx.turtle.pen_down = true;
        for i in 1..=steps {
            let angle = (i as f64 * step_angle).to_radians();
            let nx = cx + r * angle.cos();
            let ny = cy + r * angle.sin();
            ctx.turtle.move_to(nx, ny);
        }
        ctx.turtle.pen_down = saved.3;
        ctx.turtle.set_pos(saved.0, saved.1);
        ctx.turtle.heading = saved.2;
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
        ctx.turtle.dot(1.0, None);
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

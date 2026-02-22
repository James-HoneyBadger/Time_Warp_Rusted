//! C language executor (subset) — port of `languages/c_lang.py` / `c_lang_fixed.py`.
//!
//! Supports: variable declarations, printf/scanf, if/else, while, for,
//! functions, basic stdio operations.

use crate::context::{ControlFlow, ExecContext};

pub fn execute_c(ctx: &mut ExecContext, command: &str) -> ControlFlow {
    let line = command.trim();
    if line.is_empty() || line.starts_with("//") || line.starts_with("/*") {
        return ControlFlow::Continue;
    }

    // Strip trailing semicolon
    let line = line.trim_end_matches(';').trim();

    let upper = line.to_uppercase();

    // ── preprocessor / includes ──────────────────────────────────────────
    if upper.starts_with("#INCLUDE") || upper.starts_with("#DEFINE") {
        return ControlFlow::Continue;
    }

    // ── printf ───────────────────────────────────────────────────────────
    if upper.starts_with("PRINTF") || upper.starts_with("PUTS") || upper.starts_with("PUTCHAR") {
        return c_printf(ctx, line);
    }

    // ── scanf / gets ─────────────────────────────────────────────────────
    if upper.starts_with("SCANF") || upper.starts_with("GETS") {
        return c_scanf(ctx, line);
    }

    // ── return ───────────────────────────────────────────────────────────
    if upper.starts_with("RETURN") {
        let rest = line[6..].trim();
        if !rest.is_empty() {
            let v = ctx.eval_f64(rest);
            ctx.set_var("RETURN", v);
        }
        return ControlFlow::Return;
    }

    // ── Block braces ──────────────────────────────────────────────────────
    if line == "{" {
        return ControlFlow::Continue;
    }
    if line == "}" || line.starts_with("}") {
        // Check for `} else {` — if we reach here, the if-branch was taken, skip else block
        let upper_line = line.to_uppercase();
        if upper_line.contains("ELSE") {
            return c_skip_block(ctx);
        }
        // Check if closing a while loop
        if let Some((while_idx, ref cond)) = ctx.while_stack.last().cloned() {
            // Re-evaluate condition and jump back if true
            let val = ctx.eval_f64(&cond);
            if val != 0.0 {
                return ControlFlow::Jump(while_idx);
            }
            ctx.while_stack.pop();
            return ControlFlow::Continue;
        }
        // Check if closing a for loop
        if let Some(frame) = ctx.for_stack.last().cloned() {
            return ControlFlow::Jump(frame.for_idx);
        }
        return ControlFlow::Continue;
    }

    // ── if (cond) { ──────────────────────────────────────────────────────
    if upper.starts_with("IF") && line[2..].trim_start().starts_with('(') {
        return c_if(ctx, line);
    }

    // ── } else { ─────────────────────────────────────────────────────────
    // If we reach `else`, it means the IF branch was taken, so skip the else block
    if upper.starts_with("ELSE") || upper == "} ELSE {" || line.starts_with("} else") {
        return c_skip_block(ctx);
    }

    // ── while (cond) { ───────────────────────────────────────────────────
    if upper.starts_with("WHILE") && line[5..].trim_start().starts_with('(') {
        return c_while(ctx, line);
    }

    // ── for (init; cond; inc) { ──────────────────────────────────────────
    if upper.starts_with("FOR") && line[3..].trim_start().starts_with('(') {
        return c_for(ctx, line);
    }

    // ── Type declarations: int x = 5; float y; ───────────────────────────
    if starts_with_type(&upper) {
        return c_declare(ctx, line);
    }

    // ── Assignments: x = expr; x += 1; x++; ++x; ────────────────────────
    if line.ends_with("++") {
        let var = line[..line.len()-2].trim().to_uppercase();
        let v = ctx.get_var(&var) + 1.0;
        ctx.set_var(&var, v);
        return ControlFlow::Continue;
    }
    if line.ends_with("--") {
        let var = line[..line.len()-2].trim().to_uppercase();
        let v = ctx.get_var(&var) - 1.0;
        ctx.set_var(&var, v);
        return ControlFlow::Continue;
    }
    if line.starts_with("++") {
        let var = line[2..].trim().to_uppercase();
        let v = ctx.get_var(&var) + 1.0;
        ctx.set_var(&var, v);
        return ControlFlow::Continue;
    }
    if line.starts_with("--") {
        let var = line[2..].trim().to_uppercase();
        let v = ctx.get_var(&var) - 1.0;
        ctx.set_var(&var, v);
        return ControlFlow::Continue;
    }

    if let Some(pos) = find_compound_assign(line) {
        return c_compound_assign(ctx, &line[..pos], &line[pos..]);
    }
    if line.contains('=') && !upper.starts_with("IF") && !upper.starts_with("WHILE") && !upper.starts_with("FOR") {
        return c_assign(ctx, line);
    }

    // ── Function call ─────────────────────────────────────────────────────
    if line.contains('(') {
        return c_call(ctx, line);
    }

    ControlFlow::Continue
}

// ── Type handling ─────────────────────────────────────────────────────────────

fn starts_with_type(upper: &str) -> bool {
    upper.starts_with("INT ")
        || upper.starts_with("FLOAT ")
        || upper.starts_with("DOUBLE ")
        || upper.starts_with("CHAR ")
        || upper.starts_with("LONG ")
        || upper.starts_with("SHORT ")
        || upper.starts_with("UNSIGNED ")
        || upper.starts_with("VOID ")
        || upper.starts_with("CONST ")
        || upper.starts_with("STATIC ")
}

fn c_declare(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    // Strip ALL leading type keywords (handles: const int, unsigned long, static int, etc.)
    let type_keywords = ["INT", "FLOAT", "DOUBLE", "CHAR", "LONG", "SHORT",
                         "UNSIGNED", "VOID", "CONST", "STATIC", "SIGNED"];
    let mut rest = line.trim();
    loop {
        let upper = rest.to_uppercase();
        let mut found = false;
        for kw in &type_keywords {
            let prefix = format!("{kw} ");
            if upper.starts_with(&prefix) {
                rest = rest[prefix.len()..].trim();
                found = true;
                break;
            }
        }
        if !found { break; }
    }
    // Also skip pointer asterisks
    let rest = rest.trim_start_matches('*').trim();
    // Handle function definitions like `main() {` — just skip them
    if rest.contains('(') {
        return ControlFlow::Continue;
    }
    // Multiple declarations: int a, b = 5, c;
    for decl in rest.split(',') {
        let decl = decl.trim().trim_end_matches(';').trim();
        if decl.contains('=') {
            if let Some(eq) = decl.find('=') {
                let name = decl[..eq].trim().to_uppercase();
                let expr = decl[eq+1..].trim();
                if expr.starts_with('"') {
                    ctx.set_str(&name, expr.trim_matches('"').to_string());
                } else {
                    let v = ctx.eval_f64(expr);
                    ctx.set_var(&name, v);
                }
            }
        } else {
            let name = decl.to_uppercase();
            if !name.is_empty() && name.chars().next().map_or(false, |c| c.is_alphabetic()) {
                ctx.set_var(&name, 0.0);
            }
        }
    }
    ControlFlow::Continue
}

// ── printf ────────────────────────────────────────────────────────────────────

fn c_printf(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    // printf("format", args...)
    let inner = extract_call_args(line);
    if inner.is_empty() {
        return ControlFlow::Continue;
    }

    let args = split_call_args(&inner);
    if args.is_empty() {
        return ControlFlow::Continue;
    }

    let fmt = args[0].trim().trim_matches('"');
    let vals: Vec<f64> = args[1..].iter().map(|a| ctx.eval_f64(a.trim())).collect();
    let val_strs: Vec<String> = args[1..]
        .iter()
        .map(|a| {
            let a = a.trim();
            if a.starts_with('"') {
                a.trim_matches('"').to_string()
            } else {
                let sv = ctx.get_str(a);
                if !sv.is_empty() { sv } else {
                    let v = ctx.eval_f64(a);
                    if v == v.floor() && v.abs() < 1e15 { format!("{}", v as i64) }
                    else { format!("{v}") }
                }
            }
        })
        .collect();

    let out = format_printf(fmt, &vals, &val_strs);
    let out = out.replace("\\n", "\n").replace("\\t", "\t");
    ctx.emit(&out);
    ControlFlow::Continue
}

fn format_printf(fmt: &str, vals: &[f64], strs: &[String]) -> String {
    let mut out = String::new();
    let mut chars = fmt.chars().peekable();
    let mut vi = 0usize;
    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&spec) = chars.peek() {
                chars.next();
                let val = vals.get(vi).copied().unwrap_or(0.0);
                let s   = strs.get(vi).map(|s| s.as_str()).unwrap_or("");
                vi += 1;
                match spec {
                    'd' | 'i' => out.push_str(&format!("{}", val as i64)),
                    'f'       => out.push_str(&format!("{:.6}", val)),
                    'e'       => out.push_str(&format!("{:e}", val)),
                    'g'       => out.push_str(&format!("{}", val)),
                    's'       => out.push_str(s),
                    'c'       => {
                        if let Some(c) = char::from_u32(val as u32) { out.push(c); }
                    }
                    '%'       => { out.push('%'); vi -= 1; }
                    _         => { out.push('%'); out.push(spec); }
                }
            } else {
                out.push('%');
            }
        } else if ch == '\\' {
            if let Some(&esc) = chars.peek() {
                chars.next();
                match esc {
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    'r' => out.push('\r'),
                    '\\' => out.push('\\'),
                    '"' => out.push('"'),
                    _ => { out.push('\\'); out.push(esc); }
                }
            }
        } else {
            out.push(ch);
        }
    }
    out
}

// ── scanf ─────────────────────────────────────────────────────────────────────

fn c_scanf(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    let inner = extract_call_args(line);
    let args = split_call_args(&inner);
    if args.len() < 2 {
        return ControlFlow::Continue;
    }
    let var = args[1].trim().trim_start_matches('&').to_uppercase();
    ctx.request_input("", &var, true);
    ControlFlow::WaitInput
}

// ── assignment ────────────────────────────────────────────────────────────────

fn c_assign(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    if let Some(eq) = line.find('=') {
        let lhs = line[..eq].trim().to_uppercase();
        let rhs = line[eq+1..].trim();
        if rhs.starts_with('"') {
            ctx.set_str(&lhs, rhs.trim_matches('"').to_string());
        } else {
            let v = ctx.eval_f64(rhs);
            ctx.set_var(&lhs, v);
        }
    }
    ControlFlow::Continue
}

fn find_compound_assign(line: &str) -> Option<usize> {
    for op in &["+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="] {
        if let Some(pos) = line.find(op) {
            return Some(pos);
        }
    }
    None
}

fn c_compound_assign(ctx: &mut ExecContext, lhs: &str, op_rhs: &str) -> ControlFlow {
    let name = lhs.trim().to_uppercase();
    let (op, rhs) = if op_rhs.starts_with("+=") {
        ('+', op_rhs[2..].trim())
    } else if op_rhs.starts_with("-=") {
        ('-', op_rhs[2..].trim())
    } else if op_rhs.starts_with("*=") {
        ('*', op_rhs[2..].trim())
    } else if op_rhs.starts_with("/=") {
        ('/', op_rhs[2..].trim())
    } else {
        return ControlFlow::Continue;
    };
    let right = ctx.eval_f64(rhs);
    let left  = ctx.get_var(&name);
    let result = match op {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => if right != 0.0 { left / right } else { 0.0 },
        _ => left,
    };
    ctx.set_var(&name, result);
    ControlFlow::Continue
}

// ── function call ─────────────────────────────────────────────────────────────

fn c_call(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    if let Some(op) = line.find('(') {
        let name = line[..op].trim().to_uppercase();
        let args_str = extract_call_args(line);
        let args: Vec<f64> = split_call_args(&args_str)
            .iter()
            .map(|a| ctx.eval_f64(a.trim()))
            .collect();

        // Built-in math functions
        let result = match name.as_str() {
            "ABS"   => args.first().map(|v| v.abs()),
            "SQRT"  => args.first().map(|v| v.sqrt()),
            "POW"   => if args.len() >= 2 { Some(args[0].powf(args[1])) } else { None },
            "SIN"   => args.first().map(|v| v.to_radians().sin()),
            "COS"   => args.first().map(|v| v.to_radians().cos()),
            "TAN"   => args.first().map(|v| v.to_radians().tan()),
            "FLOOR" => args.first().map(|v| v.floor()),
            "CEIL"  => args.first().map(|v| v.ceil()),
            _ => None,
        };
        if let Some(v) = result {
            ctx.set_var("RETURN", v);
        }
    }
    ControlFlow::Continue
}

// ── C control flow ────────────────────────────────────────────────────────────

fn c_if(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    let cond_str = extract_paren_expr(line, 2);
    let val = ctx.eval_f64(&cond_str);
    if val == 0.0 {
        // Condition false — skip to matching `} else {` or end of block
        c_skip_to_else_or_end(ctx)
    } else {
        ControlFlow::Continue
    }
}

fn c_while(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    let cond_str = extract_paren_expr(line, 5);
    let val = ctx.eval_f64(&cond_str);
    if val == 0.0 {
        // False — skip block
        return c_skip_block(ctx);
    }
    // Push while loop info: (line_idx, condition)
    // Store in while_stack same as BASIC (reusing context fields)
    let already = ctx.while_stack.last().map_or(false, |(idx, _)| *idx == ctx.line_idx);
    if !already {
        ctx.while_stack.push((ctx.line_idx, cond_str));
    }
    ControlFlow::Continue
}

fn c_for(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    let paren = extract_paren_expr(line, 3);
    let parts: Vec<&str> = paren.splitn(3, ';').collect();
    if parts.len() < 3 {
        return ControlFlow::Continue;
    }
    let init = parts[0].trim();
    let cond = parts[1].trim();
    let inc  = parts[2].trim();

    // On first entry, execute init and push loop info
    let already = ctx.for_stack.iter().any(|f| f.for_idx == ctx.line_idx);
    if !already {
        execute_c(ctx, init);
        let val = if cond.is_empty() { 1.0 } else { ctx.eval_f64(cond) };
        if val == 0.0 {
            return c_skip_block(ctx);
        }
        ctx.for_stack.push(crate::context::ForFrame {
            var_name: inc.to_string(),         // reuse var_name to store increment expr
            end_val: 0.0,                      // unused for C
            step: 0.0,                         // unused for C
            for_idx: ctx.line_idx,
        });
        // Also store the condition in a string var so we can re-evaluate
        ctx.set_str(&format!("__FOR_COND_{}", ctx.line_idx), cond.to_string());
        ControlFlow::Continue
    } else {
        // Re-entry: execute increment, check condition
        execute_c(ctx, inc);
        let cond_key = format!("__FOR_COND_{}", ctx.line_idx);
        let cond = ctx.get_str(&cond_key);
        let cond = if cond.is_empty() { "1".to_string() } else { cond };
        let val = ctx.eval_f64(&cond);
        if val == 0.0 {
            ctx.for_stack.retain(|f| f.for_idx != ctx.line_idx);
            return c_skip_block(ctx);
        }
        ControlFlow::Continue
    }
}

/// Skip to the end of the current block (matching `}`), handling nesting.
fn c_skip_block(ctx: &ExecContext) -> ControlFlow {
    let mut depth = 1i32;
    for i in ctx.line_idx + 1..ctx.program_lines.len() {
        let (_, ref line) = ctx.program_lines[i];
        let t = line.trim();
        for ch in t.chars() {
            if ch == '{' { depth += 1; }
            if ch == '}' { depth -= 1; }
        }
        if depth <= 0 {
            return ControlFlow::Jump(i + 1);
        }
    }
    ControlFlow::End
}

/// Skip to `} else {` or end of block.
fn c_skip_to_else_or_end(ctx: &ExecContext) -> ControlFlow {
    let mut depth = 1i32;
    for i in ctx.line_idx + 1..ctx.program_lines.len() {
        let (_, ref line) = ctx.program_lines[i];
        let t = line.trim();
        for ch in t.chars() {
            if ch == '{' { depth += 1; }
            if ch == '}' { depth -= 1; }
        }
        if depth == 0 {
            // Check if this line or the next is `else`
            let upper = t.to_uppercase();
            if upper.contains("ELSE") {
                // Jump to after the `else {` — the else block
                return ControlFlow::Jump(i + 1);
            }
            // Check the next line for `else`
            if i + 1 < ctx.program_lines.len() {
                let next = ctx.program_lines[i + 1].1.trim().to_uppercase();
                if next.starts_with("ELSE") || next.starts_with("} ELSE") {
                    return ControlFlow::Jump(i + 2); // skip the `else {` line
                }
            }
            return ControlFlow::Jump(i + 1);
        }
    }
    ControlFlow::End
}

/// Extract the parenthesized expression from a line like `if (x > 0) {`
fn extract_paren_expr(line: &str, keyword_len: usize) -> String {
    let rest = &line[keyword_len..];
    if let Some(open) = rest.find('(') {
        let after = &rest[open + 1..];
        let mut depth = 1i32;
        for (i, ch) in after.char_indices() {
            if ch == '(' { depth += 1; }
            if ch == ')' { depth -= 1; }
            if depth == 0 {
                return after[..i].trim().to_string();
            }
        }
    }
    String::new()
}

// ── utility ───────────────────────────────────────────────────────────────────

fn extract_call_args(line: &str) -> String {
    let start = line.find('(').map(|p| p + 1).unwrap_or(line.len());
    let end   = line.rfind(')').unwrap_or(line.len());
    if start >= end { return String::new(); }
    line[start..end].to_string()
}

fn split_call_args(args: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_q = false;
    for ch in args.chars() {
        if ch == '"' { in_q = !in_q; }
        if !in_q {
            if ch == '(' { depth += 1; }
            if ch == ')' { depth -= 1; }
            if ch == ',' && depth == 0 {
                result.push(current.trim().to_string());
                current.clear();
                continue;
            }
        }
        current.push(ch);
    }
    if !current.trim().is_empty() {
        result.push(current.trim().to_string());
    }
    result
}

//! Pascal language executor — port of `languages/pascal.py`.
//!
//! Supports: var declarations, begin/end, if/then/else, while/do,
//! for/to/downto/do, repeat/until, write/writeln, readln, functions, procedures.

use crate::context::{ControlFlow, ExecContext};

pub fn execute_pascal(ctx: &mut ExecContext, command: &str) -> ControlFlow {
    let line = command.trim();
    if line.is_empty() {
        return ControlFlow::Continue;
    }

    let upper = line.to_uppercase();
    let upper_stripped = upper.trim_end_matches(';').trim();

    // ── Comments ──────────────────────────────────────────────────────────
    if upper_stripped.starts_with("//")
        || upper_stripped.starts_with("{")
        || upper_stripped.starts_with("(*")
    {
        return ControlFlow::Continue;
    }

    // ── Program / uses (no-op) ────────────────────────────────────────────
    if upper_stripped.starts_with("PROGRAM")
        || upper_stripped.starts_with("USES")
        || upper_stripped == "UNIT"
        || upper_stripped == "INTERFACE"
        || upper_stripped == "IMPLEMENTATION"
    {
        return ControlFlow::Continue;
    }

    // ── var / const section headers ───────────────────────────────────────
    if upper_stripped == "VAR" {
        ctx.pascal_in_var_section = true;
        ctx.pascal_in_const_section = false;
        return ControlFlow::Continue;
    }
    if upper_stripped == "CONST" {
        ctx.pascal_in_const_section = true;
        ctx.pascal_in_var_section = false;
        return ControlFlow::Continue;
    }

    // ── begin / end ───────────────────────────────────────────────────────
    if upper_stripped == "BEGIN" {
        ctx.pascal_in_var_section = false;
        ctx.pascal_in_const_section = false;
        return ControlFlow::Continue;
    }
    if upper_stripped == "END" || upper_stripped == "END." || upper_stripped == "END;" {
        // Check if closing a while loop
        if let Some((while_idx, ref cond)) = ctx.while_stack.last().cloned() {
            let val = ctx.eval_f64(&cond);
            if val != 0.0 {
                ctx.while_stack.pop(); // will be re-pushed by WHILE handler
                return ControlFlow::Jump(while_idx);
            }
            ctx.while_stack.pop();
            return ControlFlow::Continue;
        }
        // Check if closing a for loop
        if let Some(ref _frame) = ctx.for_stack.last() {
            return match ctx.process_next(None) {
                Some(loop_start) => ControlFlow::Jump(loop_start),
                None => ControlFlow::Continue,
            };
        }
        return ControlFlow::End;
    }

    // ── var declaration: x : integer; ────────────────────────────────────
    if ctx.pascal_in_var_section {
        if let Some(col) = line.find(':') {
            let names_part = &line[..col];
            for name in names_part.split(',') {
                let n = name.trim().to_uppercase();
                if n.chars().next().map_or(false, |c| c.is_alphabetic()) {
                    ctx.set_var(&n, 0.0);
                }
            }
            return ControlFlow::Continue;
        }
    }

    // ── const declaration: X = 10; ───────────────────────────────────────
    if ctx.pascal_in_const_section {
        if let Some(eq) = line.find('=') {
            let name = line[..eq].trim().to_uppercase();
            let expr = line[eq+1..].trim().trim_end_matches(';').trim();
            let v = ctx.eval_f64(expr);
            ctx.set_var(&name, v);
            return ControlFlow::Continue;
        }
    }

    // ── write / writeln ───────────────────────────────────────────────────
    if upper_stripped.starts_with("WRITELN") || upper_stripped.starts_with("WRITE") {
        return pascal_write(ctx, line, upper_stripped.starts_with("WRITELN"));
    }

    // ── readln / read ─────────────────────────────────────────────────────
    if upper_stripped.starts_with("READLN") || upper_stripped.starts_with("READ") {
        return pascal_readln(ctx, line);
    }

    // ── clrscr ────────────────────────────────────────────────────────────
    if upper_stripped == "CLRSCR" || upper_stripped == "CLRSCR()" {
        ctx.turtle.clear_screen();
        ctx.text_lines.clear();
        return ControlFlow::Continue;
    }

    // ── if … then ─────────────────────────────────────────────────────────
    // (must be checked BEFORE `:=` assignment, since `for i := ...` contains `:=`)
    if upper_stripped.starts_with("IF ") {
        return pascal_if(ctx, line.trim_end_matches(';').trim());
    }

    // ── while … do ────────────────────────────────────────────────────────
    if upper_stripped.starts_with("WHILE ") {
        return pascal_while(ctx, line.trim_end_matches(';').trim());
    }

    // ── for … do ──────────────────────────────────────────────────────────
    if upper_stripped.starts_with("FOR ") {
        return pascal_for(ctx, line.trim_end_matches(';').trim());
    }

    // ── assignment: x := expr ─────────────────────────────────────────────
    if let Some(pos) = line.find(":=") {
        let lhs = line[..pos].trim().to_uppercase();
        let rhs = line[pos+2..].trim().trim_end_matches(';').trim();

        // Array: arr[i] := val
        if lhs.ends_with(']') {
            if let Some(ob) = lhs.find('[') {
                let name = lhs[..ob].trim().to_string();
                let idx_str = &lhs[ob+1..lhs.len()-1];
                let idx = ctx.eval_f64(idx_str) as usize;
                let val = ctx.eval_f64(rhs);
                ctx.set_array(&name, idx, val);
                return ControlFlow::Continue;
            }
        }

        if rhs.starts_with('\'') || rhs.starts_with('"') {
            let sv = rhs.trim_matches(|c| c == '\'' || c == '"').to_string();
            ctx.set_str(&lhs, sv);
        } else {
            let v = ctx.eval_f64(rhs);
            ctx.set_var(&lhs, v);
        }
        return ControlFlow::Continue;
    }

    // ── repeat … until ────────────────────────────────────────────────────
    if upper_stripped == "REPEAT" {
        ctx.do_stack.push(ctx.line_idx);
        return ControlFlow::Continue;
    }
    if upper_stripped.starts_with("UNTIL ") {
        let cond = line[6..].trim().trim_end_matches(';').trim();
        let v = ctx.eval_f64(cond);
        if v == 0.0 {
            if let Some(rep_idx) = ctx.do_stack.last().copied() {
                return ControlFlow::Jump(rep_idx + 1);
            }
        } else {
            ctx.do_stack.pop();
        }
        return ControlFlow::Continue;
    }

    // ── procedure / function definitions ─────────────────────────────────
    if upper_stripped.starts_with("PROCEDURE ") || upper_stripped.starts_with("FUNCTION ") {
        return pascal_proc(ctx, line);
    }

    // ── procedure call or standalone expression ───────────────────────────
    if line.contains('(') {
        let name = line.split('(').next().unwrap_or("").trim().to_uppercase();
        if ctx.subs.contains_key(&name) {
            let args_str = line.split('(').nth(1).unwrap_or("").trim_end_matches(')').trim_end_matches(';').trim();
            let args: Vec<f64> = args_str.split(',')
                .map(|a| ctx.eval_f64(a.trim()))
                .collect();
            return call_sub(ctx, &name, &args);
        }
    }

    ControlFlow::Continue
}

// ── write / writeln ───────────────────────────────────────────────────────────

fn pascal_write(ctx: &mut ExecContext, line: &str, newline: bool) -> ControlFlow {
    let inner = extract_parens(line);
    let mut out = String::new();
    for part in split_args(&inner) {
        let part = part.trim();
        if part.starts_with('\'') || part.starts_with('"') {
            out.push_str(part.trim_matches(|c| c == '\'' || c == '"'));
        } else {
            // Try string variable first, then numeric
            let sval = ctx.get_str(&part.to_uppercase());
            if !sval.is_empty() {
                out.push_str(&sval);
            } else {
                let v = ctx.eval_f64(part);
                if v == v.floor() && v.abs() < 1e15 { out.push_str(&format!("{}", v as i64)); }
                else { out.push_str(&format!("{v}")); }
            }
        }
    }
    if newline { out.push('\n'); }
    ctx.emit(&out);
    ControlFlow::Continue
}

// ── readln ────────────────────────────────────────────────────────────────────

fn pascal_readln(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    let inner = extract_parens(line);
    let var = inner.trim().to_uppercase();
    if !var.is_empty() {
        ctx.request_input("", &var, true);
        return ControlFlow::WaitInput;
    }
    ControlFlow::Continue
}

// ── if / then / else ─────────────────────────────────────────────────────────

fn pascal_if(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    let upper = line.to_uppercase();
    let then_pos = match find_word(&upper, "THEN") {
        Some(p) => p,
        None => return ControlFlow::Continue,
    };
    let cond_str = &line[3..then_pos].trim();
    let rest    = &line[then_pos + 4..].trim();

    let val = ctx.eval_f64(cond_str);
    let (then_part, else_part) = split_then_else(rest);

    if val != 0.0 {
        execute_pascal(ctx, then_part.trim())
    } else if let Some(ep) = else_part {
        execute_pascal(ctx, ep.trim())
    } else {
        ControlFlow::Continue
    }
}

fn find_word(hay: &str, needle: &str) -> Option<usize> {
    let nlen = needle.len();
    for i in 0..=hay.len().saturating_sub(nlen) {
        if hay[i..].starts_with(needle) {
            let before = i == 0 || !hay[i-1..i].chars().next().unwrap_or(' ').is_alphanumeric();
            let after  = i + nlen >= hay.len() || !hay[i+nlen..i+nlen+1].chars().next().unwrap_or(' ').is_alphanumeric();
            if before && after { return Some(i); }
        }
    }
    None
}

fn split_then_else(rest: &str) -> (&str, Option<&str>) {
    let upper = rest.to_uppercase();
    if let Some(ep) = find_word(&upper, "ELSE") {
        (&rest[..ep].trim_end_matches(';').trim(), Some(&rest[ep+4..].trim()))
    } else {
        (rest.trim_end_matches(';').trim(), None)
    }
}

// ── while … do ───────────────────────────────────────────────────────────────

fn pascal_while(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    let upper = line.to_uppercase();
    let do_pos = match find_word(&upper, "DO") {
        Some(p) => p,
        None => return ControlFlow::Continue,
    };
    let cond = &line[6..do_pos].trim();
    let val = ctx.eval_f64(cond);
    if val == 0.0 {
        // Skip to matching end (line-based: just skip this line)
        return ControlFlow::Continue; // body handled by executor loop
    }
    ctx.while_stack.push((ctx.line_idx, cond.to_string()));
    ControlFlow::Continue
}

// ── for … do ─────────────────────────────────────────────────────────────────

fn pascal_for(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    // FOR var := start TO end DO
    let upper = line.to_uppercase();
    let assign_pos = match line.find(":=") { Some(p) => p, None => return ControlFlow::Continue };
    let to_pos     = match find_word(&upper, "TO").or_else(|| find_word(&upper, "DOWNTO")) {
        Some(p) => p, None => return ControlFlow::Continue,
    };
    let do_pos = match find_word(&upper, "DO") { Some(p) => p, None => return ControlFlow::Continue };

    let var_name = line[4..assign_pos].trim().to_uppercase();
    let start    = ctx.eval_f64(line[assign_pos+2..to_pos].trim());
    let downto   = upper[to_pos..].starts_with("DOWNTO");
    let to_len   = if downto { 6 } else { 2 };
    let end_val  = ctx.eval_f64(line[to_pos+to_len..do_pos].trim());
    let step     = if downto { -1.0 } else { 1.0 };

    ctx.set_var(&var_name, start);
    if (step > 0.0 && start > end_val) || (step < 0.0 && start < end_val) {
        return ControlFlow::Continue; // skip immediately
    }

    // Check for inline single-statement body after DO
    let rest_after_do = line[do_pos+2..].trim().trim_end_matches(';').trim();
    let rest_up = rest_after_do.to_uppercase();
    if !rest_after_do.is_empty() && rest_up != "BEGIN" {
        // Single-statement body on same line as FOR
        loop {
            execute_pascal(ctx, rest_after_do);
            let current = ctx.get_var(&var_name);
            let next_val = current + step;
            ctx.set_var(&var_name, next_val);
            let done = if step >= 0.0 { next_val > end_val } else { next_val < end_val };
            if done { break; }
        }
        return ControlFlow::Continue;
    }

    // Check if next line starts a BEGIN block or is a single-statement body
    let next_idx = ctx.line_idx + 1;
    if next_idx < ctx.program_lines.len() {
        let next_up = ctx.program_lines[next_idx].1.trim().to_uppercase();
        if !next_up.starts_with("BEGIN") {
            // Single-statement body on next line
            let body = ctx.program_lines[next_idx].1.clone();
            loop {
                execute_pascal(ctx, &body);
                let current = ctx.get_var(&var_name);
                let next_val = current + step;
                ctx.set_var(&var_name, next_val);
                let done = if step >= 0.0 { next_val > end_val } else { next_val < end_val };
                if done { break; }
            }
            return ControlFlow::Jump(next_idx + 1);
        }
    }

    // Multi-line body with BEGIN/END — use stack-based mechanism
    ctx.push_for(&var_name, end_val, step, ctx.line_idx);
    ControlFlow::Continue
}

// ── procedure / function definitions ─────────────────────────────────────────

fn pascal_proc(ctx: &mut ExecContext, header: &str) -> ControlFlow {
    let is_func = header.to_uppercase().starts_with("FUNCTION");
    let name_start = if is_func { 9 } else { 10 };
    let rest = &header[name_start..].trim();

    let (name, params) = if let Some(op) = rest.find('(') {
        let n = rest[..op].trim().to_uppercase();
        let p_str = &rest[op+1..rest.find(')').unwrap_or(rest.len())];
        let params: Vec<String> = p_str.split(';').flat_map(|seg| {
            let (names, _type) = seg.split_once(':').unwrap_or((seg, ""));
            names.split(',').map(|n| n.trim().to_uppercase()).filter(|n| !n.is_empty()).collect::<Vec<_>>()
        }).collect();
        (n, params)
    } else {
        let n = rest.split_whitespace().next().unwrap_or("").trim_matches(':').to_uppercase();
        (n, vec![])
    };

    // Collect body until END;
    let start = ctx.line_idx + 1;
    let mut body = Vec::new();
    let mut end_idx = ctx.program_lines.len();
    let mut depth = 0usize;
    for i in start..ctx.program_lines.len() {
        let (_, line) = &ctx.program_lines[i];
        let up = line.trim().to_uppercase();
        if up == "BEGIN"  { depth += 1; }
        if up == "END" || up == "END;" || up == "END." {
            if depth == 0 { end_idx = i; break; }
            depth -= 1;
        }
        body.push(line.clone());
    }

    ctx.subs.insert(name.clone(), crate::context::SubDef { name: name.clone(), params, body_lines: body });
    ControlFlow::Jump(end_idx + 1)
}

fn call_sub(ctx: &mut ExecContext, name: &str, args: &[f64]) -> ControlFlow {
    let sub = match ctx.subs.get(name).cloned() {
        Some(s) => s,
        None => return ControlFlow::Continue,
    };

    let saved: Vec<(String, f64)> = sub.params.iter().enumerate().map(|(i, p)| {
        let old = ctx.get_var(p);
        ctx.set_var(p, args.get(i).copied().unwrap_or(0.0));
        (p.clone(), old)
    }).collect();

    for line in &sub.body_lines.clone() {
        match execute_pascal(ctx, line) {
            ControlFlow::Continue => {}
            ControlFlow::Return | ControlFlow::End => break,
            cf => return cf,
        }
    }

    for (p, v) in saved { ctx.set_var(&p, v); }
    ControlFlow::Continue
}

// ── utilities ─────────────────────────────────────────────────────────────────

fn extract_parens(line: &str) -> String {
    let start = line.find('(').map(|p| p + 1).unwrap_or(line.len());
    let end   = line.rfind(')').unwrap_or(line.len());
    if start >= end { return String::new(); }
    line[start..end].to_string()
}

fn split_args(args: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_q  = false;
    for ch in args.chars() {
        if ch == '\'' || ch == '"' { in_q = !in_q; }
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
    if !current.trim().is_empty() { result.push(current.trim().to_string()); }
    result
}

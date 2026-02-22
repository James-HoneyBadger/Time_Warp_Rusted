//! Logo language executor — port of `languages/logo.py`.
//!
//! Supports turtle graphics, procedures (TO … END), REPEAT, loops,
//! conditionals and arithmetic expressions.

use crate::context::{ControlFlow, ExecContext, SubDef};
use tw_graphics::turtle::parse_color;
use std::collections::HashMap;

// ── entry point ───────────────────────────────────────────────────────────────

pub fn execute_logo(ctx: &mut ExecContext, command: &str) -> ControlFlow {
    let tokens = tokenize_logo(command);
    if tokens.is_empty() {
        return ControlFlow::Continue;
    }
    exec_tokens(ctx, &tokens, 0).0
}

// Tokenise a Logo line into a list of strings.
fn tokenize_logo(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;

    for ch in text.chars() {
        match ch {
            '[' => {
                if depth > 0 { current.push(ch); }
                depth += 1;
                if depth == 1 { current.push('['); }
            }
            ']' => {
                depth -= 1;
                if depth >= 1 {
                    current.push(ch);
                } else {
                    current.push(']');
                    let t = current.trim().to_string();
                    if !t.is_empty() { tokens.push(t); }
                    current.clear();
                }
            }
            ' ' | '\t' if depth == 0 => {
                let t = current.trim().to_string();
                if !t.is_empty() { tokens.push(t); }
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    let t = current.trim().to_string();
    if !t.is_empty() { tokens.push(t); }
    tokens
}

// Execute a token stream starting at `start`.  Returns (ControlFlow, tokens_consumed).
fn exec_tokens(ctx: &mut ExecContext, tokens: &[String], start: usize) -> (ControlFlow, usize) {
    // Guard against stack overflow from deep recursion
    ctx.call_depth += 1;
    if ctx.call_depth > ctx.max_call_depth {
        ctx.call_depth -= 1;
        ctx.emit("⚠️ Maximum recursion depth exceeded\n");
        return (ControlFlow::End, tokens.len() - start);
    }
    let result = exec_tokens_inner(ctx, tokens, start);
    ctx.call_depth -= 1;
    result
}

fn exec_tokens_inner(ctx: &mut ExecContext, tokens: &[String], start: usize) -> (ControlFlow, usize) {
    let mut i = start;
    while i < tokens.len() {
        let tok = tokens[i].to_uppercase();
        match tok.as_str() {
            // ── Turtle movement ──────────────────────────────────────────
            "FORWARD" | "FD" => {
                let (v, adv) = get_number(ctx, tokens, i + 1);
                ctx.turtle.forward(v);
                i += 1 + adv;
            }
            "BACK" | "BK" | "BACKWARD" => {
                let (v, adv) = get_number(ctx, tokens, i + 1);
                ctx.turtle.backward(v);
                i += 1 + adv;
            }
            "LEFT" | "LT" => {
                let (v, adv) = get_number(ctx, tokens, i + 1);
                ctx.turtle.left(v);
                i += 1 + adv;
            }
            "RIGHT" | "RT" => {
                let (v, adv) = get_number(ctx, tokens, i + 1);
                ctx.turtle.right(v);
                i += 1 + adv;
            }
            "PENUP" | "PU" => { ctx.turtle.pen_up(); i += 1; }
            "PENDOWN" | "PD" => { ctx.turtle.pen_down_cmd(); i += 1; }
            "HOME" => { ctx.turtle.home(); i += 1; }
            "CLEARSCREEN" | "CS" | "CLEAR" => { ctx.turtle.clear_screen(); i += 1; }
            "HIDETURTLE" | "HT" => { ctx.turtle.hide_turtle(); i += 1; }
            "SHOWTURTLE" | "ST" => { ctx.turtle.show_turtle(); i += 1; }

            // ── Positioning ──────────────────────────────────────────────
            "SETXY" | "SETPOSITION" | "SETPOS" => {
                // Support both `SETXY x y` and `SETPOSITION [x y]`
                if i + 1 < tokens.len() && tokens[i + 1].starts_with('[') {
                    let inner = tokens[i + 1].trim_matches(|c| c == '[' || c == ']');
                    let nums: Vec<f64> = inner.split_whitespace()
                        .filter_map(|s| s.parse::<f64>().ok())
                        .collect();
                    if nums.len() >= 2 {
                        ctx.turtle.set_pos(nums[0], nums[1]);
                    }
                    i += 2;
                } else {
                    let (x, ax) = get_number(ctx, tokens, i + 1);
                    let (y, ay) = get_number(ctx, tokens, i + 1 + ax);
                    ctx.turtle.set_pos(x, y);
                    i += 1 + ax + ay;
                }
            }
            "SETX" => {
                let (v, adv) = get_number(ctx, tokens, i + 1);
                ctx.turtle.set_x(v);
                i += 1 + adv;
            }
            "SETY" => {
                let (v, adv) = get_number(ctx, tokens, i + 1);
                ctx.turtle.set_y(v);
                i += 1 + adv;
            }
            "SETHEADING" | "SETH" => {
                let (v, adv) = get_number(ctx, tokens, i + 1);
                ctx.turtle.set_heading(v);
                i += 1 + adv;
            }

            // ── Pen colour / width ───────────────────────────────────────
            "SETPENCOLOR" | "SETPC" | "SETCOLOR" => {
                let (adv, color) = get_color(ctx, tokens, i + 1);
                if let Some(c) = color { ctx.turtle.set_pen_color(c); }
                i += 1 + adv;
            }
            "SETBGCOLOR" | "SETBG" => {
                let (adv, color) = get_color(ctx, tokens, i + 1);
                if let Some(c) = color { ctx.turtle.set_bg_color(c); }
                i += 1 + adv;
            }
            "SETPENWIDTH" | "SETPW" | "PENWIDTH" | "SETPENSIZE" => {
                let (v, adv) = get_number(ctx, tokens, i + 1);
                ctx.turtle.set_pen_width(v);
                i += 1 + adv;
            }

            // ── Arc / dot ────────────────────────────────────────────────
            "ARC" => {
                let (radius, ar) = get_number(ctx, tokens, i + 1);
                let (angle,  aa) = get_number(ctx, tokens, i + 1 + ar);
                ctx.turtle.arc(radius, angle);
                i += 1 + ar + aa;
            }
            "DOT" => {
                let (r, adv) = get_number(ctx, tokens, i + 1);
                ctx.turtle.dot(r, None);
                i += 1 + adv;
            }

            // ── Fill ─────────────────────────────────────────────────────
            "BEGINFILL" | "BEGIN_FILL" | "FILLED" => {
                // Optional fill colour argument; defaults to current pen colour
                let (adv, color) = get_color(ctx, tokens, i + 1);
                let fill_c = color.unwrap_or(ctx.turtle.pen_color);
                ctx.turtle.begin_fill(fill_c);
                i += 1 + adv;
            }
            "ENDFILL" | "END_FILL" => {
                ctx.turtle.end_fill();
                i += 1;
            }

            // ── Label ────────────────────────────────────────────────────
            "LABEL" => {
                let text = if i + 1 < tokens.len() {
                    let t = tokens[i + 1].trim_matches('"').to_string();
                    i += 2;
                    t
                } else { i += 1; String::new() };
                ctx.turtle.label(&text, 14);
            }

            // ── Print ────────────────────────────────────────────────────
            "PRINT" | "SHOW" | "TYPE" => {
                if i + 1 >= tokens.len() {
                    ctx.emit("\n");
                    i += 1;
                } else {
                    let next = &tokens[i + 1];
                    if next.starts_with('[') && next.ends_with(']') {
                        // Bracket text: PRINT [hello world]
                        let text = strip_brackets(next);
                        ctx.emit(&format!("{text}\n"));
                        i += 2;
                    } else if next.starts_with('"') {
                        // Quoted string (single token)
                        ctx.emit(&format!("{}\n", next.trim_matches('"')));
                        i += 2;
                    } else {
                        // Expression (possibly multi-token): PRINT :N * 2
                        let (val, adv) = eval_logo_expr(ctx, tokens, i + 1);
                        if adv == 0 {
                            ctx.emit("\n");
                            i += 1;
                        } else {
                            let s = if val == val.floor() && val.abs() < 1e15 {
                                format!("{}", val as i64)
                            } else {
                                format!("{val}")
                            };
                            ctx.emit(&format!("{s}\n"));
                            i += 1 + adv;
                        }
                    }
                }
            }

            // ── Variables ────────────────────────────────────────────────
            "MAKE" => {
                // MAKE "varname value
                if i + 2 < tokens.len() {
                    let name = tokens[i + 1].trim_matches('"').to_uppercase();
                    let raw = &tokens[i + 2];
                    // If the value looks like a quoted word (Logo `"word`), store
                    // it as a string variable so colour names survive round-trips
                    // through `:VAR` references.
                    let bare = raw.trim_matches('"');
                    if raw.starts_with('"') && bare.parse::<f64>().is_err() {
                        // String value — store in both string and numeric tables
                        ctx.set_str(&name, bare.to_string());
                        ctx.set_var(&name, 0.0); // numeric fallback
                        i += 3;
                    } else if raw.starts_with('[') {
                        // Bracket-list value (e.g. `MAKE "C [255 0 0]`) — store as string
                        ctx.set_str(&name, raw.clone());
                        ctx.set_var(&name, 0.0);
                        i += 3;
                    } else {
                        let (v, adv) = get_number(ctx, tokens, i + 2);
                        ctx.set_var(&name, v);
                        i += 2 + adv;
                    }
                } else { i += 1; }
            }

            // ── REPEAT ───────────────────────────────────────────────────
            "REPEAT" => {
                let (count, adv) = get_number(ctx, tokens, i + 1);
                let block_idx = i + 1 + adv;
                if block_idx < tokens.len() {
                    let block = tokens[block_idx].clone();
                    let inner = strip_brackets(&block);
                    let inner_toks = tokenize_logo(&inner);
                    for rep in 0..count as usize {
                        ctx.iteration_count += 1;
                        if ctx.iteration_count > ctx.max_iterations {
                            ctx.emit("⚠️ Maximum iteration count exceeded\n");
                            return (ControlFlow::End, tokens.len() - start);
                        }
                        // Set REPCOUNT variable
                        ctx.set_var("REPCOUNT", (rep + 1) as f64);
                        match exec_tokens(ctx, &inner_toks, 0).0 {
                            ControlFlow::Continue => {}
                            cf => return (cf, tokens.len() - start),
                        }
                    }
                    i = block_idx + 1;
                } else { i += 1; }
            }

            // ── FOREVER ──────────────────────────────────────────────────
            "FOREVER" => {
                let block_idx = i + 1;
                if block_idx < tokens.len() {
                    let block = tokens[block_idx].clone();
                    let inner = strip_brackets(&block);
                    let inner_toks = tokenize_logo(&inner);
                    loop {
                        ctx.iteration_count += 1;
                        if ctx.iteration_count > ctx.max_iterations { break; }
                        match exec_tokens(ctx, &inner_toks, 0).0 {
                            ControlFlow::Continue => {}
                            cf => return (cf, tokens.len() - start),
                        }
                    }
                    i = block_idx + 1;
                } else { i += 1; }
            }

            // ── IF / IFELSE ───────────────────────────────────────────────
            "IF" => {
                let (cond, adv) = eval_logo_expr(ctx, tokens, i + 1);
                let then_idx = i + 1 + adv;
                if then_idx < tokens.len() {
                    let then_block = &tokens[then_idx];
                    if cond != 0.0 {
                        let inner = strip_brackets(then_block);
                        let inner_toks = tokenize_logo(&inner);
                        match exec_tokens(ctx, &inner_toks, 0).0 {
                            ControlFlow::Continue => {}
                            cf => return (cf, tokens.len() - start),
                        }
                    }
                    i = then_idx + 1;
                } else { i += 1; }
            }
            "IFELSE" => {
                let (cond, adv) = eval_logo_expr(ctx, tokens, i + 1);
                let then_idx = i + 1 + adv;
                let else_idx = then_idx + 1;
                if else_idx < tokens.len() {
                    let chosen = if cond != 0.0 { &tokens[then_idx] } else { &tokens[else_idx] };
                    let inner = strip_brackets(chosen);
                    let inner_toks = tokenize_logo(&inner);
                    match exec_tokens(ctx, &inner_toks, 0).0 {
                        ControlFlow::Continue => {}
                        cf => return (cf, tokens.len() - start),
                    }
                    i = else_idx + 1;
                } else { i += 1; }
            }

            // ── TO … END (procedure definition) ──────────────────────────
            "TO" => {
                // Collect tokens until END
                let (consumed, cf) = collect_to_definition(ctx, tokens, i);
                if consumed > 0 {
                    i += consumed;
                } else {
                    return (cf, tokens.len() - start);
                }
            }

            "END" => {
                return (ControlFlow::Return, i - start + 1);
            }

            "STOP" => {
                // Exit current procedure (not the whole program)
                return (ControlFlow::Return, i - start + 1);
            }

            // ── WAIT ─────────────────────────────────────────────────────
            "WAIT" | "BYE" => {
                return (ControlFlow::End, i - start + 1);
            }

            // ── GPIO / IoT Commands ──────────────────────────────────────
            "PINMODE" => {
                let (pin, a1) = get_number(ctx, tokens, i + 1);
                let mode = if i + 1 + a1 < tokens.len() {
                    tokens[i + 1 + a1].to_uppercase()
                } else { "OUTPUT".to_string() };
                ctx.emit(&format!("GPIO:PINMODE {} {}\n", pin as u8, mode));
                i += 2 + a1;
            }
            "DIGITALWRITE" | "SETPIN" => {
                let (pin, a1) = get_number(ctx, tokens, i + 1);
                let (val, a2) = get_number(ctx, tokens, i + 1 + a1);
                ctx.emit(&format!("GPIO:WRITE {} {}\n", pin as u8, val as u8));
                i += 1 + a1 + a2;
            }
            "DIGITALREAD" | "READPIN" => {
                let (pin, a1) = get_number(ctx, tokens, i + 1);
                ctx.emit(&format!("GPIO:READ {}\n", pin as u8));
                i += 1 + a1;
            }
            "PWMWRITE" => {
                let (pin, a1) = get_number(ctx, tokens, i + 1);
                let (duty, a2) = get_number(ctx, tokens, i + 1 + a1);
                let norm = if duty > 1.0 { duty / 255.0 } else { duty };
                ctx.emit(&format!("GPIO:PWM {} {:.4}\n", pin as u8, norm));
                i += 1 + a1 + a2;
            }
            "GPIORESET" => {
                ctx.emit("GPIO:RESET\n");
                i += 1;
            }

            // ── Procedure call or `:var` reference ───────────────────────
            _ => {
                // :varname — return value (used in expressions, no-op here)
                if tok.starts_with(':') {
                    i += 1;
                    continue;
                }

                // User procedure call?
                let name = tok.clone();
                if let Some(sub) = ctx.subs.get(&name).cloned() {
                    // Bind parameters
                    let mut bound: HashMap<String, f64> = HashMap::new();
                    let mut adv = 0usize;
                    for p in &sub.params {
                        let (v, a) = get_number(ctx, tokens, i + 1 + adv);
                        bound.insert(p.clone(), v);
                        adv += a;
                    }
                    let saved: Vec<(String, f64)> = bound.iter().map(|(k, _)| {
                        let old = ctx.get_var(k);
                        (k.clone(), old)
                    }).collect();
                    for (k, v) in &bound { ctx.set_var(k, *v); }

                    let body_toks = tokenize_logo(&sub.body_lines.join(" "));
                    match exec_tokens(ctx, &body_toks, 0).0 {
                        ControlFlow::Continue | ControlFlow::Return => {}
                        cf => return (cf, tokens.len() - start),
                    }
                    for (k, v) in saved { ctx.set_var(&k, v); }
                    i += 1 + adv;
                } else {
                    i += 1;
                }
            }
        }
    }
    (ControlFlow::Continue, tokens.len() - start)
}

// ── TO … END collection ───────────────────────────────────────────────────────

fn collect_to_definition(ctx: &mut ExecContext, tokens: &[String], to_idx: usize) -> (usize, ControlFlow) {
    // tokens[to_idx] == "TO", tokens[to_idx+1] == name, tokens[to_idx+2..] == params until [body] or END
    if to_idx + 1 >= tokens.len() {
        return (0, ControlFlow::Continue);
    }
    let name = tokens[to_idx + 1].to_uppercase();
    let mut params = Vec::new();
    let mut body_toks = Vec::new();
    let mut i = to_idx + 2;
    let mut _found_end = false;

    // Collect parameter names (start with :)
    while i < tokens.len() && tokens[i].starts_with(':') {
        params.push(tokens[i][1..].to_uppercase());
        i += 1;
    }

    // Collect body tokens until END
    while i < tokens.len() {
        if tokens[i].to_uppercase() == "END" {
            i += 1;
            _found_end = true;
            break;
        }
        body_toks.push(tokens[i].clone());
        i += 1;
    }

    ctx.subs.insert(name.clone(), SubDef {
        name: name.clone(),
        params,
        body_lines: vec![body_toks.join(" ")],
    });
    ctx.labels.insert(name, to_idx);

    (i - to_idx, ControlFlow::Continue)
}

// ── Argument helpers ──────────────────────────────────────────────────────────

/// Get a numeric value from token stream at position `pos`.
/// Returns (value, tokens_consumed).
fn get_number(ctx: &ExecContext, tokens: &[String], pos: usize) -> (f64, usize) {
    if pos >= tokens.len() { return (0.0, 0); }
    let tok = &tokens[pos];
    // Logo :varname syntax
    if tok.starts_with(':') {
        let var = tok[1..].to_uppercase();
        return (ctx.get_var(&var), 1);
    }
    // Quoted number or expression in parens
    if tok.starts_with('(') {
        let inner = tok.trim_matches(|c| c == '(' || c == ')');
        return (ctx.eval_f64(inner), 1);
    }
    // Direct number
    if let Ok(v) = tok.parse::<f64>() {
        return (v, 1);
    }
    // Expression attempt
    (ctx.eval_f64(tok), 1)
}

/// Get a colour from the next 1–3 tokens.  Returns (tokens_consumed, color).
fn get_color(ctx: &ExecContext, tokens: &[String], pos: usize) -> (usize, Option<tw_graphics::turtle::Rgb>) {
    if pos >= tokens.len() { return (0, None); }
    let tok = &tokens[pos];

    // Strip Logo quote marks: `"RED"` → `RED`, `"RED` → `RED`, `"#FF0000` → `#FF0000`
    let bare = tok.trim_matches('"');

    // Resolve `:VAR` variable references — the variable may hold a palette index,
    // a colour name string, a hex string, or an [R G B] bracket list.
    if tok.starts_with(':') {
        let var = tok[1..].to_uppercase();

        // First check string variables — they may hold a colour name, hex, or [r g b]
        let sval = ctx.get_str(&var);
        if !sval.is_empty() {
            // Try bracket-list stored as string: "[255 0 0]"
            if sval.starts_with('[') {
                let inner = sval.trim_matches(|c: char| c == '[' || c == ']');
                let nums: Vec<u8> = inner.split_whitespace()
                    .filter_map(|s| s.parse::<u8>().ok())
                    .collect();
                if nums.len() >= 3 {
                    return (1, Some((nums[0], nums[1], nums[2])));
                }
            }
            if let Some(c) = parse_color(&sval) {
                return (1, Some(c));
            }
        }

        // Fall back to numeric variable → palette index
        let val = ctx.get_var(&var);
        if val >= 0.0 && val <= 255.0 {
            if let Some(c) = tw_graphics::turtle::default_palette_16(val as u8) {
                return (1, Some(c));
            }
        }

        return (1, None);
    }

    // Numeric palette index
    if let Ok(idx) = bare.parse::<u8>() {
        return (1, tw_graphics::turtle::default_palette_16(idx));
    }

    // Named colour or hex colour (single token)
    if let Some(c) = parse_color(bare) {
        return (1, Some(c));
    }

    // Bracket-list: [r g b]
    if tok.starts_with('[') {
        let inner = tok.trim_matches(|c| c == '[' || c == ']');
        let nums: Vec<u8> = inner.split_whitespace()
            .filter_map(|s| s.parse::<u8>().ok())
            .collect();
        if nums.len() >= 3 {
            return (1, Some((nums[0], nums[1], nums[2])));
        }
    }

    // Always consume the argument token even if we could not parse it,
    // so it does not leak into the token stream as an unrecognised command.
    (1, None)
}

fn strip_brackets(s: &str) -> String {
    let s = s.trim();
    if s.starts_with('[') && s.ends_with(']') {
        s[1..s.len()-1].trim().to_string()
    } else {
        s.to_string()
    }
}

/// Evaluate a multi-token Logo expression, stopping before bracket blocks.
/// Replaces `:VAR` references with their numeric values.
fn eval_logo_expr(ctx: &ExecContext, tokens: &[String], start: usize) -> (f64, usize) {
    let mut parts = Vec::new();
    let mut i = start;
    while i < tokens.len() {
        let tok = &tokens[i];
        if tok.starts_with('[') { break; }
        parts.push(tok.clone());
        i += 1;
    }
    if parts.is_empty() { return (0.0, 0); }
    let expr: String = parts.iter().map(|t| {
        if t.starts_with(':') {
            let var = t[1..].to_uppercase();
            format!("{}", ctx.get_var(&var))
        } else {
            t.clone()
        }
    }).collect::<Vec<_>>().join(" ");
    (ctx.eval_f64(&expr), i - start)
}

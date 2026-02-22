//! PILOT language executor — supports both classic (T: A: M: C: J: E:)
//! and verbose keyword syntax (PRINT, ACCEPT, MATCH/CASE/END, etc.).

use crate::context::{ControlFlow, ExecContext};

/// Known verbose keywords — checked at the start of a line (case-insensitive).
const VERBOSE_KEYWORDS: &[&str] = &[
    "REMARK", "PRINT", "ACCEPT", "COMPUTE", "MATCH", "CASE", "DEFAULT",
    "END", "JUMP", "TU", "RULE", "RETURN", "STOP",
];

/// Check whether a line uses verbose keyword syntax (no colon required).
fn is_verbose_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('*') {
        return false;
    }
    let first_word = trimmed.split_whitespace().next().unwrap_or("");
    let upper = first_word.trim_end_matches(':').to_uppercase();
    VERBOSE_KEYWORDS.contains(&upper.as_str())
}

// ══════════════════════════════════════════════════════════════════════════════
// Entry point
// ══════════════════════════════════════════════════════════════════════════════

pub fn execute_pilot(ctx: &mut ExecContext, command: &str) -> ControlFlow {
    let cmd = command.trim();
    if cmd.is_empty() {
        return ControlFlow::Continue;
    }

    // Label definitions (*LABEL) — no-op at runtime
    if cmd.starts_with('*') {
        return ControlFlow::Continue;
    }

    if is_verbose_line(cmd) {
        return execute_verbose(ctx, cmd);
    }

    execute_classic(ctx, cmd)
}

// ══════════════════════════════════════════════════════════════════════════════
// Verbose keyword syntax
// ══════════════════════════════════════════════════════════════════════════════

fn execute_verbose(ctx: &mut ExecContext, cmd: &str) -> ControlFlow {
    let trimmed = cmd.trim();

    // Split into keyword and rest
    let (raw_keyword, rest) = match trimmed.find(char::is_whitespace) {
        Some(pos) => (trimmed[..pos].to_string(), trimmed[pos..].trim()),
        None      => (trimmed.to_string(), ""),
    };
    // Strip trailing colon from keyword (e.g. "DEFAULT:" → "DEFAULT")
    let keyword = raw_keyword.trim_end_matches(':').to_uppercase();

    match keyword.as_str() {
        // ── REMARK — comment ─────────────────────────────────────────────
        "REMARK" => ControlFlow::Continue,

        // ── PRINT — output with $VAR interpolation ───────────────────────
        "PRINT" => {
            if rest.is_empty() {
                ctx.emit("\n");
            } else if rest.ends_with(';') {
                // Semicolon suppresses newline
                let text = pilot_interpolate(ctx, &rest[..rest.len()-1]);
                ctx.emit(&text);
            } else {
                let text = pilot_interpolate(ctx, rest);
                ctx.emit(&format!("{text}\n"));
            }
            ControlFlow::Continue
        }

        // ── ACCEPT — read user input ─────────────────────────────────────
        "ACCEPT" => {
            // ACCEPT varname prompt text
            let mut parts = rest.splitn(2, char::is_whitespace);
            let var = parts.next().unwrap_or("").trim();
            let prompt_part = parts.next().unwrap_or("").trim();
            if var.is_empty() {
                ctx.request_input("? ", "INPUT", false);
            } else {
                let prompt_str = if prompt_part.is_empty() {
                    "? ".to_string()
                } else if prompt_part.ends_with('?') || prompt_part.ends_with(':') {
                    format!("{prompt_part} ")
                } else {
                    format!("{prompt_part}: ")
                };
                ctx.request_input(&prompt_str, var, false);
            }
            ControlFlow::WaitInput
        }

        // ── COMPUTE — variable assignment/calculation ────────────────────
        "COMPUTE" => {
            pilot_compute(ctx, rest);
            ControlFlow::Continue
        }

        // ── MATCH — begin a structured MATCH/CASE/DEFAULT/END block ──────
        "MATCH" => {
            pilot_match_block(ctx, rest)
        }

        // ── CASE / DEFAULT / END — encountered outside a MATCH block
        //    (stale after a JUMP into/out of a block), just skip them.
        "CASE" | "DEFAULT" | "END" => ControlFlow::Continue,

        // ── JUMP — unconditional jump to label ──────────────────────────
        "JUMP" => {
            let label = rest.trim();
            if !label.is_empty() {
                if let Some(idx) = ctx.resolve_label(label) {
                    return ControlFlow::Jump(idx);
                }
                ctx.emit(&format!("❌ JUMP: label not found: {label}\n"));
            }
            ControlFlow::Continue
        }

        // ── TU — call subroutine (Transfer to Use) ─────────────────────
        "TU" => {
            let name = rest.trim();
            if !name.is_empty() {
                if let Some(idx) = ctx.resolve_label(name) {
                    ctx.push_gosub(ctx.line_idx + 1);
                    return ControlFlow::Jump(idx);
                }
                ctx.emit(&format!("❌ TU: subroutine not found: {name}\n"));
            }
            ControlFlow::Continue
        }

        // ── RULE — subroutine definition (acts as label, skip at runtime)
        "RULE" => ControlFlow::Continue,

        // ── RETURN — return from subroutine ──────────────────────────────
        "RETURN" => {
            if let Some(ret) = ctx.pop_gosub() {
                ControlFlow::Jump(ret)
            } else {
                ControlFlow::End
            }
        }

        // ── STOP — halt execution ────────────────────────────────────────
        "STOP" => ControlFlow::End,

        _ => {
            ctx.emit(&format!("❌ Unknown PILOT keyword: {keyword}\n"));
            ControlFlow::Continue
        }
    }
}

// ── COMPUTE handler ──────────────────────────────────────────────────────────

fn pilot_compute(ctx: &mut ExecContext, rest: &str) {
    // Format: COMPUTE varname expression
    let mut parts = rest.splitn(2, char::is_whitespace);
    let var_name = parts.next().unwrap_or("").trim().trim_start_matches('$');
    let expr_str = parts.next().unwrap_or("0").trim();

    if var_name.is_empty() {
        ctx.emit("❌ COMPUTE requires: COMPUTE varname expression\n");
        return;
    }

    // Expand $VAR references in the expression for the evaluator
    let expanded = expand_dollar_vars(ctx, expr_str);

    // Try evaluating as numeric expression
    match ctx.eval_expr(&expanded) {
        Ok(val) => {
            ctx.set_var(var_name, val);
        }
        Err(_) => {
            // Numeric eval failed — store as string (with interpolation)
            let text = pilot_interpolate(ctx, expr_str);
            ctx.set_str(var_name, text);
        }
    }
}

/// Expand `$VAR` references in an expression to their values so the
/// expression evaluator can handle them.
fn expand_dollar_vars(ctx: &ExecContext, expr: &str) -> String {
    let mut result = String::with_capacity(expr.len());
    let chars: Vec<char> = expr.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        if chars[i] == '$' {
            i += 1; // skip $
            let start = i;
            while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            if i > start {
                let name: String = chars[start..i].iter().collect();
                let upper = name.to_uppercase();
                if let Some(&v) = ctx.variables.get(&upper) {
                    if v == v.floor() && v.abs() < 1e15 {
                        result.push_str(&format!("{}", v as i64));
                    } else {
                        result.push_str(&format!("{v}"));
                    }
                } else if let Some(sv) = ctx.string_vars.get(&upper) {
                    if let Ok(v) = sv.parse::<f64>() {
                        if v == v.floor() && v.abs() < 1e15 {
                            result.push_str(&format!("{}", v as i64));
                        } else {
                            result.push_str(&format!("{v}"));
                        }
                    } else {
                        result.push_str(sv);
                    }
                } else {
                    result.push('0');
                }
            } else {
                result.push('$');
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

// ── MATCH / CASE / DEFAULT / END block handler ──────────────────────────────

fn pilot_match_block(ctx: &mut ExecContext, rest: &str) -> ControlFlow {
    // Get the match value
    let expanded = expand_dollar_vars(ctx, rest.trim());
    let match_val = ctx.eval_f64(&expanded);
    let match_str = pilot_interpolate(ctx, rest.trim());

    // Scan forward to find CASE/DEFAULT/END structure
    let start_idx = ctx.line_idx + 1;
    let total = ctx.program_lines.len();

    let mut depth = 0i32;
    let mut found_branch: Option<usize> = None;
    let mut default_branch: Option<usize> = None;
    let mut end_idx: Option<usize> = None;

    let mut scan = start_idx;
    while scan < total {
        let line = ctx.program_lines[scan].1.trim().to_string();
        let upper = line.to_uppercase();
        let first_word = upper.split_whitespace().next().unwrap_or("");

        if first_word == "MATCH" {
            depth += 1;
        } else if first_word == "END" {
            if depth == 0 {
                end_idx = Some(scan);
                break;
            }
            depth -= 1;
        } else if depth == 0 {
            let trimmed_upper = upper.trim_start();
            if trimmed_upper.starts_with("CASE ") || trimmed_upper == "CASE" {
                // Extract case value: skip "CASE", strip trailing ":"
                let case_text = line.trim();
                let after_case = if case_text.len() > 4 {
                    case_text[4..].trim().trim_end_matches(':')
                } else {
                    ""
                };

                if found_branch.is_none() && case_matches(ctx, after_case, match_val, &match_str) {
                    found_branch = Some(scan + 1);
                }
            } else if trimmed_upper.starts_with("DEFAULT") {
                if found_branch.is_none() {
                    default_branch = Some(scan + 1);
                }
            }
        }
        scan += 1;
    }

    let branch_start = found_branch.or(default_branch);

    if let Some(branch) = branch_start {
        execute_branch(ctx, branch, total);
    }

    // Jump past END
    if let Some(end) = end_idx {
        return ControlFlow::Jump(end + 1);
    }

    // If we didn't find END, scan further
    let mut scan2 = if scan < total { scan + 1 } else { total };
    let mut d2 = 0i32;
    while scan2 < total {
        let upper = ctx.program_lines[scan2].1.trim().to_uppercase();
        let fw = upper.split_whitespace().next().unwrap_or("");
        if fw == "MATCH" { d2 += 1; }
        else if fw == "END" {
            if d2 == 0 { return ControlFlow::Jump(scan2 + 1); }
            d2 -= 1;
        }
        scan2 += 1;
    }

    ControlFlow::Continue
}

/// Check whether a CASE value matches the MATCH value.
fn case_matches(ctx: &ExecContext, case_str: &str, match_val: f64, match_str: &str) -> bool {
    let trimmed = case_str.trim();
    if trimmed.is_empty() {
        return false;
    }

    // Range match: [low-high]
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        let inner = &trimmed[1..trimmed.len()-1];
        // Find the separator dash (skip leading negative numbers)
        if let Some(dash_pos) = find_range_dash(inner) {
            let low_str = inner[..dash_pos].trim();
            let high_str = inner[dash_pos+1..].trim();
            if let (Ok(low), Ok(high)) = (low_str.parse::<f64>(), high_str.parse::<f64>()) {
                return match_val >= low && match_val <= high;
            }
        }
        return false;
    }

    // Variable / expression match: contains $
    if trimmed.contains('$') {
        let expanded = expand_dollar_vars(ctx, trimmed);
        if let Ok(case_val) = ctx.eval_expr(&expanded) {
            return (match_val - case_val).abs() < 1e-9;
        }
        let interp = pilot_interpolate(ctx, trimmed);
        return interp.trim().to_uppercase() == match_str.trim().to_uppercase();
    }

    // Numeric match
    if let Ok(case_val) = trimmed.parse::<f64>() {
        return (match_val - case_val).abs() < 1e-9;
    }

    // String match (case-insensitive)
    trimmed.to_uppercase() == match_str.trim().to_uppercase()
}

/// Find the dash separator in a range like "90-100" or "2-3".
/// Skips a leading minus (negative number).
fn find_range_dash(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let start = if !bytes.is_empty() && bytes[0] == b'-' { 1 } else { 0 };
    s[start..].find('-').map(|p| p + start)
}

/// Execute lines within a MATCH branch until hitting CASE/DEFAULT/END at depth 0.
fn execute_branch(ctx: &mut ExecContext, start: usize, total: usize) {
    let mut i = start;
    let mut depth = 0i32;

    while i < total {
        let line_text = ctx.program_lines[i].1.clone();
        let trimmed = line_text.trim();
        let upper_trimmed = trimmed.to_uppercase();
        let first_word = upper_trimmed.split_whitespace().next().unwrap_or("");

        // Track nested MATCH blocks
        if first_word == "MATCH" {
            depth += 1;
        }

        if depth == 0 {
            // Check for branch/block boundaries at our level
            if first_word == "CASE" || upper_trimmed.starts_with("DEFAULT") {
                return;
            }
            if first_word == "END" {
                return;
            }
        }

        if first_word == "END" && depth > 0 {
            depth -= 1;
        }

        // Execute this line
        let cf = execute_pilot(ctx, trimmed);
        match cf {
            ControlFlow::Continue => { i += 1; }
            ControlFlow::Jump(target) => { i = target; }
            ControlFlow::End => {
                ctx.line_idx = total;
                return;
            }
            ControlFlow::WaitInput => {
                // Input request queued — advance past this line
                i += 1;
            }
            ControlFlow::Gosub(target) => {
                ctx.push_gosub(i + 1);
                i = target;
            }
            ControlFlow::Return => {
                if let Some(ret) = ctx.pop_gosub() {
                    i = ret;
                } else {
                    return;
                }
            }
            ControlFlow::JumpLabel(label) => {
                if let Some(idx) = ctx.resolve_label(&label) {
                    i = idx;
                } else {
                    i += 1;
                }
            }
            ControlFlow::Error(e) => {
                ctx.emit(&format!("❌ {e}\n"));
                i += 1;
            }
        }
    }
}

// ── $VAR interpolation for PILOT verbose syntax ─────────────────────────────

/// Replace `$VAR` references in text with their values.
fn pilot_interpolate(ctx: &ExecContext, text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        if chars[i] == '$' {
            let start = i + 1;
            let mut end = start;
            while end < n && (chars[end].is_alphanumeric() || chars[end] == '_') {
                end += 1;
            }
            if end > start {
                let name: String = chars[start..end].iter().collect();
                let upper = name.to_uppercase();
                if let Some(&v) = ctx.variables.get(&upper) {
                    if v == v.floor() && v.abs() < 1e15 {
                        result.push_str(&format!("{}", v as i64));
                    } else {
                        result.push_str(&format!("{v}"));
                    }
                } else if let Some(sv) = ctx.string_vars.get(&upper) {
                    result.push_str(sv);
                } else {
                    // Unknown variable — keep original text
                    result.push('$');
                    result.push_str(&name);
                }
                i = end;
            } else {
                result.push('$');
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

// ══════════════════════════════════════════════════════════════════════════════
// Classic single-letter PILOT syntax (T: A: M: C: J: U: E: R: D:)
// ══════════════════════════════════════════════════════════════════════════════

fn execute_classic(ctx: &mut ExecContext, cmd: &str) -> ControlFlow {
    if cmd.len() < 2 {
        return ControlFlow::Continue;
    }

    // Find the colon separator
    let colon_pos = match cmd.find(':') {
        Some(p) if p >= 1 => p,
        _ => {
            ctx.emit(&format!("❌ Invalid PILOT command: {cmd}\n"));
            return ControlFlow::Continue;
        }
    };

    let prefix = cmd[..colon_pos].to_uppercase();
    let rest   = cmd[colon_pos + 1..].trim();

    // Extract base command letter and conditional suffix
    let (cmd_type, condition) = match prefix.len() {
        1 => (prefix.chars().next().unwrap(), None),
        2 => {
            let mut ch = prefix.chars();
            let base = ch.next().unwrap();
            let suf  = ch.next().unwrap();
            if suf == 'Y' || suf == 'N' {
                (base, Some(suf))
            } else {
                ctx.emit(&format!("❌ Invalid PILOT prefix: {prefix}\n"));
                return ControlFlow::Continue;
            }
        }
        _ => {
            ctx.emit(&format!("❌ Invalid PILOT prefix: {prefix}\n"));
            return ControlFlow::Continue;
        }
    };

    // Check conditional execution
    match condition {
        Some('Y') if !ctx.last_match => return ControlFlow::Continue,
        Some('N') if  ctx.last_match => return ControlFlow::Continue,
        _ => {}
    }

    match cmd_type {
        // T: — Type (output text with variable interpolation)
        'T' => {
            let text = ctx.interpolate(rest);
            ctx.emit(&format!("{text}\n"));
            ControlFlow::Continue
        }

        // A: — Accept (read user input into variable)
        'A' => {
            let var = rest.trim();
            if var.is_empty() {
                ctx.request_input("? ", "INPUT", false);
            } else {
                ctx.request_input("? ", var, false);
            }
            ControlFlow::WaitInput
        }

        // M: — Match (compare last input against pattern list)
        'M' => {
            let pattern = rest.trim();
            if pattern.is_empty() {
                ctx.last_match = false;
                return ControlFlow::Continue;
            }
            let last_input = ctx.last_input.trim().to_uppercase();
            let alternatives: Vec<&str> = pattern.split(',').collect();

            ctx.last_match = alternatives.iter().any(|alt| {
                let alt = alt.trim().to_uppercase();
                if alt.contains('*') {
                    wildcard_match(&alt, &last_input)
                } else {
                    last_input == alt
                }
            });
            ControlFlow::Continue
        }

        // Y: — Jump if last match succeeded
        'Y' => {
            if ctx.last_match {
                let label = rest.trim();
                if !label.is_empty() {
                    if let Some(idx) = ctx.resolve_label(label) {
                        return ControlFlow::Jump(idx);
                    }
                }
            }
            ControlFlow::Continue
        }

        // N: — Jump if last match failed
        'N' => {
            if !ctx.last_match {
                let label = rest.trim();
                if !label.is_empty() {
                    if let Some(idx) = ctx.resolve_label(label) {
                        return ControlFlow::Jump(idx);
                    }
                }
            }
            ControlFlow::Continue
        }

        // C: — Compute (variable assignment: VAR = expr)
        'C' => {
            if let Some(eq) = rest.find('=') {
                let var  = rest[..eq].trim().to_uppercase();
                let expr = rest[eq+1..].trim();
                let val  = ctx.eval_f64(expr);
                ctx.set_var(&var, val);
            } else {
                ctx.emit("❌ C: requires VAR = expr\n");
            }
            ControlFlow::Continue
        }

        // J: — Jump unconditionally to label
        'J' => {
            let label = rest.trim();
            if !label.is_empty() {
                if let Some(idx) = ctx.resolve_label(label) {
                    return ControlFlow::Jump(idx);
                }
                ctx.emit(&format!("❌ J: label not found: {label}\n"));
            }
            ControlFlow::Continue
        }

        // U: — Use (read variable value or compute)
        'U' => {
            if rest.contains('=') {
                return execute_classic(ctx, &format!("C:{rest}"));
            }
            let var = rest.trim().to_uppercase();
            let val = ctx.get_var(&var);
            ctx.emit(&format!("{val}\n"));
            ControlFlow::Continue
        }

        // E: — End (halt)
        'E' => ControlFlow::End,

        // R: — Remark
        'R' => ControlFlow::Continue,

        // D: — Display (same as T in many variants)
        'D' => {
            let text = ctx.interpolate(rest);
            ctx.emit(&format!("{text}\n"));
            ControlFlow::Continue
        }

        unknown => {
            ctx.emit(&format!("⚠️ Unknown PILOT command: {unknown}:\n"));
            ControlFlow::Continue
        }
    }
}

// ── wildcard matching ──────────────────────────────────────────────────────────

fn wildcard_match(pattern: &str, input: &str) -> bool {
    let pts: Vec<&str> = pattern.split('*').collect();
    let mut pos = 0usize;
    for (i, part) in pts.iter().enumerate() {
        if part.is_empty() { continue; }
        if i == 0 {
            if !input.starts_with(part) { return false; }
            pos = part.len();
        } else if i == pts.len() - 1 {
            if !input[pos..].ends_with(part) { return false; }
        } else {
            if let Some(found) = input[pos..].find(part) {
                pos += found + part.len();
            } else {
                return false;
            }
        }
    }
    true
}
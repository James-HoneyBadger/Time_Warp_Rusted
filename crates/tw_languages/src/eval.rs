//! Safe mathematical expression evaluator — port of `expression_evaluator.py`.
//!
//! Supports: +, -, *, /, MOD, ^ (power), comparisons (=, <, >, <=, >=, <>),
//! logical operators (AND, OR, NOT), parentheses, functions, variables, arrays.

use std::collections::HashMap;

// ── public entry point ────────────────────────────────────────────────────────

pub fn evaluate(
    expr: &str,
    vars: &HashMap<String, f64>,
    str_vars: &HashMap<String, String>,
    arrays: &HashMap<String, Vec<f64>>,
) -> Result<f64, String> {
    let tokens = tokenize(expr, vars, str_vars, arrays)?;
    let rpn    = shunting_yard(&tokens)?;
    eval_rpn(&rpn, arrays)
}

// ── token types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    /// A unary negation marker (distinct from binary minus).
    UnaryMinus,
    Op(char),          // + - * / ^ %
    Func(String),      // sin, cos, …
    Comma,
    LParen,
    RParen,
    // Derived comparison / logical operators stored as multi-char strings.
    Cmp(String),       // =, <, >, <=, >=, <>
    And,
    Or,
    Not,
}

// ── tokeniser ────────────────────────────────────────────────────────────────

fn tokenize(
    expr: &str,
    vars: &HashMap<String, f64>,
    str_vars: &HashMap<String, String>,
    arrays: &HashMap<String, Vec<f64>>,
) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = expr.chars().collect();
    let n = chars.len();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < n {
        let ch = chars[i];
        match ch {
            ' ' | '\t' => { i += 1; }

            '0'..='9' | '.' => {
                let start = i;
                while i < n && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                let v: f64 = s.parse().map_err(|_| format!("bad number: {s}"))?;
                tokens.push(Token::Number(v));
            }

            '"' => {
                // String literal — evaluate its length as 0, used by LEN()
                i += 1;
                let start = i;
                while i < n && chars[i] != '"' { i += 1; }
                let _s: String = chars[start..i].iter().collect();
                i += 1; // closing "
                // String not directly usable as a number; push 0
                tokens.push(Token::Number(0.0));
            }

            '+' => { tokens.push(Token::Op('+')); i += 1; }
            '-' => { tokens.push(Token::Op('-')); i += 1; }
            '*' => { tokens.push(Token::Op('*')); i += 1; }
            '/' => { tokens.push(Token::Op('/')); i += 1; }
            '^' => { tokens.push(Token::Op('^')); i += 1; }
            '%' => { tokens.push(Token::Op('%')); i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            ',' => { tokens.push(Token::Comma);  i += 1; }

            '<' => {
                if i + 1 < n && chars[i + 1] == '=' {
                    tokens.push(Token::Cmp("<=".into())); i += 2;
                } else if i + 1 < n && chars[i + 1] == '>' {
                    tokens.push(Token::Cmp("<>".into())); i += 2;
                } else {
                    tokens.push(Token::Cmp("<".into())); i += 1;
                }
            }
            '>' => {
                if i + 1 < n && chars[i + 1] == '=' {
                    tokens.push(Token::Cmp(">=".into())); i += 2;
                } else {
                    tokens.push(Token::Cmp(">".into())); i += 1;
                }
            }
            '=' => { tokens.push(Token::Cmp("=".into())); i += 1; }

            'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;
                while i < n && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '$') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                let upper = word.to_uppercase();

                // Check for AND / OR / NOT
                if upper == "AND" {
                    tokens.push(Token::And);
                } else if upper == "OR" {
                    tokens.push(Token::Or);
                } else if upper == "NOT" {
                    tokens.push(Token::Not);
                } else if upper == "MOD" {
                    tokens.push(Token::Op('%'));
                } else if is_function(&upper) {
                    tokens.push(Token::Func(upper));
                } else if i < n && chars[i] == '(' {
                    // User-defined function call — treat as known func if possible
                    tokens.push(Token::Func(upper));
                } else {
                    // Check for array: NAME(index)
                    if i < n && chars[i] == '(' {
                        // handled above as Func
                    }
                    // Variable — look up value
                    // Strip trailing $ for string length
                    if upper.ends_with('$') {
                        let base = &upper[..upper.len()-1];
                        let sv = str_vars.get(base).map(|s| s.len() as f64).unwrap_or(0.0);
                        tokens.push(Token::Number(sv));
                    } else {
                        let v = vars.get(&upper).copied().unwrap_or(0.0);
                        tokens.push(Token::Number(v));
                    }
                }
            }

            _ => { i += 1; } // skip unknown
        }
    }

    // Resolve unary minus: `-` after operator / LParen / start is unary
    let mut result = Vec::with_capacity(tokens.len());
    for (_idx, tok) in tokens.iter().enumerate() {
        if *tok == Token::Op('-') {
            let prev = result.last();
            let is_unary = match prev {
                None => true,
                Some(Token::Op(_))
                | Some(Token::Cmp(_))
                | Some(Token::LParen)
                | Some(Token::Comma)
                | Some(Token::And)
                | Some(Token::Or)
                | Some(Token::Not) => true,
                _ => false,
            };
            if is_unary {
                result.push(Token::UnaryMinus);
                continue;
            }
        }
        result.push(tok.clone());
    }

    let _ = arrays; // used via Func resolution
    Ok(result)
}

fn is_function(name: &str) -> bool {
    matches!(
        name,
        "SIN" | "COS" | "TAN" | "ASIN" | "ACOS" | "ATAN" | "ATAN2"
        | "SINH" | "COSH" | "TANH"
        | "SQRT" | "SQR" | "ABS" | "FLOOR" | "CEIL" | "ROUND"
        | "EXP" | "LOG" | "LOG10"
        | "INT" | "FIX" | "SGN"
        | "RAND" | "RND" | "RANDOM"
        | "MIN" | "MAX" | "POW"
        | "LEN" | "VAL" | "STR"
        | "CHR" | "ASC" | "MID" | "LEFT" | "RIGHT"
        | "PI" | "E"
    )
}

// ── Shunting-Yard to RPN ──────────────────────────────────────────────────────

fn precedence(tok: &Token) -> i32 {
    match tok {
        Token::Or                           => 1,
        Token::And                          => 2,
        Token::Not                          => 3,
        Token::Cmp(_)                       => 4,
        Token::Op('+') | Token::Op('-')     => 5,
        Token::Op('*') | Token::Op('/') | Token::Op('%') => 6,
        Token::Op('^')                      => 8,
        Token::UnaryMinus                   => 9,
        Token::Func(_)                      => 10,
        _                                   => 0,
    }
}

fn right_assoc(tok: &Token) -> bool {
    matches!(tok, Token::Op('^') | Token::UnaryMinus | Token::Not)
}

fn shunting_yard(tokens: &[Token]) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut ops:    Vec<Token> = Vec::new();

    for tok in tokens {
        match tok {
            Token::Number(_) => output.push(tok.clone()),

            Token::Func(_) => ops.push(tok.clone()),

            Token::Comma => {
                while ops.last().map_or(false, |t| *t != Token::LParen) {
                    output.push(ops.pop().unwrap());
                }
            }

            Token::LParen => ops.push(tok.clone()),

            Token::RParen => {
                while ops.last().map_or(false, |t| *t != Token::LParen) {
                    output.push(ops.pop().unwrap());
                }
                if ops.last() == Some(&Token::LParen) {
                    ops.pop();
                }
                // If a function is on top now, pop it too
                if matches!(ops.last(), Some(Token::Func(_))) {
                    output.push(ops.pop().unwrap());
                }
            }

            Token::Op(_)
            | Token::Cmp(_)
            | Token::And
            | Token::Or
            | Token::Not
            | Token::UnaryMinus => {
                while let Some(top) = ops.last() {
                    if *top == Token::LParen {
                        break;
                    }
                    let top_prec = precedence(top);
                    let cur_prec = precedence(tok);
                    if top_prec > cur_prec || (top_prec == cur_prec && !right_assoc(tok)) {
                        output.push(ops.pop().unwrap());
                    } else {
                        break;
                    }
                }
                ops.push(tok.clone());
            }
        }
    }
    while let Some(op) = ops.pop() {
        output.push(op);
    }
    Ok(output)
}

// ── RPN evaluator ─────────────────────────────────────────────────────────────

fn eval_rpn(tokens: &[Token], arrays: &HashMap<String, Vec<f64>>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    let pop = |stack: &mut Vec<f64>| -> Result<f64, String> {
        stack.pop().ok_or_else(|| "stack underflow".to_string())
    };

    for tok in tokens {
        match tok {
            Token::Number(v) => stack.push(*v),

            Token::UnaryMinus => {
                let a = pop(&mut stack)?;
                stack.push(-a);
            }

            Token::Not => {
                let a = pop(&mut stack)?;
                stack.push(if a == 0.0 { -1.0 } else { 0.0 });
            }

            Token::Op(op) => {
                let b = pop(&mut stack)?;
                let a = pop(&mut stack)?;
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => {
                        if b == 0.0 {
                            return Err("division by zero".to_string());
                        }
                        a / b
                    }
                    '%' => {
                        if b == 0.0 { 0.0 } else { a % b }
                    }
                    '^' => a.powf(b),
                    _ => return Err(format!("unknown op {op}")),
                };
                stack.push(result);
            }

            Token::Cmp(op) => {
                let b = pop(&mut stack)?;
                let a = pop(&mut stack)?;
                let result = match op.as_str() {
                    "="  => a == b,
                    "<"  => a < b,
                    ">"  => a > b,
                    "<=" => a <= b,
                    ">=" => a >= b,
                    "<>" | "!=" => a != b,
                    _ => false,
                };
                stack.push(if result { -1.0 } else { 0.0 });
            }

            Token::And => {
                let b = pop(&mut stack)?;
                let a = pop(&mut stack)?;
                stack.push(if a != 0.0 && b != 0.0 { -1.0 } else { 0.0 });
            }

            Token::Or => {
                let b = pop(&mut stack)?;
                let a = pop(&mut stack)?;
                stack.push(if a != 0.0 || b != 0.0 { -1.0 } else { 0.0 });
            }

            Token::Func(name) => {
                apply_function(name, &mut stack, arrays)?;
            }

            _ => {}
        }
    }

    stack.pop().ok_or_else(|| "empty expression".to_string())
}

fn apply_function(name: &str, stack: &mut Vec<f64>, arrays: &HashMap<String, Vec<f64>>) -> Result<(), String> {
    let pop1 = |s: &mut Vec<f64>| s.pop().ok_or_else(|| format!("{name}: missing arg"));
    let pop2 = |s: &mut Vec<f64>| {
        let b = s.pop().ok_or_else(|| format!("{name}: missing arg2"))?;
        let a = s.pop().ok_or_else(|| format!("{name}: missing arg1"))?;
        Ok::<(f64, f64), String>((a, b))
    };

    match name {
        "SIN"    => { let a = pop1(stack)?; stack.push(a.to_radians().sin()); }
        "COS"    => { let a = pop1(stack)?; stack.push(a.to_radians().cos()); }
        "TAN"    => { let a = pop1(stack)?; stack.push(a.to_radians().tan()); }
        "ASIN"   => { let a = pop1(stack)?; stack.push(a.asin().to_degrees()); }
        "ACOS"   => { let a = pop1(stack)?; stack.push(a.acos().to_degrees()); }
        "ATAN"   => { let a = pop1(stack)?; stack.push(a.atan().to_degrees()); }
        "ATAN2"  => { let (a, b) = pop2(stack)?; stack.push(a.atan2(b).to_degrees()); }
        "SINH"   => { let a = pop1(stack)?; stack.push(a.sinh()); }
        "COSH"   => { let a = pop1(stack)?; stack.push(a.cosh()); }
        "TANH"   => { let a = pop1(stack)?; stack.push(a.tanh()); }
        "SQRT" | "SQR" => { let a = pop1(stack)?; stack.push(a.abs().sqrt()); }
        "ABS"    => { let a = pop1(stack)?; stack.push(a.abs()); }
        "FLOOR"  => { let a = pop1(stack)?; stack.push(a.floor()); }
        "CEIL"   => { let a = pop1(stack)?; stack.push(a.ceil()); }
        "ROUND"  => { let a = pop1(stack)?; stack.push(a.round()); }
        "EXP"    => { let a = pop1(stack)?; stack.push(a.exp()); }
        "LOG"    => { let a = pop1(stack)?; stack.push(if a > 0.0 { a.ln() } else { 0.0 }); }
        "LOG10"  => { let a = pop1(stack)?; stack.push(if a > 0.0 { a.log10() } else { 0.0 }); }
        "INT"    => { let a = pop1(stack)?; stack.push(a.floor()); }
        "FIX"    => { let a = pop1(stack)?; stack.push(a.trunc()); }
        "SGN"    => { let a = pop1(stack)?; stack.push(if a > 0.0 { 1.0 } else if a < 0.0 { -1.0 } else { 0.0 }); }
        "RAND" | "RND" => { stack.push(rand_f64()); }
        "RANDOM" => { let a = pop1(stack)?; stack.push(if a > 0.0 { (rand_f64() * a).floor() } else { 0.0 }); }
        "MIN"    => { let (a, b) = pop2(stack)?; stack.push(a.min(b)); }
        "MAX"    => { let (a, b) = pop2(stack)?; stack.push(a.max(b)); }
        "POW"    => { let (a, b) = pop2(stack)?; stack.push(a.powf(b)); }
        "PI"     => { stack.push(std::f64::consts::PI); }
        "E"      => { stack.push(std::f64::consts::E); }
        "LEN"    => { let a = pop1(stack)?; stack.push(a); } // already resolved to len
        "VAL"    => { let a = pop1(stack)?; stack.push(a); }
        _        => {
            // Check if it's an array reference
            if let Some(arr) = arrays.get(name) {
                let idx = pop1(stack)? as usize;
                stack.push(arr.get(idx).copied().unwrap_or(0.0));
            } else {
                // Unknown function — pop arg, push 0
                let _ = stack.pop();
                stack.push(0.0);
            }
        }
    }
    Ok(())
}

/// Simple LCG-based deterministic "random" (good enough for educational use).
fn rand_f64() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    static SEED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let seed = SEED.load(std::sync::atomic::Ordering::Relaxed);
    let next = if seed == 0 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64)
            .unwrap_or(12345)
    } else {
        seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)
    };
    SEED.store(next, std::sync::atomic::Ordering::Relaxed);
    (next >> 33) as f64 / (u32::MAX as f64)
}

//! Forth language executor — port of `languages/forth.py`.
//!
//! Stack-based interpreter with a runtime dictionary.
//! Entry point: `ForthExecutor`.

use crate::context::ExecContext;
use std::collections::HashMap;

// ── Forth executor ────────────────────────────────────────────────────────────

pub struct ForthExecutor {
    pub stack:        Vec<i64>,
    pub return_stack: Vec<usize>,
    pub memory:       Vec<i64>,
    pub dictionary:   HashMap<String, Vec<String>>, // user-defined words → token list
    pub output:       String,
    /// DO/LOOP stack: (index, limit, token-position of DO)
    pub loop_stack:   Vec<(i64, i64, usize)>,
    compiling:        bool,
    new_word:         String,
    new_def:          Vec<String>,
    /// Call depth for user-defined words (prevents runaway recursion).
    call_depth:       u32,
    /// Iteration counter for BEGIN/AGAIN/UNTIL/REPEAT loops.
    iteration_count:  u64,
}

impl Default for ForthExecutor {
    fn default() -> Self {
        Self {
            stack:        Vec::new(),
            return_stack: Vec::new(),
            memory:       vec![0; 256],
            dictionary:   HashMap::new(),
            output:       String::new(),
            loop_stack:   Vec::new(),
            compiling:    false,
            new_word:     String::new(),
            new_def:      Vec::new(),
            call_depth:   0,
            iteration_count: 0,
        }
    }
}

impl ForthExecutor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Execute a line of Forth source, modifying turtle state on `ctx`.
    pub fn execute(&mut self, ctx: &mut ExecContext, line: &str) -> String {
        let saved_output = std::mem::take(&mut self.output);
        let tokens = self.tokenize_forth(line);

        let mut i = 0;
        while i < tokens.len() {
            let tok = &tokens[i];

            if self.compiling {
                if tok == ";" {
                    let word = self.new_word.clone();
                    let def = self.new_def.clone();
                    self.dictionary.insert(word, def);
                    self.new_word.clear();
                    self.new_def.clear();
                    self.compiling = false;
                } else {
                    self.new_def.push(tok.clone());
                }
                i += 1;
                continue;
            }

            if tok == ":" {
                // : name ... ;
                if i + 1 < tokens.len() {
                    self.new_word = tokens[i + 1].to_uppercase();
                    i += 2;
                    self.compiling = true;
                    self.new_def.clear();
                    continue;
                }
            }

            // Handle dot-quote string: ."text..."
            if tok.starts_with(".\"") {
                // The text (without the leading .") was captured by the tokenizer
                let text = &tok[2..];
                // Strip trailing " if present
                let text = text.strip_suffix('"').unwrap_or(text);
                self.output.push_str(text);
                i += 1;
                continue;
            }

            let upper = tok.to_uppercase();
            self.exec_word(ctx, &upper, &tokens, &mut i);
            i += 1;
        }

        let result = self.output.clone();
        self.output = saved_output;
        result
    }

    fn exec_word(&mut self, ctx: &mut ExecContext, word: &str, tokens: &[String], i: &mut usize) {
        match word {
            // Stack ops
            "DUP"    => { if let Some(&t) = self.stack.last() { self.stack.push(t); } }
            "DROP"   => { self.stack.pop(); }
            "SWAP"   => { let n = self.stack.len(); if n >= 2 { self.stack.swap(n-1, n-2); } }
            "OVER"   => { let n = self.stack.len(); if n >= 2 { self.stack.push(self.stack[n-2]); } }
            "ROT"    => {
                let n = self.stack.len();
                if n >= 3 {
                    let a = self.stack.remove(n-3);
                    self.stack.push(a);
                }
            }
            "NIP"    => { let n = self.stack.len(); if n >= 2 { self.stack.remove(n-2); } }
            "TUCK"   => {
                let n = self.stack.len();
                if n >= 2 {
                    let top = self.stack[n-1];
                    self.stack.insert(n-2, top);
                }
            }
            "2DUP"   => {
                let n = self.stack.len();
                if n >= 2 { let a = self.stack[n-2]; let b = self.stack[n-1]; self.stack.push(a); self.stack.push(b); }
            }
            "2DROP"  => { self.stack.pop(); self.stack.pop(); }
            "2SWAP"  => {
                let n = self.stack.len();
                if n >= 4 {
                    self.stack.swap(n-1, n-3);
                    self.stack.swap(n-2, n-4);
                }
            }

            // Arithmetic
            "+"   => { let b = self.pop(); let a = self.pop(); self.stack.push(a.wrapping_add(b)); }
            "-"   => { let b = self.pop(); let a = self.pop(); self.stack.push(a.wrapping_sub(b)); }
            "*"   => { let b = self.pop(); let a = self.pop(); self.stack.push(a.wrapping_mul(b)); }
            "/"   => {
                let b = self.pop();
                let a = self.pop();
                self.stack.push(if b != 0 { a / b } else { 0 });
            }
            "MOD" => {
                let b = self.pop();
                let a = self.pop();
                self.stack.push(if b != 0 { a % b } else { 0 });
            }
            "/MOD" => {
                let b = self.pop(); let a = self.pop();
                if b != 0 { self.stack.push(a % b); self.stack.push(a / b); }
                else { self.stack.push(0); self.stack.push(0); }
            }
            "NEGATE" => { let a = self.pop(); self.stack.push(-a); }
            "ABS"    => { let a = self.pop(); self.stack.push(a.abs()); }
            "MAX"    => { let b = self.pop(); let a = self.pop(); self.stack.push(a.max(b)); }
            "MIN"    => { let b = self.pop(); let a = self.pop(); self.stack.push(a.min(b)); }
            "1+"     => { let a = self.pop(); self.stack.push(a + 1); }
            "1-"     => { let a = self.pop(); self.stack.push(a - 1); }
            "2*"     => { let a = self.pop(); self.stack.push(a * 2); }
            "2/"     => { let a = self.pop(); self.stack.push(a / 2); }

            // Logic/comparison
            "="   => { let b = self.pop(); let a = self.pop(); self.stack.push(if a == b { -1 } else { 0 }); }
            "<>"  => { let b = self.pop(); let a = self.pop(); self.stack.push(if a != b { -1 } else { 0 }); }
            "<"   => { let b = self.pop(); let a = self.pop(); self.stack.push(if a < b  { -1 } else { 0 }); }
            ">"   => { let b = self.pop(); let a = self.pop(); self.stack.push(if a > b  { -1 } else { 0 }); }
            "<="  => { let b = self.pop(); let a = self.pop(); self.stack.push(if a <= b { -1 } else { 0 }); }
            ">="  => { let b = self.pop(); let a = self.pop(); self.stack.push(if a >= b { -1 } else { 0 }); }
            "0="  => { let a = self.pop(); self.stack.push(if a == 0 { -1 } else { 0 }); }
            "0<"  => { let a = self.pop(); self.stack.push(if a < 0  { -1 } else { 0 }); }
            "0>"  => { let a = self.pop(); self.stack.push(if a > 0  { -1 } else { 0 }); }
            "AND" => { let b = self.pop(); let a = self.pop(); self.stack.push(a & b); }
            "OR"  => { let b = self.pop(); let a = self.pop(); self.stack.push(a | b); }
            "XOR" => { let b = self.pop(); let a = self.pop(); self.stack.push(a ^ b); }
            "INVERT" => { let a = self.pop(); self.stack.push(!a); }

            // I/O
            "."   => {
                let a = self.pop();
                self.output.push_str(&format!("{a} "));
            }
            ".S"  => {
                for v in &self.stack { self.output.push_str(&format!("{v} ")); }
                self.output.push('\n');
            }
            "CR"  => { self.output.push('\n'); }
            "EMIT" => {
                let a = self.pop();
                if let Some(ch) = char::from_u32(a as u32) { self.output.push(ch); }
            }
            "SPACE" => { self.output.push(' '); }
            "SPACES" => {
                let n = self.pop().max(0) as usize;
                for _ in 0..n { self.output.push(' '); }
            }

            // Memory
            "@"  => {
                let addr = self.pop() as usize;
                let v = self.memory.get(addr).copied().unwrap_or(0);
                self.stack.push(v);
            }
            "!"  => {
                let addr = self.pop() as usize;
                let val  = self.pop();
                if addr > 65536 { /* safety limit */ }
                else if addr < self.memory.len() { self.memory[addr] = val; }
                else { self.memory.resize(addr + 1, 0); self.memory[addr] = val; }
            }
            "C@" => {
                let addr = self.pop() as usize;
                let v = self.memory.get(addr).copied().unwrap_or(0);
                self.stack.push(v & 0xFF);
            }
            "C!" => {
                let addr = self.pop() as usize;
                let val  = self.pop();
                if addr < self.memory.len() { self.memory[addr] = val & 0xFF; }
            }
            "+!"  => {
                let addr = self.pop() as usize;
                let n    = self.pop();
                if addr < self.memory.len() { self.memory[addr] += n; }
            }
            "VARIABLE" => {
                // Allocate next memory slot
                let addr = self.memory.len() as i64;
                self.memory.push(0);
                if *i + 1 < tokens.len() {
                    let name = tokens[*i + 1].clone();
                    self.dictionary.insert(name, vec!["ADDR".to_string(), addr.to_string()]);
                    *i += 1;
                }
            }
            "CONSTANT" => {
                let val = self.pop();
                if *i + 1 < tokens.len() {
                    let name = tokens[*i + 1].clone();
                    self.dictionary.insert(name, vec!["CONST".to_string(), val.to_string()]);
                    *i += 1;
                }
            }

            // IF / ELSE / THEN (immediate-mode, simplified)
            "IF" => {
                let cond = self.pop();
                if cond == 0 {
                    // Skip until ELSE or THEN
                    let mut depth = 1;
                    let start = *i + 1;
                    let mut j = start;
                    while j < tokens.len() {
                        match tokens[j].as_str() {
                            "IF" => depth += 1,
                            "THEN" => { depth -= 1; if depth == 0 { *i = j; break; } }
                            "ELSE" if depth == 1 => { *i = j; break; }
                            _ => {}
                        }
                        j += 1;
                    }
                }
            }
            "ELSE" => {
                // Skip until THEN
                let mut depth = 1;
                let mut j = *i + 1;
                while j < tokens.len() {
                    match tokens[j].as_str() {
                        "IF" => depth += 1,
                        "THEN" => { depth -= 1; if depth == 0 { *i = j; break; } }
                        _ => {}
                    }
                    j += 1;
                }
            }
            "THEN" => {} // no-op; jump destination

            // BEGIN / AGAIN / UNTIL / WHILE / REPEAT
            "BEGIN" => { self.return_stack.push(*i); self.iteration_count = 0; }
            "AGAIN" => {
                self.iteration_count += 1;
                if self.iteration_count > 100_000 {
                    self.output.push_str("⚠️ Forth: loop limit reached (BEGIN/AGAIN)\n");
                    self.return_stack.pop();
                } else if let Some(begin) = self.return_stack.last().copied() {
                    *i = begin; // will be incremented by outer loop
                }
            }
            "UNTIL" => {
                let cond = self.pop();
                if cond == 0 {
                    self.iteration_count += 1;
                    if self.iteration_count > 100_000 {
                        self.output.push_str("⚠️ Forth: loop limit reached (BEGIN/UNTIL)\n");
                        self.return_stack.pop();
                    } else if let Some(begin) = self.return_stack.last().copied() {
                        *i = begin;
                    }
                } else {
                    self.return_stack.pop();
                }
            }
            "WHILE" => {
                let cond = self.pop();
                if cond == 0 {
                    // Skip to REPEAT
                    let mut depth = 1;
                    let mut j = *i + 1;
                    while j < tokens.len() {
                        match tokens[j].as_str() {
                            "BEGIN" => depth += 1,
                            "REPEAT" => { depth -= 1; if depth == 0 { *i = j; break; } }
                            _ => {}
                        }
                        j += 1;
                    }
                    self.return_stack.pop();
                }
            }
            "REPEAT" => {
                self.iteration_count += 1;
                if self.iteration_count > 100_000 {
                    self.output.push_str("⚠️ Forth: loop limit reached (BEGIN/REPEAT)\n");
                    self.return_stack.pop();
                } else if let Some(begin) = self.return_stack.last().copied() {
                    *i = begin;
                }
            }

            // DO / LOOP — loop state on dedicated loop_stack
            "DO" => {
                let start = self.pop();
                let limit = self.pop();
                self.loop_stack.push((start, limit, *i));
            }
            "LOOP" => {
                if let Some((index, limit, do_pos)) = self.loop_stack.last_mut() {
                    *index += 1;
                    if *index < *limit {
                        *i = *do_pos;
                    } else {
                        self.loop_stack.pop();
                    }
                }
            }
            "+LOOP" => {
                let step = self.pop();
                if let Some((index, limit, do_pos)) = self.loop_stack.last_mut() {
                    *index += step;
                    let done = if step > 0 { *index >= *limit } else { *index <= *limit };
                    if !done {
                        *i = *do_pos;
                    } else {
                        self.loop_stack.pop();
                    }
                }
            }
            "I" => {
                if let Some(&(index, _, _)) = self.loop_stack.last() {
                    self.stack.push(index);
                }
            }
            "J" => {
                if self.loop_stack.len() >= 2 {
                    let j_idx = self.loop_stack.len() - 2;
                    self.stack.push(self.loop_stack[j_idx].0);
                }
            }
            "LEAVE" => {
                if let Some((_, _, do_pos)) = self.loop_stack.pop() {
                    // Skip forward to matching LOOP/+LOOP
                    let mut depth = 1i32;
                    let mut j = do_pos + 1;
                    while j < tokens.len() {
                        let w = tokens[j].to_uppercase();
                        match w.as_str() {
                            "DO" => depth += 1,
                            "LOOP" | "+LOOP" => { depth -= 1; if depth == 0 { *i = j; break; } }
                            _ => {}
                        }
                        j += 1;
                    }
                }
            }

            // Turtle graphics extensions
            "FD"   => { let n = self.pop() as f64; ctx.turtle.forward(n); }
            "BK"   => { let n = self.pop() as f64; ctx.turtle.backward(n); }
            "RT"   => { let n = self.pop() as f64; ctx.turtle.right(n as f64); }
            "LT"   => { let n = self.pop() as f64; ctx.turtle.left(n as f64); }
            "PU"   => { ctx.turtle.pen_up(); }
            "PD"   => { ctx.turtle.pen_down_cmd(); }
            "HOME" => { ctx.turtle.home(); }
            "CLEAN" | "CLEARSCREEN" | "CS" => { ctx.turtle.clear_screen(); }
            "PEN"  => {
                let c = self.pop() as u8;
                if let Some(color) = tw_graphics::turtle::default_palette_16(c) {
                    ctx.turtle.set_pen_color(color);
                }
            }
            "COLOR" => {
                // COLOR word — accept a named colour or palette index from the stack
                // (Forth-style: push the colour index/name, then call COLOR)
                let c = self.pop() as u8;
                if let Some(color) = tw_graphics::turtle::default_palette_16(c) {
                    ctx.turtle.set_pen_color(color);
                }
            }
            "SETX" => { let x = self.pop() as f64; ctx.turtle.set_x(x); }
            "SETY" => { let y = self.pop() as f64; ctx.turtle.set_y(y); }

            // ── Numeric literals ─────────────────────────────────────────
            _ => {
                // Try number
                if let Ok(v) = word.parse::<i64>() {
                    self.stack.push(v);
                    return;
                }
                // Try hex literal
                if word.starts_with("0X") || word.starts_with("$") {
                    let hex = word.trim_start_matches("0X").trim_start_matches('$');
                    if let Ok(v) = i64::from_str_radix(hex, 16) {
                        self.stack.push(v);
                        return;
                    }
                }
                // User-defined word
                if let Some(def) = self.dictionary.get(word).cloned() {
                    if def.len() == 2 && def[0] == "ADDR" {
                        // Variable — push address
                        if let Ok(addr) = def[1].parse::<i64>() {
                            self.stack.push(addr);
                        }
                        return;
                    }
                    if def.len() == 2 && def[0] == "CONST" {
                        if let Ok(v) = def[1].parse::<i64>() {
                            self.stack.push(v);
                        }
                        return;
                    }
                    let line = def.join(" ");
                    self.call_depth += 1;
                    if self.call_depth > 256 {
                        self.output.push_str("⚠️ Forth: call depth limit exceeded\n");
                        self.call_depth -= 1;
                        return;
                    }
                    let result = self.execute(ctx, &line);
                    self.call_depth -= 1;
                    if !result.is_empty() {
                        self.output.push_str(&result);
                    }
                } else {
                    self.output.push_str(&format!("⚠️ Forth: unknown word: {word}\n"));
                }
            }
        }
    }

    fn pop(&mut self) -> i64 {
        self.stack.pop().unwrap_or(0)
    }

    /// Tokenize a Forth source line, handling `."` strings (preserving case)
    /// and `( ... )` comments.
    fn tokenize_forth(&self, line: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let n = chars.len();
        let mut i = 0;

        while i < n {
            // Skip whitespace
            if chars[i].is_whitespace() { i += 1; continue; }

            // `( ... )` comment — skip until closing `)`
            if chars[i] == '(' && (i + 1 >= n || chars[i + 1].is_whitespace()) {
                i += 1;
                while i < n && chars[i] != ')' { i += 1; }
                if i < n { i += 1; } // skip ')'
                continue;
            }

            // `\` line comment — skip rest of line
            if chars[i] == '\\' && (i + 1 >= n || chars[i + 1].is_whitespace()) {
                break;
            }

            // `."` dot-quote string — collect until closing `"` preserving case
            if chars[i] == '.' && i + 1 < n && chars[i + 1] == '"' {
                i += 2; // skip `." `
                // Skip one optional space after ."
                if i < n && chars[i] == ' ' { i += 1; }
                let mut s = String::from(".\"");
                while i < n && chars[i] != '"' {
                    s.push(chars[i]);
                    i += 1;
                }
                s.push('"');
                if i < n { i += 1; } // skip closing "
                tokens.push(s);
                continue;
            }

            // Normal word — collect until whitespace, uppercase
            let start = i;
            while i < n && !chars[i].is_whitespace() { i += 1; }
            let word: String = chars[start..i].iter().collect();
            tokens.push(word.to_uppercase());
        }
        tokens
    }
}

// ── public wrapper ─────────────────────────────────────────────────────────────

pub fn execute_forth_safe(ctx: &mut ExecContext, line: &str) -> String {
    let mut forth = ctx.forth.take().unwrap_or_default();
    let out = forth.execute(ctx, line);
    ctx.forth = Some(forth);
    out
}

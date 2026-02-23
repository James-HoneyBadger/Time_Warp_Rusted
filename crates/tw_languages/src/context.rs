//! `ExecContext` — shared interpreter state used by all language executors.
//!
//! This deliberately mirrors the Python `Interpreter` object so that the
//! language-executor functions (`execute_basic`, `execute_logo`, …) have the
//! same conceptual "view" of the machine as their Python counterparts had.

use std::collections::HashMap;
use tw_graphics::TurtleState;

// ── control flow ─────────────────────────────────────────────────────────────

/// Signal returned from a single-statement handler.
#[derive(Debug, Clone, PartialEq)]
pub enum ControlFlow {
    Continue,
    /// Jump to an absolute line index (0-based) in `program_lines`.
    Jump(usize),
    /// Jump to a named label (resolved by the executor).
    JumpLabel(String),
    /// Push current position and jump.
    Gosub(usize),
    /// Return from subroutine.
    Return,
    /// Halt execution normally.
    End,
    /// Wait for user input; resume when available.
    WaitInput,
    /// Runtime error message.
    Error(String),
}

// ── loop / call frames ───────────────────────────────────────────────────────

/// A `FOR … TO … STEP` loop frame (BASIC / Pascal).
#[derive(Debug, Clone)]
pub struct ForFrame {
    pub var_name: String,
    pub end_val:  f64,
    pub step:     f64,
    /// Index in `program_lines` of the FOR statement itself.
    pub for_idx:  usize,
}

/// A GOSUB / procedure-call return address.
#[derive(Debug, Clone)]
pub struct GosubFrame {
    /// Return address: index in `program_lines` to jump back to.
    pub return_idx: usize,
}

/// A user-defined sub-routine (SUB / FUNCTION / PROCEDURE / TO … END).
#[derive(Debug, Clone)]
pub struct SubDef {
    pub name:       String,
    pub params:     Vec<String>,
    /// Owned body lines (already stripped of enclosing TO/SUB/PROCEDURE header).
    pub body_lines: Vec<String>,
}

// ── Pascal-specific block stack entry ────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PascalBlock {
    If { else_idx: Option<usize>, end_idx: usize },
    While { while_idx: usize, end_idx: usize },
    Repeat { repeat_idx: usize },
    For { var: String, end_val: i64, step: i64, end_idx: usize },
    Case { end_idx: usize, matched: bool },
}

// ── Prolog fact / rule ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PrologFact {
    pub functor: String,
    pub args:    Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PrologRule {
    pub functor: String,
    pub params:  Vec<String>,
    pub body:    Vec<(String, Vec<String>)>, // [(functor, args)]
}

// ── shared context ────────────────────────────────────────────────────────────

/// All mutable state shared between the main execution loop and every
/// language executor.  Roughly equivalent to the Python `Interpreter` class.
pub struct ExecContext {
    // ── variables ────────────────────────────────────────────────────────
    /// Numeric variables (float).  BASIC, Logo, PILOT, Forth all use this.
    pub variables:        HashMap<String, f64>,
    /// String variables (`name$` in BASIC).
    pub string_vars:      HashMap<String, String>,
    /// Numeric arrays.  Key is the bare name; index is 0-based.
    pub arrays:           HashMap<String, Vec<f64>>,

    // ── program ──────────────────────────────────────────────────────────
    /// `(line_number, source_text)` — 0-based index is the cursor.
    pub program_lines:    Vec<(u32, String)>,
    /// Current execution cursor (index into `program_lines`).
    pub line_idx:         usize,

    // ── output ───────────────────────────────────────────────────────────
    pub output:           Vec<String>,
    /// Escape-hatch text accumulated within a single statement.
    pub line_output:      String,

    // ── control flow ─────────────────────────────────────────────────────
    pub for_stack:        Vec<ForFrame>,
    pub gosub_stack:      Vec<GosubFrame>,
    pub while_stack:      Vec<(usize, String)>,  // (while_idx, condition_expr)
    pub do_stack:         Vec<usize>,             // repeat-until / do-loop

    // ── subroutines ──────────────────────────────────────────────────────
    pub subs:             HashMap<String, SubDef>,

    // ── PILOT ────────────────────────────────────────────────────────────
    pub last_match:       bool,
    pub last_input:       String,

    // ── Labels (BASIC, PILOT, Logo) ───────────────────────────────────────
    /// label → line index mapping, built before execution.
    pub labels:           HashMap<String, usize>,

    // ── input queue ──────────────────────────────────────────────────────
    /// Pending user-input requests: (prompt, var_name, is_numeric).
    pub input_requests:   Vec<(String, String, bool)>,
    /// Buffered user responses.
    pub input_responses:  Vec<String>,
    /// Currently waiting for input?
    pub waiting_input:    bool,

    // ── Pascal ───────────────────────────────────────────────────────────
    pub pascal_block_stack: Vec<PascalBlock>,
    pub pascal_in_var_section: bool,
    pub pascal_in_const_section: bool,

    // ── Prolog ───────────────────────────────────────────────────────────
    pub prolog_facts:     Vec<PrologFact>,
    pub prolog_rules:     Vec<PrologRule>,
    /// Accumulates Prolog clauses that span multiple lines.
    pub prolog_buffer:    String,

    // ── Forth ────────────────────────────────────────────────────────────
    pub forth:            Option<crate::forth::ForthExecutor>,

    // ── graphics ─────────────────────────────────────────────────────────
    pub turtle:           TurtleState,

    // ── text mode ────────────────────────────────────────────────────────
    pub text_lines:       Vec<String>,

    // ── safety limits ────────────────────────────────────────────────────
    pub iteration_count:  u64,
    pub max_iterations:   u64,
    pub call_depth:       u32,
    pub max_call_depth:   u32,

    // ── SELECT CASE ──────────────────────────────────────────────────────
    pub select_val:       Option<f64>,
    pub in_select:        bool,
    pub select_matched:   bool,

    // ── Multi-line IF/THEN/ELSE/END IF ───────────────────────────────────
    /// Stack for block-IF tracking.  Each entry = "currently executing body".
    /// When any entry is `false`, lines are skipped until the matching
    /// `ELSE` or `END IF` flips / pops it.
    pub if_block_stack:   Vec<bool>,
}

impl Default for ExecContext {
    fn default() -> Self {
        Self {
            variables:            HashMap::new(),
            string_vars:          HashMap::new(),
            arrays:               HashMap::new(),
            program_lines:        Vec::new(),
            line_idx:             0,
            output:               Vec::new(),
            line_output:          String::new(),
            for_stack:            Vec::new(),
            gosub_stack:          Vec::new(),
            while_stack:          Vec::new(),
            do_stack:             Vec::new(),
            subs:                 HashMap::new(),
            last_match:           false,
            last_input:           String::new(),
            labels:               HashMap::new(),
            input_requests:       Vec::new(),
            input_responses:      Vec::new(),
            waiting_input:        false,
            pascal_block_stack:   Vec::new(),
            pascal_in_var_section:  false,
            pascal_in_const_section: false,
            prolog_facts:         Vec::new(),
            prolog_rules:         Vec::new(),
            prolog_buffer:        String::new(),
            forth:                None,
            turtle:               TurtleState::new(),
            text_lines:           Vec::new(),
            iteration_count:      0,
            max_iterations:       100_000,
            call_depth:           0,
            max_call_depth:       256,
            select_val:           None,
            in_select:            false,
            select_matched:       false,
            if_block_stack:       Vec::new(),
        }
    }
}

impl ExecContext {
    pub fn new() -> Self {
        Self::default()
    }

    // ── variable helpers ─────────────────────────────────────────────────

    pub fn get_var(&self, name: &str) -> f64 {
        let upper = name.to_uppercase();
        self.variables.get(&upper).copied().unwrap_or(0.0)
    }

    pub fn set_var(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_uppercase(), value);
    }

    pub fn get_str(&self, name: &str) -> String {
        let upper = name.to_uppercase();
        self.string_vars.get(&upper).cloned().unwrap_or_default()
    }

    pub fn set_str(&mut self, name: &str, value: String) {
        self.string_vars.insert(name.to_uppercase(), value);
    }

    pub fn get_array(&self, name: &str, idx: usize) -> f64 {
        let upper = name.to_uppercase();
        self.arrays
            .get(&upper)
            .and_then(|v| v.get(idx))
            .copied()
            .unwrap_or(0.0)
    }

    pub fn set_array(&mut self, name: &str, idx: usize, val: f64) {
        let upper = name.to_uppercase();
        let arr = self.arrays.entry(upper).or_insert_with(Vec::new);
        if arr.len() <= idx {
            arr.resize(idx + 1, 0.0);
        }
        arr[idx] = val;
    }

    pub fn dim_array(&mut self, name: &str, size: usize) {
        let upper = name.to_uppercase();
        self.arrays.insert(upper, vec![0.0; size + 1]);
    }

    // ── output helpers ────────────────────────────────────────────────────

    pub fn emit(&mut self, text: &str) {
        self.output.push(text.to_string());
    }

    pub fn emit_ln(&mut self, text: &str) {
        let mut s = text.to_string();
        if !s.ends_with('\n') {
            s.push('\n');
        }
        self.output.push(s);
    }

    // ── program / label helpers ───────────────────────────────────────────

    /// Build the `labels` map from `program_lines`.
    /// Recognises:
    ///  - BASIC line numbers (`100 PRINT …` → label `"100"`)
    ///  - Logo/PILOT labels (`*label` or just `label:`)
    pub fn build_labels(&mut self) {
        self.labels.clear();
        for (idx, (ln, text)) in self.program_lines.iter().enumerate() {
            // BASIC-style: store the line number as its own label
            self.labels.insert(ln.to_string(), idx);

            let trimmed = text.trim();

            // PILOT star-labels: *LABEL
            if let Some(rest) = trimmed.strip_prefix('*') {
                let label = rest.trim().to_uppercase();
                if !label.is_empty() {
                    self.labels.insert(label, idx);
                }
            }

            // BASIC / Pascal goto labels: `label:` at start of line
            if trimmed.ends_with(':') && !trimmed.contains(' ') {
                let label = trimmed.trim_end_matches(':').to_uppercase();
                if !label.is_empty() {
                    self.labels.insert(label, idx);
                }
            }

            // Logo TO … END: store procedure name
            let up = trimmed.to_uppercase();
            if up.starts_with("TO ") {
                let proc = up[3..].split_whitespace().next().unwrap_or("").to_string();
                if !proc.is_empty() {
                    self.labels.insert(proc, idx);
                }
            }

            // PILOT verbose RULE name: — register as subroutine label
            if up.starts_with("RULE ") {
                let rest = up[5..].trim().trim_end_matches(':');
                let name = rest.split_whitespace().next().unwrap_or("").to_string();
                if !name.is_empty() {
                    self.labels.insert(name, idx);
                }
            }
        }
    }

    /// Resolve a label to a line index.
    pub fn resolve_label(&self, label: &str) -> Option<usize> {
        let upper = label.trim().to_uppercase();
        self.labels.get(&upper).copied()
    }

    // ── FOR / NEXT helpers ────────────────────────────────────────────────

    pub fn push_for(&mut self, var: &str, end_val: f64, step: f64, for_idx: usize) {
        self.for_stack.push(ForFrame {
            var_name: var.to_uppercase(),
            end_val,
            step,
            for_idx,
        });
    }

    /// Process a NEXT statement.  Returns the line index to jump back to
    /// (the line after the FOR) or `None` if the loop is complete.
    pub fn process_next(&mut self, var: Option<&str>) -> Option<usize> {
        if self.for_stack.is_empty() {
            return None;
        }
        let frame_idx = if let Some(v) = var {
            let upper = v.to_uppercase();
            self.for_stack.iter().rposition(|f| f.var_name == upper)?
        } else {
            self.for_stack.len() - 1
        };
        let frame = self.for_stack[frame_idx].clone();
        let current = self.get_var(&frame.var_name);
        let next = current + frame.step;
        self.set_var(&frame.var_name, next);

        let done = if frame.step >= 0.0 {
            next > frame.end_val
        } else {
            next < frame.end_val
        };

        if done {
            // Pop this frame and all frames on top of it
            self.for_stack.truncate(frame_idx);
            None
        } else {
            Some(frame.for_idx + 1)
        }
    }

    // ── GOSUB / RETURN helpers ────────────────────────────────────────────

    pub fn push_gosub(&mut self, return_idx: usize) {
        self.gosub_stack.push(GosubFrame { return_idx });
    }

    pub fn pop_gosub(&mut self) -> Option<usize> {
        self.gosub_stack.pop().map(|f| f.return_idx)
    }

    // ── expression evaluator ─────────────────────────────────────────────

    /// Simple safe expression evaluator for numeric expressions.
    /// Handles: +, -, *, /, MOD, ^, functions, variables.
    pub fn eval_expr(&self, expr: &str) -> Result<f64, String> {
        crate::eval::evaluate(expr.trim(), &self.variables, &self.string_vars, &self.arrays)
    }

    /// Evaluate and return a float, defaulting to 0.0 on error.
    pub fn eval_f64(&self, expr: &str) -> f64 {
        self.eval_expr(expr).unwrap_or(0.0)
    }

    /// Expand `*VAR*` (BASIC) and `#VAR` (PILOT) interpolation in text.
    pub fn interpolate(&self, text: &str) -> String {
        let mut result = text.to_string();

        // *VAR* BASIC-style
        let mut out = String::with_capacity(result.len());
        let mut chars = result.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '*' {
                let mut name = String::new();
                loop {
                    match chars.peek() {
                        Some(&'*') => { chars.next(); break; }
                        Some(&c) if c.is_alphanumeric() || c == '_' => {
                            name.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                if !name.is_empty() {
                    let upper = name.to_uppercase();
                    if let Some(&v) = self.variables.get(&upper) {
                        let s = if v == v.floor() && v.abs() < 1e15 {
                            format!("{}", v as i64)
                        } else {
                            format!("{}", v)
                        };
                        out.push_str(&s);
                        continue;
                    } else if let Some(sv) = self.string_vars.get(&upper) {
                        out.push_str(sv);
                        continue;
                    }
                }
                out.push('*');
                out.push_str(&name);
                out.push('*');
            } else {
                out.push(ch);
            }
        }
        result = out;

        // #VAR PILOT-style
        let re_pilot: &str = "#";
        if result.contains(re_pilot) {
            let mut out2 = String::with_capacity(result.len());
            let mut chars2 = result.chars().peekable();
            while let Some(ch) = chars2.next() {
                if ch == '#' {
                    let mut name = String::new();
                    while let Some(&nc) = chars2.peek() {
                        if nc.is_alphanumeric() || nc == '_' {
                            name.push(nc);
                            chars2.next();
                        } else {
                            break;
                        }
                    }
                    if !name.is_empty() {
                        let upper = name.to_uppercase();
                        if let Some(&v) = self.variables.get(&upper) {
                            let s = if v == v.floor() && v.abs() < 1e15 {
                                format!("{}", v as i64)
                            } else {
                                format!("{}", v)
                            };
                            out2.push_str(&s);
                            continue;
                        } else if let Some(sv) = self.string_vars.get(&upper) {
                            out2.push_str(sv);
                            continue;
                        }
                    }
                    out2.push('#');
                    out2.push_str(&name);
                } else {
                    out2.push(ch);
                }
            }
            result = out2;
        }

        result
    }

    // ── input handling ────────────────────────────────────────────────────

    pub fn provide_input(&mut self, value: &str) {
        if let Some((_, var, numeric)) = self.input_requests.first().cloned() {
            if numeric {
                let v = value.trim().parse::<f64>().unwrap_or(0.0);
                self.set_var(&var, v);
            } else {
                self.set_str(&var, value.trim().to_string());
                // Also set numeric if parseable
                if let Ok(v) = value.trim().parse::<f64>() {
                    self.set_var(&var, v);
                }
            }
            self.last_input = value.trim().to_string();
            self.input_requests.remove(0);
        }
        self.waiting_input = !self.input_requests.is_empty();
    }

    pub fn request_input(&mut self, prompt: &str, var: &str, numeric: bool) {
        self.input_requests.push((prompt.to_string(), var.to_uppercase(), numeric));
        self.waiting_input = true;
    }

    // ── reset ─────────────────────────────────────────────────────────────

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Reset execution state but keep loaded subroutines / labels.
    pub fn reset_execution(&mut self) {
        self.variables.clear();
        self.string_vars.clear();
        self.arrays.clear();
        self.output.clear();
        self.line_output.clear();
        self.for_stack.clear();
        self.gosub_stack.clear();
        self.while_stack.clear();
        self.do_stack.clear();
        self.line_idx = 0;
        self.iteration_count = 0;
        self.call_depth = 0;
        self.last_match = false;
        self.last_input.clear();
        self.input_requests.clear();
        self.input_responses.clear();
        self.waiting_input = false;
        self.pascal_block_stack.clear();
        self.pascal_in_var_section = false;
        self.prolog_facts.clear();
        self.prolog_rules.clear();
        self.prolog_buffer.clear();
        self.text_lines.clear();
        self.turtle.clear_screen();
        self.select_val = None;
        self.in_select = false;
        self.select_matched = false;
        self.if_block_stack.clear();
    }
}

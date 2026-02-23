//! Main `Interpreter` — wraps `ExecContext` and drives the execution loop.
//! Port of `core/interpreter.py`.

use crate::debugger::ExecutionTimeline;
use crate::language::Language;
use tw_iot::GpioManager;
use tw_iot::board::Board;
use tw_languages::{
    context::{ControlFlow, ExecContext},
    execute_basic, execute_c, execute_logo, execute_pascal, execute_pilot, execute_prolog,
    forth::execute_forth_safe,
};

/// Execution state seen by the UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunState {
    Idle,
    Running,
    WaitingInput,
    Finished,
    Error(String),
}

/// Top-level interpreter that loads, runs, and debugs programs.
pub struct Interpreter {
    pub ctx:       ExecContext,
    pub language:  Language,
    pub timeline:  ExecutionTimeline,
    pub state:     RunState,
    /// Number of statements executed per `step_batch()` call.
    pub batch_size: usize,
    /// Whether to record frames into the timeline.
    pub record:    bool,
    /// GPIO / IoT manager for Raspberry Pi integration.
    pub gpio:      GpioManager,
}

impl Interpreter {
    pub fn new(language: Language) -> Self {
        let mut gpio = GpioManager::new(Board::Simulator);
        let _ = gpio.connect();
        Self {
            ctx:        ExecContext::new(),
            language,
            timeline:   ExecutionTimeline::new(),
            state:      RunState::Idle,
            batch_size: 200,
            record:     false,
            gpio,
        }
    }

    /// Load source text and prepare for execution.
    pub fn load(&mut self, source: &str) {
        self.ctx.reset_execution();
        self.timeline.clear();

        let lines: Vec<(u32, String)> = Self::parse_lines(source, self.language);
        self.ctx.program_lines = lines;
        self.ctx.build_labels();
        self.state = RunState::Idle;
    }

    fn parse_lines(source: &str, language: Language) -> Vec<(u32, String)> {
        // Logo needs special preprocessing: bracket blocks `[...]` and
        // `TO…END` definitions can span multiple source lines, but the
        // executor sees one program-line at a time.  We join all non-comment
        // lines into logical statements that respect nesting.
        if language == Language::Logo {
            return Self::parse_logo_lines(source);
        }

        let mut result = Vec::new();
        for (i, raw) in source.lines().enumerate() {
            let line = raw.trim_end();
            // BASIC line numbers
            if language == Language::Basic {
                let mut parts = line.splitn(2, ' ');
                if let Some(num_str) = parts.next() {
                    if let Ok(n) = num_str.parse::<u32>() {
                        let rest = parts.next().unwrap_or("").to_string();
                        result.push((n, rest));
                        continue;
                    }
                }
            }
            result.push(((i + 1) as u32, line.to_string()));
        }
        result
    }

    /// Preprocess Logo source into logical lines.
    ///
    /// Logo programs use multi-line bracket blocks (`REPEAT n [ ... ]`) and
    /// `TO name … END` procedure definitions.  We collapse these into single
    /// logical lines so that the tokeniser and executor see complete
    /// statements.
    fn parse_logo_lines(source: &str) -> Vec<(u32, String)> {
        // Step 1: strip comment lines & blank lines, join into one stream
        let mut joined = String::new();
        let mut first_line: Option<u32> = None;
        for (i, raw) in source.lines().enumerate() {
            let trimmed = raw.trim();
            // Skip comment lines
            if trimmed.starts_with(';') || trimmed.is_empty() {
                continue;
            }
            if first_line.is_none() {
                first_line = Some((i + 1) as u32);
            }
            if !joined.is_empty() {
                joined.push(' ');
            }
            joined.push_str(trimmed);
        }

        if joined.trim().is_empty() {
            return Vec::new();
        }

        // Step 2: split into logical statements.
        // A "logical statement" is a top-level command sequence.  We split on
        // `TO` keyword boundaries so procedure definitions are separate from
        // the main body, which aids debugging.
        let line_no = first_line.unwrap_or(1);
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_to_block = false;
        let mut bracket_depth = 0i32;

        for token in Self::logo_rough_tokenize(&joined) {
            let up = token.to_uppercase();

            if up == "TO" && bracket_depth == 0 && !in_to_block {
                // Flush any accumulated tokens as a logical line
                let flushed = current.trim().to_string();
                if !flushed.is_empty() {
                    result.push((line_no, flushed));
                }
                current = String::from("TO");
                in_to_block = true;
                continue;
            }

            if up == "END" && bracket_depth == 0 && in_to_block {
                current.push_str(" END");
                let flushed = current.trim().to_string();
                if !flushed.is_empty() {
                    result.push((line_no, flushed));
                }
                current = String::new();
                in_to_block = false;
                continue;
            }

            // Track bracket nesting
            for ch in token.chars() {
                if ch == '[' { bracket_depth += 1; }
                if ch == ']' { bracket_depth -= 1; }
            }

            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(&token);
        }

        let flushed = current.trim().to_string();
        if !flushed.is_empty() {
            result.push((line_no, flushed));
        }

        // If we produced nothing, put an empty placeholder so exec doesn't skip
        if result.is_empty() {
            result.push((1, String::new()));
        }
        result
    }

    /// Rough tokeniser that splits on whitespace while keeping bracket blocks
    /// `[…]` together as single tokens.
    fn logo_rough_tokenize(source: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut depth = 0i32;

        for ch in source.chars() {
            match ch {
                '[' => {
                    depth += 1;
                    current.push(ch);
                }
                ']' => {
                    current.push(ch);
                    depth -= 1;
                    if depth == 0 {
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
                _ => {
                    current.push(ch);
                }
            }
        }
        let t = current.trim().to_string();
        if !t.is_empty() { tokens.push(t); }
        tokens
    }

    /// Run up to `batch_size` statements in one call, driven by the UI frame timer.
    /// Returns `true` if execution is still ongoing.
    pub fn step_batch(&mut self) -> bool {
        if matches!(self.state, RunState::Idle | RunState::Finished | RunState::Error(_)) {
            return false;
        }
        if matches!(self.state, RunState::WaitingInput) {
            // Don't progress until input is provided.
            return true;
        }

        let mut steps = 0;
        while steps < self.batch_size && self.ctx.line_idx < self.ctx.program_lines.len() {
            let (line_no, line_text) = self.ctx.program_lines[self.ctx.line_idx].clone();
            let line_idx = self.ctx.line_idx;

            if self.record {
                let output = self.ctx.output.join("");
                self.timeline.record_frame(
                    line_no, &line_text,
                    &self.ctx.variables,
                    &self.ctx.string_vars,
                    &self.ctx.arrays,
                    &output,
                );
            }

            // Breakpoint check
            if self.timeline.has_breakpoint(line_no) && steps > 0 {
                self.state = RunState::WaitingInput; // repurpose as "paused"
                return true;
            }

            let cf = self.dispatch(&line_text);
            self.process_gpio_commands();
            steps += 1;

            match cf {
                ControlFlow::Continue => { self.ctx.line_idx += 1; }
                ControlFlow::Jump(idx) => { self.ctx.line_idx = idx; }
                ControlFlow::JumpLabel(label) => {
                    if let Some(idx) = self.ctx.resolve_label(&label) {
                        self.ctx.line_idx = idx;
                    } else {
                        self.state = RunState::Error(format!("Label not found: {label}"));
                        return false;
                    }
                }
                ControlFlow::Gosub(idx) => {
                    self.ctx.push_gosub(line_idx + 1);
                    self.ctx.line_idx = idx;
                }
                ControlFlow::Return => {
                    if let Some(ret) = self.ctx.pop_gosub() {
                        self.ctx.line_idx = ret;
                    } else {
                        self.state = RunState::Finished;
                        return false;
                    }
                }
                ControlFlow::End => {
                    self.state = RunState::Finished;
                    return false;
                }
                ControlFlow::WaitInput => {
                    self.state = RunState::WaitingInput;
                    return true;
                }
                ControlFlow::Error(msg) => {
                    self.state = RunState::Error(msg);
                    return false;
                }
            }
        }

        if self.ctx.line_idx >= self.ctx.program_lines.len() {
            self.state = RunState::Finished;
            false
        } else {
            true
        }
    }

    /// Dispatch a single source line to the appropriate language executor.
    fn dispatch(&mut self, line: &str) -> ControlFlow {
        match self.language {
            Language::Basic  => execute_basic(&mut self.ctx, line),
            Language::Pilot  => execute_pilot(&mut self.ctx, line),
            Language::Logo   => execute_logo(&mut self.ctx, line),
            Language::C      => execute_c(&mut self.ctx, line),
            Language::Pascal => execute_pascal(&mut self.ctx, line),
            Language::Prolog => execute_prolog(&mut self.ctx, line),
            Language::Forth  => {
                let out = execute_forth_safe(&mut self.ctx, line);
                if !out.is_empty() { self.ctx.emit(&out); }
                ControlFlow::Continue
            }
        }
    }

    /// Scan output for GPIO: command prefixes and route them to the GpioManager.
    fn process_gpio_commands(&mut self) {
        use tw_iot::gpio::PinMode;

        // Collect GPIO commands from output, remove them from visible output
        let mut gpio_lines = Vec::new();
        let mut clean_output = Vec::new();
        for line in &self.ctx.output {
            let mut remaining = String::new();
            for part in line.split('\n') {
                if part.starts_with("GPIO:") {
                    gpio_lines.push(part.to_string());
                } else if !part.is_empty() {
                    if !remaining.is_empty() { remaining.push('\n'); }
                    remaining.push_str(part);
                }
            }
            if !remaining.is_empty() {
                remaining.push('\n');
                clean_output.push(remaining);
            }
        }

        if gpio_lines.is_empty() { return; }

        // Replace output with cleaned version (GPIO commands not visible to user)
        self.ctx.output = clean_output;

        for cmd in &gpio_lines {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.is_empty() { continue; }

            match parts[0] {
                "GPIO:PINMODE" if parts.len() >= 3 => {
                    if let Ok(pin) = parts[1].parse::<u8>() {
                        let mode = match parts[2] {
                            "INPUT"  | "IN"  => PinMode::Input,
                            "OUTPUT" | "OUT" => PinMode::Output,
                            "PWM"            => PinMode::Pwm,
                            "I2C"            => PinMode::I2c,
                            "SPI"            => PinMode::Spi,
                            _ => PinMode::Output,
                        };
                        if let Err(e) = self.gpio.pin_mode(pin, mode) {
                            self.ctx.emit(&format!("⚠ GPIO error: {e}\n"));
                        }
                    }
                }
                "GPIO:WRITE" if parts.len() >= 3 => {
                    if let (Ok(pin), Ok(val)) = (parts[1].parse::<u8>(), parts[2].parse::<u8>()) {
                        if let Err(e) = self.gpio.digital_write(pin, val != 0) {
                            self.ctx.emit(&format!("⚠ GPIO error: {e}\n"));
                        }
                    }
                }
                "GPIO:READ" if parts.len() >= 2 => {
                    if let Ok(pin) = parts[1].parse::<u8>() {
                        match self.gpio.digital_read(pin) {
                            Ok(v) => {
                                // Store result in variable PINVAL
                                self.ctx.set_var("PINVAL", if v { 1.0 } else { 0.0 });
                            }
                            Err(e) => {
                                self.ctx.emit(&format!("⚠ GPIO error: {e}\n"));
                            }
                        }
                    }
                }
                "GPIO:PWM" if parts.len() >= 3 => {
                    if let (Ok(pin), Ok(duty)) = (parts[1].parse::<u8>(), parts[2].parse::<f64>()) {
                        if let Err(e) = self.gpio.pwm_write(pin, duty) {
                            self.ctx.emit(&format!("⚠ GPIO error: {e}\n"));
                        }
                    }
                }
                "GPIO:RESET" => {
                    self.gpio.reset();
                }
                _ => {}
            }
        }
    }

    /// Start running (sets state to Running).
    pub fn run(&mut self) {
        if self.ctx.program_lines.is_empty() { return; }
        if matches!(self.state, RunState::Idle | RunState::Finished | RunState::Error(_)) {
            self.ctx.line_idx = 0;
        }
        self.state = RunState::Running;
    }

    /// Stop and reset.
    pub fn stop(&mut self) {
        self.state = RunState::Finished;
    }

    /// Provide user input and resume execution.
    pub fn provide_input(&mut self, text: &str) {
        let had_request = !self.ctx.input_requests.is_empty();
        self.ctx.provide_input(text);
        if matches!(self.state, RunState::WaitingInput) {
            // Advance past the INPUT/ACCEPT/scanf/readln statement so it
            // isn't re-dispatched (which would create a new request loop).
            if had_request && self.ctx.line_idx < self.ctx.program_lines.len() {
                self.ctx.line_idx += 1;
            }
            self.state = RunState::Running;
        }
    }

    /// Accumulated output text.
    pub fn output(&self) -> String {
        self.ctx.output.join("")
    }

    /// Clear output.
    pub fn clear_output(&mut self) {
        self.ctx.output.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_to_completion(lang: Language, source: &str) -> (String, RunState) {
        let mut interp = Interpreter::new(lang);
        interp.load(source);
        interp.run();
        // Run up to 10_000 steps to prevent infinite loops in tests
        let mut total = 0;
        while total < 10_000 {
            if !interp.step_batch() {
                break;
            }
            total += interp.batch_size;
        }
        (interp.output(), interp.state)
    }

    #[test]
    fn basic_print_works() {
        let (out, state) = run_to_completion(Language::Basic, r#"10 PRINT "Hello"
20 END
"#);
        assert!(matches!(state, RunState::Finished));
        assert!(out.contains("Hello"), "Expected 'Hello' in output, got: {out:?}");
    }

    #[test]
    fn basic_sample_program() {
        let source = Language::Basic.sample_program();
        let (out, state) = run_to_completion(Language::Basic, source);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("Hello from BASIC!"), "output={out:?}");
        assert!(out.contains("Number:"), "output={out:?}");
    }

    #[test]
    fn logo_draws_lines() {
        let source = Language::Logo.sample_program();
        let mut interp = Interpreter::new(Language::Logo);
        interp.load(source);
        interp.run();
        while interp.step_batch() {}
        // The REPEAT 4 [FORWARD 100 RIGHT 90] should produce 4 lines
        assert!(!interp.ctx.turtle.lines.is_empty(),
            "Expected turtle lines, found none. output={}", interp.output());
    }

    #[test]
    fn logo_multiline_procedure() {
        // Multi-line TO…END with REPEAT spanning lines — the core bug this fixes
        let source = r#"; Test multi-line Logo
TO SQUARE :SIZE
    REPEAT 4 [
        FORWARD :SIZE
        RIGHT 90
    ]
END

SQUARE 80
"#;
        let mut interp = Interpreter::new(Language::Logo);
        interp.load(source);
        interp.run();
        while interp.step_batch() {}
        assert!(matches!(interp.state, RunState::Finished), "state={:?}", interp.state);
        // Should produce 4 line segments (80-unit square)
        assert_eq!(interp.ctx.turtle.lines.len(), 4,
            "Expected 4 lines for a square, got {}. Turtle pos=({}, {})",
            interp.ctx.turtle.lines.len(),
            interp.ctx.turtle.x, interp.ctx.turtle.y);
    }

    #[test]
    fn logo_print_bracket_text() {
        let source = "PRINT [Hello from Logo]\n";
        let (out, state) = run_to_completion(Language::Logo, source);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("Hello from Logo"), "output={out:?}");
    }

    #[test]
    fn logo_recursive_tree_no_crash() {
        // This is the exact pattern from 05_trees.logo that caused stack overflow.
        // Binary recursion: each TREE call makes 2 more TREE calls.
        let source = r#"TO TREE :SIZE :ANGLE
    IF :SIZE < 5 [ STOP ]
    FORWARD :SIZE
    LEFT :ANGLE
    TREE (:SIZE * 0.7) :ANGLE
    RIGHT (:ANGLE * 2)
    TREE (:SIZE * 0.7) :ANGLE
    LEFT :ANGLE
    BACKWARD :SIZE
END

TREE 120 25
"#;
        let mut interp = Interpreter::new(Language::Logo);
        interp.load(source);
        interp.run();
        while interp.step_batch() {}
        // Should finish without stack overflow
        assert!(matches!(interp.state, RunState::Finished),
            "state={:?}, output={}", interp.state, interp.output());
        // Should draw lines (binary tree)
        assert!(!interp.ctx.turtle.lines.is_empty(),
            "Expected tree lines, got none");
    }

    #[test]
    fn c_printf_works() {
        let (out, state) = run_to_completion(Language::C, r#"#include <stdio.h>
int main() {
    printf("Hi from C\n");
    return 0;
}
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("Hi from C"), "output={out:?}");
    }

    #[test]
    fn pascal_writeln_works() {
        let (out, state) = run_to_completion(Language::Pascal, r#"program Test;
begin
  writeln('Hello Pascal');
end.
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("Hello Pascal"), "output={out:?}");
    }

    #[test]
    fn forth_produces_output() {
        let (out, state) = run_to_completion(Language::Forth, "5 3 + . CR\n");
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("8"), "Expected '8' in output, got: {out:?}");
    }

    // ══════════════════════════════════════════════════════════════════════
    // BASIC — thorough tests
    // ══════════════════════════════════════════════════════════════════════

    #[test]
    fn basic_for_next_loop() {
        let (out, _) = run_to_completion(Language::Basic, r#"10 LET SUM = 0
20 FOR I = 1 TO 5
30     SUM = SUM + I
40 NEXT I
50 PRINT SUM
60 END
"#);
        assert!(out.contains("15"), "Expected '15', got: {out:?}");
    }

    #[test]
    fn basic_gosub_return() {
        let (out, _) = run_to_completion(Language::Basic, r#"10 GOSUB 100
20 PRINT "back"
30 END
100 PRINT "sub"
110 RETURN
"#);
        assert!(out.contains("sub"), "Missing 'sub' in: {out:?}");
        assert!(out.contains("back"), "Missing 'back' in: {out:?}");
    }

    #[test]
    fn basic_if_then_else() {
        let (out, _) = run_to_completion(Language::Basic, r#"10 LET X = 5
20 IF X > 3 THEN PRINT "big" ELSE PRINT "small"
30 END
"#);
        assert!(out.contains("big"), "Expected 'big', got: {out:?}");
    }

    #[test]
    fn basic_while_wend() {
        let (out, _) = run_to_completion(Language::Basic, r#"10 LET X = 1
20 WHILE X <= 3
30     PRINT X
40     X = X + 1
50 WEND
60 END
"#);
        assert!(out.contains("1"), "Missing '1' in: {out:?}");
        assert!(out.contains("2"), "Missing '2' in: {out:?}");
        assert!(out.contains("3"), "Missing '3' in: {out:?}");
    }

    #[test]
    fn basic_dim_array() {
        let (out, _) = run_to_completion(Language::Basic, r#"10 DIM A(3)
20 A(1) = 10
30 A(2) = 20
40 PRINT A(1) + A(2)
50 END
"#);
        assert!(out.contains("30"), "Expected '30', got: {out:?}");
    }

    #[test]
    fn basic_string_variable() {
        let (out, _) = run_to_completion(Language::Basic, r#"10 LET N$ = "World"
20 PRINT "Hello " + N$
30 END
"#);
        assert!(out.contains("Hello World"), "Expected 'Hello World', got: {out:?}");
    }

    // ══════════════════════════════════════════════════════════════════════
    // PILOT — thorough tests
    // ══════════════════════════════════════════════════════════════════════

    #[test]
    fn pilot_print_and_compute() {
        let (out, state) = run_to_completion(Language::Pilot, r#"COMPUTE x 5
COMPUTE y 3
COMPUTE sum $x + $y
PRINT Sum = $sum
STOP
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("Sum = 8"), "Expected 'Sum = 8', got: {out:?}");
    }

    #[test]
    fn pilot_match_case_block() {
        let (out, state) = run_to_completion(Language::Pilot, r#"COMPUTE val 42
MATCH $val
    CASE 42:
        PRINT found it
    DEFAULT:
        PRINT not found
END
STOP
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("found it"), "Expected 'found it', got: {out:?}");
    }

    #[test]
    fn pilot_classic_type_compute() {
        let (out, state) = run_to_completion(Language::Pilot, r#"C: X = 10
T: X is #X
E:
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("X is 10"), "Expected 'X is 10', got: {out:?}");
    }

    #[test]
    fn pilot_jump_and_labels() {
        let (out, state) = run_to_completion(Language::Pilot, r#"COMPUTE i 1
*loop
PRINT $i;
COMPUTE i $i + 1
MATCH $i
    CASE 4:
        JUMP done
    DEFAULT:
        JUMP loop
END
*done
PRINT  done
STOP
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("1"), "Missing '1' in: {out:?}");
        assert!(out.contains("3"), "Missing '3' in: {out:?}");
    }

    #[test]
    fn pilot_tu_rule_subroutine() {
        let (out, state) = run_to_completion(Language::Pilot, r#"TU greet
STOP
RULE greet:
    PRINT Hello from sub
RETURN
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("Hello from sub"), "Expected 'Hello from sub', got: {out:?}");
    }

    // ══════════════════════════════════════════════════════════════════════
    // Logo — thorough tests
    // ══════════════════════════════════════════════════════════════════════

    #[test]
    fn logo_make_and_print_variable() {
        let (out, state) = run_to_completion(Language::Logo, "MAKE \"X 10\nPRINT :X\n");
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("10"), "Expected '10', got: {out:?}");
    }

    #[test]
    fn logo_repeat_print() {
        let (out, state) = run_to_completion(Language::Logo, "REPEAT 3 [PRINT [hello]]\n");
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        let count = out.matches("hello").count();
        assert_eq!(count, 3, "Expected 3 'hello', got {count}. output={out:?}");
    }

    #[test]
    fn logo_if_conditional() {
        let (out, state) = run_to_completion(Language::Logo, "IF 1 [PRINT [yes]]\nIF 0 [PRINT [no]]\n");
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("yes"), "Missing 'yes' in: {out:?}");
        assert!(!out.contains("no"), "Should not contain 'no'. output={out:?}");
    }

    #[test]
    fn logo_procedure_with_param() {
        let (out, state) = run_to_completion(Language::Logo, r#"TO DOUBLE :N
    PRINT :N * 2
END
DOUBLE 7
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("14"), "Expected '14', got: {out:?}");
    }

    #[test]
    fn logo_stop_exits_procedure_not_program() {
        // STOP should exit only the current procedure, allowing the next line to run
        let (out, state) = run_to_completion(Language::Logo, r#"TO TEST :N
    IF :N < 1 [ STOP ]
    PRINT :N
END
TEST 0
PRINT [after]
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("after"), "Expected program to continue after STOP. output={out:?}");
        assert!(!out.contains("0"), "Should not print 0. output={out:?}");
    }

    // ══════════════════════════════════════════════════════════════════════
    // C — thorough tests
    // ══════════════════════════════════════════════════════════════════════

    #[test]
    fn c_variable_and_increment() {
        let (out, _) = run_to_completion(Language::C, r#"int x = 5;
x++;
printf("%d\n", x);
"#);
        assert!(out.contains("6"), "Expected '6', got: {out:?}");
    }

    #[test]
    fn c_compound_assignment() {
        let (out, _) = run_to_completion(Language::C, r#"int x = 10;
x += 5;
printf("%d\n", x);
"#);
        assert!(out.contains("15"), "Expected '15', got: {out:?}");
    }

    #[test]
    fn c_const_int_declaration() {
        let (out, _) = run_to_completion(Language::C, r#"const int N = 12;
printf("%d\n", N);
"#);
        assert!(out.contains("12"), "Expected '12', got: {out:?}");
    }

    #[test]
    fn c_if_else() {
        let (out, _) = run_to_completion(Language::C, r#"int x = 5;
if (x > 3) {
    printf("big\n");
} else {
    printf("small\n");
}
"#);
        assert!(out.contains("big"), "Expected 'big', got: {out:?}");
        assert!(!out.contains("small"), "Should not contain 'small'. output={out:?}");
    }

    #[test]
    fn c_while_loop() {
        let (out, _) = run_to_completion(Language::C, r#"int i = 1;
while (i <= 3) {
    printf("%d\n", i);
    i++;
}
"#);
        assert!(out.contains("1"), "Missing '1' in: {out:?}");
        assert!(out.contains("3"), "Missing '3' in: {out:?}");
    }

    #[test]
    fn c_for_loop() {
        let (out, _) = run_to_completion(Language::C, r#"int sum = 0;
for (int i = 1; i <= 5; i++) {
    sum += i;
}
printf("%d\n", sum);
"#);
        assert!(out.contains("15"), "Expected '15', got: {out:?}");
    }

    // ══════════════════════════════════════════════════════════════════════
    // Pascal — thorough tests
    // ══════════════════════════════════════════════════════════════════════

    #[test]
    fn pascal_variable_assignment() {
        let (out, state) = run_to_completion(Language::Pascal, r#"program Test;
var
  x: Integer;
begin
  x := 42;
  writeln(x);
end.
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("42"), "Expected '42', got: {out:?}");
    }

    #[test]
    fn pascal_if_then_else() {
        let (out, _) = run_to_completion(Language::Pascal, r#"program Test;
begin
  if 5 > 3 then writeln('yes')
  else writeln('no');
end.
"#);
        assert!(out.contains("yes"), "Expected 'yes', got: {out:?}");
    }

    #[test]
    fn pascal_string_writeln() {
        let (out, _) = run_to_completion(Language::Pascal, r#"program Test;
begin
  writeln('Hello', ' ', 'World');
end.
"#);
        assert!(out.contains("Hello World"), "Expected 'Hello World', got: {out:?}");
    }

    #[test]
    fn pascal_for_loop() {
        let (out, _) = run_to_completion(Language::Pascal, r#"program Test;
var
  i: Integer;
  sum: Integer;
begin
  sum := 0;
  for i := 1 to 5 do
    sum := sum + i;
  writeln(sum);
end.
"#);
        assert!(out.contains("15"), "Expected '15', got: {out:?}");
    }

    // ══════════════════════════════════════════════════════════════════════
    // Forth — thorough tests
    // ══════════════════════════════════════════════════════════════════════

    #[test]
    fn forth_dup_multiply() {
        let (out, _) = run_to_completion(Language::Forth, "3 DUP * . CR\n");
        assert!(out.contains("9"), "Expected '9', got: {out:?}");
    }

    #[test]
    fn forth_word_definition() {
        let (out, _) = run_to_completion(Language::Forth, ": DOUBLE 2 * ;\n5 DOUBLE . CR\n");
        assert!(out.contains("10"), "Expected '10', got: {out:?}");
    }

    #[test]
    fn forth_if_then() {
        let (out, _) = run_to_completion(Language::Forth, "10 0 > IF 1 . THEN CR\n");
        assert!(out.contains("1"), "Expected '1', got: {out:?}");
    }

    #[test]
    fn forth_dot_quote_string() {
        let (out, _) = run_to_completion(Language::Forth, ".\" Hello Forth\" CR\n");
        assert!(out.contains("Hello Forth"), "Expected 'Hello Forth', got: {out:?}");
    }

    #[test]
    fn forth_begin_until_loop() {
        let (out, _) = run_to_completion(Language::Forth, "1 BEGIN DUP . 1+ DUP 4 > UNTIL DROP CR\n");
        assert!(out.contains("1"), "Missing '1' in: {out:?}");
        assert!(out.contains("2"), "Missing '2' in: {out:?}");
        assert!(out.contains("3"), "Missing '3' in: {out:?}");
    }

    #[test]
    fn forth_parentheses_comment() {
        let (out, _) = run_to_completion(Language::Forth, "( this is a comment ) 42 . CR\n");
        assert!(out.contains("42"), "Expected '42', got: {out:?}");
    }

    // ══════════════════════════════════════════════════════════════════════
    // Prolog — thorough tests
    // ══════════════════════════════════════════════════════════════════════

    #[test]
    fn prolog_simple_query() {
        let (out, state) = run_to_completion(Language::Prolog, r#"parent(alice, bob).
parent(bob, carol).
?- parent(alice, bob).
"#);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("true"), "Expected 'true', got: {out:?}");
    }

    #[test]
    fn prolog_variable_binding() {
        let (out, _) = run_to_completion(Language::Prolog, r#"parent(alice, bob).
parent(bob, carol).
?- parent(alice, X).
"#);
        assert!(out.contains("bob"), "Expected 'bob' in solution, got: {out:?}");
    }

    #[test]
    fn prolog_multiple_solutions() {
        let (out, _) = run_to_completion(Language::Prolog, r#"likes(alice, cats).
likes(bob, cats).
likes(carol, dogs).
?- likes(X, cats).
"#);
        assert!(out.contains("alice"), "Missing 'alice' in: {out:?}");
        assert!(out.contains("bob"), "Missing 'bob' in: {out:?}");
        assert!(out.contains("2 solutions"), "Expected '2 solutions', got: {out:?}");
    }

    #[test]
    fn prolog_rule_resolution() {
        let (out, _) = run_to_completion(Language::Prolog, r#"parent(a, b).
parent(b, c).
grandparent(X, Z) :- parent(X, Y), parent(Y, Z).
?- grandparent(a, c).
"#);
        assert!(out.contains("true"), "Expected 'true' for grandparent(a,c), got: {out:?}");
    }

    #[test]
    fn prolog_recursive_rule() {
        let (out, _) = run_to_completion(Language::Prolog, r#"parent(a, b).
parent(b, c).
parent(c, d).
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).
?- ancestor(a, d).
"#);
        assert!(out.contains("true"), "Expected 'true' for ancestor(a,d), got: {out:?}");
    }

    #[test]
    fn prolog_false_query() {
        let (out, _) = run_to_completion(Language::Prolog, r#"parent(a, b).
?- parent(b, a).
"#);
        assert!(out.contains("false"), "Expected 'false', got: {out:?}");
    }

    // ── INPUT tests ───────────────────────────────────────────────────────

    /// Helper: run until WaitingInput, provide a response, then run to completion.
    fn run_with_input(lang: Language, source: &str, inputs: &[&str]) -> (String, RunState) {
        let mut interp = Interpreter::new(lang);
        interp.load(source);
        interp.run();
        let mut input_idx = 0;
        let mut total = 0;
        loop {
            if !interp.step_batch() { break; }
            if matches!(interp.state, RunState::WaitingInput) {
                if input_idx < inputs.len() {
                    interp.provide_input(inputs[input_idx]);
                    input_idx += 1;
                } else {
                    break; // no more inputs to provide
                }
            }
            total += interp.batch_size;
            if total > 50_000 { break; } // safety
        }
        (interp.output(), interp.state)
    }

    #[test]
    fn basic_input_resumes_after_response() {
        let source = r#"10 INPUT "Name: "; NAME$
20 PRINT "Hello, "; NAME$
30 END
"#;
        let (out, state) = run_with_input(Language::Basic, source, &["Alice"]);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("Hello"), "Expected greeting in output, got: {out:?}");
        assert!(out.contains("Alice"), "Expected 'Alice' in output, got: {out:?}");
    }

    #[test]
    fn basic_input_numeric() {
        let source = r#"10 INPUT "Number: "; N
20 LET R = N * 2
30 PRINT R
40 END
"#;
        let (out, state) = run_with_input(Language::Basic, source, &["7"]);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("14"), "Expected '14' in output, got: {out:?}");
    }

    #[test]
    fn pilot_accept_resumes_after_response() {
        let source = "T:What is your name?\nA:NAME\nT:Hello, #NAME!\nE:\n";
        let (out, state) = run_with_input(Language::Pilot, source, &["Bob"]);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("Hello, Bob"), "Expected 'Hello, Bob' in output, got: {out:?}");
    }

    #[test]
    fn c_scanf_resumes_after_response() {
        let source = r#"#include <stdio.h>
int main() {
    int x;
    scanf("%d", &x);
    printf("Got: %d\n", x);
    return 0;
}
"#;
        let (out, state) = run_with_input(Language::C, source, &["42"]);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("Got: 42"), "Expected 'Got: 42' in output, got: {out:?}");
    }

    #[test]
    fn pascal_readln_resumes_after_response() {
        let source = r#"program Test;
var
  n : integer;
begin
  readln(n);
  writeln('Value: ', n);
end.
"#;
        let (out, state) = run_with_input(Language::Pascal, source, &["99"]);
        assert!(matches!(state, RunState::Finished), "state={state:?}");
        assert!(out.contains("Value: 99"), "Expected 'Value: 99' in output, got: {out:?}");
    }
}

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
        // BASIC: pre-scan for SUB/FUNCTION definitions so CALL works
        // even when the sub appears after the call site.
        if self.language == Language::Basic {
            self.prescan_basic_subs();
        }
        // C: pre-scan for user-defined function definitions
        if self.language == Language::C {
            self.prescan_c_functions();
        }
        self.state = RunState::Running;
    }

    /// Pre-scan BASIC program for SUB/FUNCTION definitions.
    fn prescan_basic_subs(&mut self) {
        let lines = &self.ctx.program_lines;
        let mut i = 0;
        while i < lines.len() {
            let up = lines[i].1.trim().to_uppercase();
            if up.starts_with("SUB ") || up.starts_with("FUNCTION ") {
                let header = if up.starts_with("SUB ") {
                    lines[i].1.trim()[4..].trim()
                } else {
                    lines[i].1.trim()[9..].trim()
                };
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
                let start = i + 1;
                let mut end_idx = lines.len();
                for j in start..lines.len() {
                    let line_up = lines[j].1.trim().to_uppercase();
                    if line_up == "END SUB" || line_up == "END FUNCTION" {
                        end_idx = j;
                        break;
                    }
                    body.push(lines[j].1.clone());
                }

                use tw_languages::context::SubDef;
                self.ctx.subs.insert(name_part.clone(), SubDef {
                    name: name_part,
                    params,
                    body_lines: body,
                });
                i = end_idx + 1;
                continue;
            }
            i += 1;
        }
    }

    /// Pre-scan C program for user-defined function definitions.
    fn prescan_c_functions(&mut self) {
        let lines = &self.ctx.program_lines;
        let c_type_kw = ["INT", "FLOAT", "DOUBLE", "CHAR", "LONG", "SHORT", "UNSIGNED", "VOID", "CONST", "STATIC", "SIGNED"];
        let mut i = 0;
        while i < lines.len() {
            let trimmed = lines[i].1.trim();
            let up = trimmed.to_uppercase();
            // Detect function definitions: type name(...) {
            // Must contain '(' and end with '{', and not be main
            if trimmed.ends_with('{') && trimmed.contains('(') {
                let first_word = up.split_whitespace().next().unwrap_or("");
                if c_type_kw.contains(&first_word) && !up.contains("MAIN") {
                    // Extract function name
                    let before_paren = trimmed.split('(').next().unwrap_or("");
                    let name = before_paren.split_whitespace().last().unwrap_or("").to_uppercase();
                    // Extract params
                    let params_str = if let (Some(op), Some(cp)) = (trimmed.find('('), trimmed.find(')')) {
                        &trimmed[op+1..cp]
                    } else { "" };
                    let params: Vec<String> = params_str.split(',')
                        .map(|p| p.split_whitespace().last().unwrap_or("").to_uppercase())
                        .filter(|p| !p.is_empty())
                        .collect();

                    // Collect body until matching '}'
                    let mut body = Vec::new();
                    let mut depth = 1i32;
                    let start = i + 1;
                    let mut end_idx = lines.len();
                    for j in start..lines.len() {
                        let line = lines[j].1.trim();
                        for ch in line.chars() {
                            if ch == '{' { depth += 1; }
                            if ch == '}' { depth -= 1; }
                        }
                        if depth <= 0 {
                            end_idx = j;
                            break;
                        }
                        body.push(lines[j].1.clone());
                    }

                    use tw_languages::context::SubDef;
                    self.ctx.subs.insert(name.clone(), SubDef {
                        name,
                        params,
                        body_lines: body,
                    });
                    i = end_idx + 1;
                    continue;
                }
            }
            i += 1;
        }
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

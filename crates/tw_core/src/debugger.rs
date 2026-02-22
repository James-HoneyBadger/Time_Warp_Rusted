//! Execution timeline and frame capture — port of `core/debugger.py`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A snapshot of a single variable at one moment in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableSnapshot {
    pub name:  String,
    pub value: VariableValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableValue {
    Number(f64),
    Text(String),
    Array(Vec<f64>),
}

/// A single captured execution step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionFrame {
    pub step_number: usize,
    pub line_number: u32,
    pub source_line: String,
    pub variables:   Vec<VariableSnapshot>,
    pub output_so_far: String,
}

/// Records, stores, and navigates an ordered list of `ExecutionFrame`s.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExecutionTimeline {
    pub frames:       Vec<ExecutionFrame>,
    pub current_step: usize,
    pub breakpoints:  Vec<u32>,   // line numbers with breakpoints set
}

impl ExecutionTimeline {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a new frame.
    pub fn record_frame(
        &mut self,
        line_number: u32,
        source_line: &str,
        vars: &HashMap<String, f64>,
        str_vars: &HashMap<String, String>,
        arrays: &HashMap<String, Vec<f64>>,
        output_so_far: &str,
    ) {
        let step = self.frames.len();
        let mut variables = Vec::new();

        let mut sorted_num: Vec<_> = vars.iter().collect();
        sorted_num.sort_by_key(|(k, _)| k.as_str());
        for (k, v) in sorted_num {
            variables.push(VariableSnapshot { name: k.clone(), value: VariableValue::Number(*v) });
        }

        let mut sorted_str: Vec<_> = str_vars.iter().collect();
        sorted_str.sort_by_key(|(k, _)| k.as_str());
        for (k, v) in sorted_str {
            variables.push(VariableSnapshot { name: format!("{k}$"), value: VariableValue::Text(v.clone()) });
        }

        let mut sorted_arr: Vec<_> = arrays.iter().collect();
        sorted_arr.sort_by_key(|(k, _)| k.as_str());
        for (k, v) in sorted_arr {
            variables.push(VariableSnapshot { name: format!("{k}[]"), value: VariableValue::Array(v.clone()) });
        }

        self.frames.push(ExecutionFrame {
            step_number: step,
            line_number,
            source_line: source_line.to_string(),
            variables,
            output_so_far: output_so_far.to_string(),
        });
    }

    /// Jump to a particular step index.  Returns `true` if valid.
    pub fn seek(&mut self, step: usize) -> bool {
        if step < self.frames.len() {
            self.current_step = step;
            true
        } else {
            false
        }
    }

    pub fn step_forward(&mut self) -> bool {
        self.seek(self.current_step + 1)
    }

    pub fn step_backward(&mut self) -> bool {
        if self.current_step > 0 {
            self.seek(self.current_step - 1)
        } else {
            false
        }
    }

    pub fn current_frame(&self) -> Option<&ExecutionFrame> {
        self.frames.get(self.current_step)
    }

    pub fn total_steps(&self) -> usize {
        self.frames.len()
    }

    pub fn clear(&mut self) {
        self.frames.clear();
        self.current_step = 0;
    }

    /// Check whether `line_number` has a breakpoint.
    pub fn has_breakpoint(&self, line: u32) -> bool {
        self.breakpoints.contains(&line)
    }

    pub fn toggle_breakpoint(&mut self, line: u32) {
        if let Some(pos) = self.breakpoints.iter().position(|&l| l == line) {
            self.breakpoints.remove(pos);
        } else {
            self.breakpoints.push(line);
        }
    }

    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
    }
}

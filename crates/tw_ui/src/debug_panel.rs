//! Debugger panel — variables inspector, timeline slider, breakpoints.
//! Port of `ui/debug_panel.py`.

use egui::{Grid, ScrollArea, Slider, Ui};
use tw_core::debugger::{ExecutionTimeline, VariableValue};
use crate::themes::Theme;

pub struct DebugPanel {
    pub visible: bool,
}

impl Default for DebugPanel {
    fn default() -> Self { Self { visible: false } }
}

impl DebugPanel {
    pub fn new() -> Self { Self::default() }

    /// Returns: (step_forward, step_back, seek_to) button events.
    pub fn show(
        &mut self,
        ui: &mut Ui,
        timeline: &mut ExecutionTimeline,
        theme: &Theme,
    ) -> (bool, bool, Option<usize>) {
        let mut go_forward = false;
        let mut go_back    = false;
        let mut seek_to    = None;

        if !self.visible { return (false, false, None); }

        let total = timeline.total_steps();
        let step  = timeline.current_step;

        ui.separator();
        ui.colored_label(theme.accent(), "⏱ Debugger");
        ui.separator();

        // Timeline slider
        ui.horizontal(|ui| {
            ui.label("Step:");
            let mut s = step;
            let slider = Slider::new(&mut s, 0..=total.saturating_sub(1))
                .clamping(egui::SliderClamping::Always)
                .show_value(true);
            if ui.add(slider).changed() {
                seek_to = Some(s);
            }
            ui.label(format!("/ {total}"));
        });

        // Navigation buttons
        ui.horizontal(|ui| {
            if ui.button("⏮").clicked() { seek_to = Some(0); }
            if ui.button("⏪").clicked() { go_back = true; }
            if ui.button("⏩").clicked() { go_forward = true; }
            if ui.button("⏭").clicked() { seek_to = Some(total.saturating_sub(1)); }
        });

        // Current frame info
        if let Some(frame) = timeline.current_frame() {
            ui.separator();
            ui.label(format!("Line {}: {}", frame.line_number, frame.source_line.trim()));
            ui.separator();

            ui.label("Variables:");
            ScrollArea::vertical()
                .id_salt("debug_vars")
                .max_height(200.0)
                .show(ui, |ui| {
                    Grid::new("var_grid")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| {
                            for var in &frame.variables {
                                ui.label(&var.name);
                                let val_str = match &var.value {
                                    VariableValue::Number(v) => {
                                        if *v == v.floor() && v.abs() < 1e15 {
                                            format!("{}", *v as i64)
                                        } else {
                                            format!("{v:.4}")
                                        }
                                    }
                                    VariableValue::Text(s)  => format!("\"{s}\""),
                                    VariableValue::Array(a) => format!("[{}]", a.iter()
                                        .take(8)
                                        .map(|v| v.to_string())
                                        .collect::<Vec<_>>()
                                        .join(", ")),
                                };
                                ui.label(val_str);
                                ui.end_row();
                            }
                        });
                });
        }

        // Breakpoints list
        if !timeline.breakpoints.is_empty() {
            ui.separator();
            ui.label("Breakpoints:");
            let bps: Vec<u32> = timeline.breakpoints.clone();
            for bp in bps {
                ui.horizontal(|ui| {
                    ui.colored_label(theme.error(), format!("🔴 Line {bp}"));
                    if ui.small_button("✕").clicked() {
                        timeline.toggle_breakpoint(bp);
                    }
                });
            }
        }

        (go_forward, go_back, seek_to)
    }
}

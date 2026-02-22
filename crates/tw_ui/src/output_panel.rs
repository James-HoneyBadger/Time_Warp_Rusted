//! Text output console — tabbed: Output | Errors | Console history.

use egui::{Color32, FontId, RichText, ScrollArea, Ui};
use crate::themes::Theme;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputTab {
    Output,
    Errors,
    Console,
}

pub struct OutputPanel {
    pub text:          String,
    pub errors:        String,
    pub console:       String,   // history of all stdin/stdout exchanges
    pub font_size:     f32,
    pub active_tab:    OutputTab,
}

impl Default for OutputPanel {
    fn default() -> Self {
        Self {
            text:         String::new(),
            errors:       String::new(),
            console:      String::new(),
            font_size:    13.0,
            active_tab:   OutputTab::Output,
        }
    }
}

impl OutputPanel {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, text: &str) {
        self.text.push_str(text);
        self.console.push_str(text);
    }

    pub fn append_error(&mut self, msg: &str) {
        if !self.errors.is_empty() { self.errors.push('\n'); }
        self.errors.push_str(msg);
        // Auto-switch to Errors tab so the user notices
        self.active_tab = OutputTab::Errors;
    }

    pub fn append_input_echo(&mut self, prompt: &str, value: &str) {
        let line = format!("{prompt}{value}\n");
        self.console.push_str(&line);
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn clear_all(&mut self) {
        self.text.clear();
        self.errors.clear();
        self.console.clear();
    }

    pub fn set(&mut self, text: String) {
        self.text = text;
    }

    /// Number of error lines (0 = clean).
    pub fn error_count(&self) -> usize {
        if self.errors.is_empty() { 0 } else { self.errors.lines().count() }
    }

    pub fn show(&mut self, ui: &mut Ui, theme: &Theme) {
        // Guard: skip rendering if the available area is degenerate
        let avail = ui.available_size();
        if !avail.x.is_finite() || !avail.y.is_finite() || avail.x < 4.0 || avail.y < 4.0 {
            return;
        }

        let bg = theme.output_bg();
        let fg = theme.output_fg();

        // ── tab bar ───────────────────────────────────────────────────────────
        ui.horizontal(|ui| {
            let err_n = self.error_count();
            let out_chars = self.text.len();
            let out_lbl = if out_chars > 0 {
                RichText::new(format!("📄 Output ({out_chars} chars)"))
            } else {
                RichText::new("📄 Output")
            };
            let err_lbl = if err_n > 0 {
                RichText::new(format!("⚠ Errors ({err_n})")).color(Color32::from_rgb(230, 80, 60))
            } else {
                RichText::new("⚠ Errors")
            };
            let con_lbl = RichText::new("💬 Console");

            if ui.selectable_label(self.active_tab == OutputTab::Output,  out_lbl).clicked() {
                self.active_tab = OutputTab::Output;
            }
            if ui.selectable_label(self.active_tab == OutputTab::Errors,  err_lbl).clicked() {
                self.active_tab = OutputTab::Errors;
            }
            if ui.selectable_label(self.active_tab == OutputTab::Console, con_lbl).clicked() {
                self.active_tab = OutputTab::Console;
            }
        });
        ui.separator();

        // ── tab body ──────────────────────────────────────────────────────────
        egui::Frame::NONE.fill(bg).inner_margin(4.0).show(ui, |ui| {
            match self.active_tab {
                OutputTab::Output => {
                    ScrollArea::vertical()
                        .id_salt("output_scroll")
                        .auto_shrink([false; 2])
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            if self.text.is_empty() {
                                ui.colored_label(
                                    Color32::from_rgb(120, 120, 120),
                                    "(no output yet — click ▶ Run or press F5)",
                                );
                            } else {
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(&self.text)
                                            .font(FontId::monospace(self.font_size))
                                            .color(fg),
                                    )
                                    .selectable(true),
                                );
                            }
                        });
                }
                OutputTab::Errors => {
                    if self.errors.is_empty() {
                        ui.colored_label(Color32::from_rgb(100, 200, 100), "✔ No errors.");
                    } else {
                        ScrollArea::vertical()
                            .id_salt("errors_scroll")
                            .auto_shrink([false; 2])
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(&self.errors)
                                            .font(FontId::monospace(self.font_size))
                                            .color(Color32::from_rgb(230, 80, 60)),
                                    )
                                    .selectable(true),
                                );
                            });
                    }
                }
                OutputTab::Console => {
                    ScrollArea::vertical()
                        .id_salt("console_scroll")
                        .auto_shrink([false; 2])
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            if self.console.is_empty() {
                                ui.colored_label(
                                    Color32::from_rgb(120, 120, 120),
                                    "(no console output)",
                                );
                            } else {
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(&self.console)
                                            .font(FontId::monospace(self.font_size))
                                            .color(fg),
                                    )
                                    .selectable(true),
                                );
                            }
                        });
                }
            }
        });
    }
}

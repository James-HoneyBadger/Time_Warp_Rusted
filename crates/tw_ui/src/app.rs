//! Main `TimeWarpApp` — the root egui application.
//! Port of `ui/main_window.py`.

use std::path::PathBuf;

use eframe::{CreationContext, Frame};
use egui::{
    Align, CentralPanel, Context, Key, Layout, Modifiers,
    RichText, SidePanel, TopBottomPanel,
};
use tw_core::{interpreter::{Interpreter, RunState}, language::Language};

use crate::{
    canvas::TurtleCanvas,
    debug_panel::DebugPanel,
    editor::CodeEditor,
    feature_panels::{FeaturePanels, Panel},
    output_panel::OutputPanel,
    themes::ThemeManager,
};

// ── split orientation ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
enum CanvasSplit {
    Horizontal,
    Vertical,
    EditorOnly,
    CanvasOnly,
}

// ── app state ─────────────────────────────────────────────────────────────────

pub struct TimeWarpApp {
    interpreter:       Interpreter,
    themes:            ThemeManager,
    editor:            CodeEditor,
    output:            OutputPanel,
    canvas:            TurtleCanvas,
    debug_panel:       DebugPanel,
    feature_pane:      FeaturePanels,
    split:             CanvasSplit,
    left_panel:        bool,
    debug_mode:        bool,
    input_buf:         String,
    status:            String,
    current_lang:      Language,
    // file management
    current_file:      Option<PathBuf>,
    unsaved:           bool,
    // find bar
    find_open:         bool,
    find_query:        String,
    // breakpoint input
    bp_line_buf:       String,
}

impl TimeWarpApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        let lang = Language::Basic;
        let mut editor = CodeEditor::new(lang);
        editor.source = lang.sample_program().to_string();

        Self {
            interpreter:  Interpreter::new(lang),
            themes:       ThemeManager::new(),
            editor,
            output:       OutputPanel::new(),
            canvas:       TurtleCanvas::new(),
            debug_panel:  DebugPanel::new(),
            feature_pane: FeaturePanels::new(),
            split:        CanvasSplit::Horizontal,
            left_panel:   true,
            debug_mode:   false,
            input_buf:    String::new(),
            status:       "Ready".to_string(),
            current_lang: lang,
            current_file: None,
            unsaved:      false,
            find_open:    false,
            find_query:   String::new(),
            bp_line_buf:  String::new(),
        }
    }

    // ── window title ─────────────────────────────────────────────────────

    fn window_title(&self) -> String {
        let file = self.current_file.as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Untitled".to_string());
        let dirty = if self.unsaved { " •" } else { "" };
        format!("⏱ Time Warp Studio — {}{}", file, dirty)
    }

    // ── keyboard shortcuts ────────────────────────────────────────────────

    fn handle_shortcuts(&mut self, ctx: &Context) {
        // F5 — Run
        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::F5)) {
            self.action_run();
        }
        // F6 — Stop
        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::F6)) {
            self.action_stop();
        }
        // F7 — Step
        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::F7)) {
            self.action_step();
        }
        // Ctrl+R — Run (alternate)
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::R)) {
            self.action_run();
        }
        // Ctrl+L — clear output
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::L)) {
            self.output.clear();
        }
        // Ctrl+= zoom in, Ctrl+- zoom out
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Equals)) {
            self.editor.zoom_in();
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Minus)) {
            self.editor.zoom_out();
        }
        // Ctrl+N — New
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::N)) {
            self.action_new();
        }
        // Ctrl+O — Open
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::O)) {
            self.action_open();
        }
        // Ctrl+S — Save
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::S)) {
            self.action_save();
        }
        // Ctrl+Shift+S — Save As
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::S)) {
            self.action_save_as();
        }
        // Ctrl+F — Find
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::F)) {
            self.find_open = !self.find_open;
        }
        // Escape — close find bar
        if self.find_open && ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape)) {
            self.find_open = false;
        }
    }

    // ── file actions ─────────────────────────────────────────────────────

    fn action_new(&mut self) {
        self.editor.source.clear();
        self.current_file = None;
        self.unsaved = false;
        self.output.clear_all();
        self.interpreter.stop();
        self.status = "New file".to_string();
    }

    fn action_open(&mut self) {
        let ext = self.current_lang.extension();
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(&format!("{} files", self.current_lang.friendly_name()), &[ext])
            .add_filter("All source files", &[
                "bas", "basic",          // BASIC
                "pil", "pilot",          // PILOT
                "logo", "lg",            // Logo
                "c", "h",               // C
                "pas", "pascal", "pp",   // Pascal
                "pro", "prolog", "pl",   // Prolog
                "fth", "4th", "fs", "forth", "f", // Forth
            ])
            .add_filter("Text files", &["txt"])
            .add_filter("All files", &["*"])
            .pick_file()
        {
            match std::fs::read_to_string(&path) {
                Ok(src) => {
                    // Infer language from extension
                    if let Some(lang) = lang_from_path(&path) {
                        self.change_language(lang);
                    }
                    self.editor.source = src;
                    self.current_file = Some(path.clone());
                    self.unsaved = false;
                    self.status = format!("Opened {}", path.display());
                }
                Err(e) => {
                    self.status = format!("Open failed: {e}");
                    self.output.append_error(&format!("Open error: {e}"));
                }
            }
        }
    }

    fn action_save(&mut self) {
        if let Some(path) = self.current_file.clone() {
            self.write_file(&path);
        } else {
            self.action_save_as();
        }
    }

    fn action_save_as(&mut self) {
        let ext = self.current_lang.extension();
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Source files", &[ext])
            .add_filter("All files", &["*"])
            .set_file_name(format!("program.{ext}"))
            .save_file()
        {
            self.write_file(&path);
            self.current_file = Some(path);
        }
    }

    fn write_file(&mut self, path: &PathBuf) {
        match std::fs::write(path, &self.editor.source) {
            Ok(_) => {
                self.unsaved = false;
                self.status = format!("Saved {}", path.display());
            }
            Err(e) => {
                self.status = format!("Save failed: {e}");
                self.output.append_error(&format!("Save error: {e}"));
            }
        }
    }

    fn action_export_output(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text files", &["txt"])
            .set_file_name("output.txt")
            .save_file()
        {
            match std::fs::write(&path, &self.output.text) {
                Ok(_) => { self.status = format!("Output exported to {}", path.display()); }
                Err(e) => { self.status = format!("Export failed: {e}"); }
            }
        }
    }

    // ── run actions ───────────────────────────────────────────────────────

    fn action_run(&mut self) {
        let line_count = self.editor.source.lines().count();
        eprintln!("[TW] action_run: {line_count} lines of {:?}", self.current_lang);
        self.interpreter.load(&self.editor.source);
        self.interpreter.record = self.debug_mode;
        self.interpreter.run();
        self.output.clear_all();
        self.canvas.reset_view();
        self.status = "Running…".to_string();
    }

    fn action_stop(&mut self) {
        self.interpreter.stop();
        self.status = "Stopped".to_string();
    }

    fn action_step(&mut self) {
        if matches!(self.interpreter.state, RunState::Idle) {
            self.interpreter.load(&self.editor.source);
            self.interpreter.record = true;
            self.interpreter.run();
        }
        let saved_batch = self.interpreter.batch_size;
        self.interpreter.batch_size = 1;
        self.interpreter.step_batch();
        self.interpreter.batch_size = saved_batch;
        self.sync_output();
    }

    fn action_clear(&mut self) {
        self.interpreter.stop();
        self.output.clear_all();
        self.interpreter.ctx.turtle.clear_screen();
        self.status = "Cleared".to_string();
    }

    fn action_toggle_breakpoint(&mut self) {
        if let Ok(n) = self.bp_line_buf.trim().parse::<u32>() {
            self.interpreter.timeline.toggle_breakpoint(n);
            self.status = format!("Breakpoint toggled at line {n}");
        }
    }

    fn change_language(&mut self, lang: Language) {
        self.current_lang = lang;
        self.interpreter.language = lang;
        self.editor.set_language(lang);
        if self.editor.source.trim().is_empty() {
            self.editor.source = lang.sample_program().to_string();
        }
        self.status = format!("Language: {}", lang.friendly_name());
    }

    fn sync_output(&mut self) {
        let text = self.interpreter.output();
        if !text.is_empty() {
            eprintln!("[TW] sync_output: {} chars, state={:?}", text.len(), self.interpreter.state);
        }
        // Surface any runtime error into the errors tab
        if let RunState::Error(e) = &self.interpreter.state.clone() {
            self.output.append_error(&format!("Runtime error: {e}"));
        }
        // Switch to Output tab so the user can see results
        if !text.is_empty() && !matches!(self.interpreter.state, RunState::Error(_)) {
            self.output.active_tab = crate::output_panel::OutputTab::Output;
        }
        self.output.set(text);
    }

    // ── menu bar ──────────────────────────────────────────────────────────

    fn menu_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {

                // ── File ─────────────────────────────────────────────────
                ui.menu_button("File", |ui| {
                    if ui.add(egui::Button::new("📄 New")
                        .shortcut_text("Ctrl+N")).clicked()
                    {
                        self.action_new(); ui.close_menu();
                    }
                    if ui.add(egui::Button::new("📂 Open…")
                        .shortcut_text("Ctrl+O")).clicked()
                    {
                        self.action_open(); ui.close_menu();
                    }
                    ui.separator();
                    if ui.add(egui::Button::new("💾 Save")
                        .shortcut_text("Ctrl+S")).clicked()
                    {
                        self.action_save(); ui.close_menu();
                    }
                    if ui.add(egui::Button::new("💾 Save As…")
                        .shortcut_text("Ctrl+Shift+S")).clicked()
                    {
                        self.action_save_as(); ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("📤 Export Output…").clicked() {
                        self.action_export_output(); ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🚪 Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                // ── Edit ─────────────────────────────────────────────────
                ui.menu_button("Edit", |ui| {
                    if ui.add(egui::Button::new("🔍 Find / Highlight")
                        .shortcut_text("Ctrl+F")).clicked()
                    {
                        self.find_open = !self.find_open; ui.close_menu();
                    }
                    ui.separator();
                    if ui.add(egui::Button::new("🔎 Zoom In")
                        .shortcut_text("Ctrl+=")).clicked()
                    {
                        self.editor.zoom_in(); ui.close_menu();
                    }
                    if ui.add(egui::Button::new("🔍 Zoom Out")
                        .shortcut_text("Ctrl+-")).clicked()
                    {
                        self.editor.zoom_out(); ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🗑 Clear Output").clicked() {
                        self.output.clear_all(); ui.close_menu();
                    }
                });

                // ── Run ──────────────────────────────────────────────────
                ui.menu_button("Run", |ui| {
                    let running = matches!(
                        self.interpreter.state,
                        RunState::Running | RunState::WaitingInput
                    );
                    if ui.add_enabled(
                        !running,
                        egui::Button::new("▶ Run").shortcut_text("F5"),
                    ).clicked() {
                        self.action_run(); ui.close_menu();
                    }
                    if ui.add_enabled(
                        !running,
                        egui::Button::new("⏭ Step").shortcut_text("F7"),
                    ).clicked() {
                        self.action_step(); ui.close_menu();
                    }
                    if ui.add_enabled(
                        running,
                        egui::Button::new("⏹ Stop").shortcut_text("F6"),
                    ).clicked() {
                        self.action_stop(); ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🗑 Clear & Reset").clicked() {
                        self.action_clear(); ui.close_menu();
                    }
                    ui.separator();
                    ui.label("Breakpoint at line:");
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.bp_line_buf)
                            .on_hover_text("Line number");
                        if ui.button("🔴 Toggle").clicked() {
                            self.action_toggle_breakpoint();
                        }
                    });
                });

                // ── View ─────────────────────────────────────────────────
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.left_panel, "Left panel");
                    ui.checkbox(&mut self.debug_mode, "Debug mode");
                    ui.separator();
                    for (label, split) in [
                        ("⬛ Horizontal split",   CanvasSplit::Horizontal),
                        ("⬜ Vertical split",     CanvasSplit::Vertical),
                        ("📝 Editor only",        CanvasSplit::EditorOnly),
                        ("🖼 Canvas only",         CanvasSplit::CanvasOnly),
                    ] {
                        let sel = self.split == split;
                        if ui.selectable_label(sel, label).clicked() {
                            self.split = split;
                            ui.close_menu();
                        }
                    }
                    ui.separator();
                    ui.label("Theme:");
                    for name in self.themes.theme_names()
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                    {
                        let sel = self.themes.current == name;
                        if ui.selectable_label(sel, &name).clicked() {
                            self.themes.set_theme(&name);
                            ui.close_menu();
                        }
                    }
                });

                // ── Language ─────────────────────────────────────────────
                ui.menu_button("Language", |ui| {
                    for lang in Language::all() {
                        let sel = *lang == self.current_lang;
                        if ui.selectable_label(sel, lang.friendly_name()).clicked() {
                            self.change_language(*lang);
                            ui.close_menu();
                        }
                    }
                });

                // ── Help ─────────────────────────────────────────────────
                ui.menu_button("Help", |ui| {
                    if ui.button("📚 Lessons").clicked() {
                        self.feature_pane.active = Panel::Lessons;
                        self.left_panel = true;
                        ui.close_menu();
                    }
                    if ui.button("📁 Examples").clicked() {
                        self.feature_pane.active = Panel::Examples;
                        self.left_panel = true;
                        ui.close_menu();
                    }
                    if ui.button("ℹ About").clicked() {
                        self.feature_pane.active = Panel::About;
                        self.left_panel = true;
                        ui.close_menu();
                    }
                });
            });
        });
    }

    // ── toolbar ───────────────────────────────────────────────────────────

    fn toolbar(&mut self, ctx: &Context) {
        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Language selector
                egui::ComboBox::from_id_salt("lang_combo")
                    .selected_text(self.current_lang.friendly_name())
                    .show_ui(ui, |ui| {
                        for lang in Language::all() {
                            let sel = *lang == self.current_lang;
                            if ui.selectable_label(sel, lang.friendly_name()).clicked() {
                                self.change_language(*lang);
                            }
                        }
                    });

                ui.separator();

                // Run / step / stop / clear
                let running = matches!(self.interpreter.state, RunState::Running | RunState::WaitingInput);
                let theme = self.themes.current().clone();
                if ui.add_enabled(!running, egui::Button::new(RichText::new("▶ Run").color(theme.success()))).clicked() {
                    self.action_run();
                }
                if ui.add_enabled(!running, egui::Button::new("⏭ Step")).clicked() {
                    self.action_step();
                }
                if ui.add_enabled(running, egui::Button::new(RichText::new("⏹ Stop").color(theme.error()))).clicked() {
                    self.action_stop();
                }
                if ui.button("🗑 Clear").clicked() {
                    self.action_clear();
                }
                if ui.button("🔁 Reset view").clicked() {
                    self.canvas.reset_view();
                }

                ui.separator();
                ui.checkbox(&mut self.debug_mode, "Debug");
                ui.checkbox(&mut self.left_panel, "Panel");

                // Unsaved indicator
                if self.unsaved {
                    ui.colored_label(theme.warning(), "●");
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    match &self.interpreter.state {
                        RunState::Running       => { ui.colored_label(theme.success(), "● Running"); }
                        RunState::WaitingInput  => { ui.colored_label(theme.warning(), "● Input…"); }
                        RunState::Finished      => { ui.colored_label(theme.accent(),  "✔ Done"); }
                        RunState::Error(e)      => { ui.colored_label(theme.error(),   format!("✗ {e}")); }
                        RunState::Idle          => {}
                    }
                    ui.label(&self.status);
                });
            });
        });
    }

    // ── find bar ─────────────────────────────────────────────────────────

    fn find_bar(&mut self, ctx: &Context) {
        if !self.find_open { return; }

        TopBottomPanel::top("find_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🔍 Find:");
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut self.find_query)
                        .desired_width(280.0)
                        .hint_text("Search text…"),
                );
                // Count occurrences
                let src = self.editor.source.clone();
                let q = self.find_query.to_lowercase();
                if q.is_empty() {
                    ui.label("—");
                } else {
                    let count = src.to_lowercase().matches(&q).count();
                    ui.label(format!("{count} match{}", if count == 1 { "" } else { "es" }));
                }
                if ui.button("✕ Close").clicked() || resp.lost_focus() && ctx.input(|i| i.key_pressed(Key::Escape)) {
                    self.find_open = false;
                }
            });
        });
    }

    // ── input bar ─────────────────────────────────────────────────────────

    fn input_bar(&mut self, ctx: &Context) {
        let waiting = matches!(self.interpreter.state, RunState::WaitingInput);
        if !waiting { return; }

        let prompt: String = self.interpreter.ctx.input_requests
            .last()
            .map(|(p, _, _)| p.clone())
            .unwrap_or_else(|| "? ".to_string());

        TopBottomPanel::bottom("input_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(self.themes.current().warning(), "⌨ Input:");
                ui.label(&prompt);
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut self.input_buf)
                        .desired_width(400.0)
                        .hint_text("Type response and press Enter…"),
                );
                let submit = (resp.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)))
                    || ui.button("Submit ↵").clicked();
                if submit {
                    let text = std::mem::take(&mut self.input_buf);
                    self.output.append_input_echo(&prompt, &text);
                    self.interpreter.provide_input(&text);
                    self.status = "Running…".to_string();
                }
                // Focus the input field automatically when waiting
                if resp.gained_focus() || waiting {
                    resp.request_focus();
                }
            });
        });
    }

    // ── status bar ────────────────────────────────────────────────────────

    fn status_bar(&self, ctx: &Context) {
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Build marker — proves the user is running the latest code
                ui.label("[b5]");
                ui.separator();
                let file_str = self.current_file.as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "Untitled".to_string());
                ui.label(&file_str);
                ui.separator();
                ui.label(format!("Lang: {}", self.current_lang.friendly_name()));
                ui.separator();
                let lines = self.editor.source.lines().count();
                let chars = self.editor.source.len();
                ui.label(format!("{lines} lines  {chars} chars"));
                ui.separator();
                // Show output char count — visible proof independent of output panel
                let out_len = self.output.text.len();
                if out_len > 0 {
                    let theme = self.themes.current();
                    ui.colored_label(
                        theme.success(),
                        format!("Output: {out_len} chars"),
                    );
                    ui.separator();
                }
                let err_n = self.output.error_count();
                if err_n > 0 {
                    let theme = self.themes.current();
                    ui.colored_label(theme.error(), format!("⚠ {err_n} error{}", if err_n == 1 { "" } else { "s" }));
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(format!("Font: {}pt", self.editor.font_size as u32));
                });
            });
        });
    }

    // ── central workspace ─────────────────────────────────────────────────

    fn central(&mut self, ctx: &Context) {
        let theme = self.themes.current().clone();

        match self.split {
            // ── Horizontal: editor on top, canvas+output on the bottom ──────
            CanvasSplit::Horizontal => {
                // Add the bottom panel first (panels must come before CentralPanel).
                TopBottomPanel::bottom("h_bottom")
                    .resizable(true)
                    .min_height(80.0)
                    .default_height(300.0)
                    .show(ctx, |ui| {
                        ui.visuals_mut().extreme_bg_color = theme.editor_bg();
                        // Split canvas (left) and output (right) side by side.
                        egui::SidePanel::left("h_canvas")
                            .resizable(true)
                            .min_width(80.0)
                            .default_width(400.0)
                            .show_inside(ui, |ui| {
                                self.canvas.show(ui, &self.interpreter.ctx.turtle);
                            });
                        // Output fills the remaining right space.
                        self.output.show(ui, &theme);
                    });
                // Editor fills the remaining central area.
                CentralPanel::default().show(ctx, |ui| {
                    ui.visuals_mut().extreme_bg_color = theme.editor_bg();
                    let changed = self.editor.show(ui, &theme);
                    if changed { self.unsaved = true; }
                });
            }
            // ── Vertical: editor on left, canvas+output on right ────────────
            CanvasSplit::Vertical => {
                // Right panel for canvas+output.
                egui::SidePanel::right("v_right")
                    .resizable(true)
                    .min_width(200.0)
                    .default_width(500.0)
                    .show(ctx, |ui| {
                        ui.visuals_mut().extreme_bg_color = theme.editor_bg();
                        // Output at the bottom of the right panel.
                        TopBottomPanel::bottom("v_output")
                            .resizable(true)
                            .min_height(80.0)
                            .default_height(250.0)
                            .show_inside(ui, |ui| {
                                self.output.show(ui, &theme);
                            });
                        // Canvas fills the remaining top space.
                        self.canvas.show(ui, &self.interpreter.ctx.turtle);
                    });
                // Editor on the left.
                CentralPanel::default().show(ctx, |ui| {
                    ui.visuals_mut().extreme_bg_color = theme.editor_bg();
                    let changed = self.editor.show(ui, &theme);
                    if changed { self.unsaved = true; }
                });
            }
            // ── Editor only ──────────────────────────────────────────────────
            CanvasSplit::EditorOnly => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.visuals_mut().extreme_bg_color = theme.editor_bg();
                    let changed = self.editor.show(ui, &theme);
                    if changed { self.unsaved = true; }
                });
            }
            // ── Canvas only ──────────────────────────────────────────────────
            CanvasSplit::CanvasOnly => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.visuals_mut().extreme_bg_color = theme.editor_bg();
                    self.canvas.show(ui, &self.interpreter.ctx.turtle);
                });
            }
        }
    }
}

// ── helpers ────────────────────────────────────────────────────────────────────

fn lang_from_path(path: &PathBuf) -> Option<Language> {
    let ext = path.extension()?.to_str()?.to_lowercase();
    Language::from_extension(&ext)
}

// ── eframe App impl ────────────────────────────────────────────────────────────

impl eframe::App for TimeWarpApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Update window title
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.window_title()));

        // ── 1. Handle all user interactions first ─────────────────────────
        // This ensures that action_run() (triggered by toolbar, menu, or
        // keyboard shortcut) sets state=Running BEFORE step_batch checks it.

        // Handle pending example load
        if let Some((lang, src)) = self.feature_pane.pending_load.take() {
            self.change_language(lang);
            self.editor.source = src;
            self.action_run();
        }

        self.handle_shortcuts(ctx);
        self.menu_bar(ctx);
        self.toolbar(ctx);
        self.status_bar(ctx);   // bottom, before input_bar
        self.input_bar(ctx);    // bottom-most, only when waiting
        self.find_bar(ctx);     // top, just below toolbar

        // ── 2. Execute program if running ─────────────────────────────────
        // Now that interactions have been processed, step_batch will catch
        // any newly-started run in the SAME frame (no lost frames).
        if matches!(self.interpreter.state, RunState::Running) {
            let still_going = self.interpreter.step_batch();
            self.sync_output();
            if !still_going {
                self.status = match &self.interpreter.state {
                    RunState::Finished    => "Finished".to_string(),
                    RunState::Error(e)    => format!("Error: {e}"),
                    _                     => "Stopped".to_string(),
                };
                // Push error into errors tab if applicable
                if let RunState::Error(e) = &self.interpreter.state.clone() {
                    self.output.append_error(&format!("Runtime error: {e}"));
                }
            }
            // Always request repaint when actively executing.
            // This is critical for both multi-frame runs AND single-frame
            // completions—egui reactive mode won't repaint without this.
            ctx.request_repaint();
        }

        // ── 3. Render panels ──────────────────────────────────────────────

        // Left panel
        if self.left_panel {
            SidePanel::left("left_panel")
                .resizable(true)
                .default_width(240.0)
                .show(ctx, |ui| {
                    let mut themes = std::mem::take(&mut self.themes);
                    self.feature_pane.show(ui, &mut themes);
                    self.themes = themes;
                    ui.separator();
                    let debug_visible = self.debug_mode;
                    self.debug_panel.visible = debug_visible;
                    let theme_clone = self.themes.current().clone();
                    let (fwd, bck, seek) = self.debug_panel.show(
                        ui, &mut self.interpreter.timeline, &theme_clone);
                    if fwd  { self.interpreter.timeline.step_forward(); }
                    if bck  { self.interpreter.timeline.step_backward(); }
                    if let Some(s) = seek { self.interpreter.timeline.seek(s); }
                });
        }

        self.central(ctx);
    }
}


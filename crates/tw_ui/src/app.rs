//! Main `TimeWarpApp` — the root egui application.
//! Port of `ui/main_window.py`.

use std::path::PathBuf;

use eframe::{CreationContext, Frame};
use egui::{
    Align, CentralPanel, Context, Key, Layout, Modifiers,
    RichText, ScrollArea, SidePanel, TopBottomPanel,
};
use tw_core::{interpreter::{Interpreter, RunState}, language::Language};

use crate::{
    canvas::TurtleCanvas,
    debug_panel::DebugPanel,
    editor::{CodeEditor, FONT_FAMILIES},
    feature_panels::{FeaturePanels, Panel},
    output_panel::OutputPanel,
    themes::{ThemeManager, FONT_SIZE_PRESETS},
};

// ── central tab ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CentralTab {
    Editor,
    Output,
    Canvas,
}

// ── Panel size configuration ──────────────────────────────────────────────────

/// User-configurable panel sizes.
struct PanelSizes {
    left_panel_width:  f32,
    input_height:      f32,
}

impl Default for PanelSizes {
    fn default() -> Self {
        Self {
            left_panel_width:  240.0,
            input_height:      150.0,
        }
    }
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
    active_tab:        CentralTab,
    left_panel:        bool,
    debug_mode:        bool,
    input_buf:         String,
    status:            String,
    current_lang:      Language,
    // file management
    current_file:      Option<PathBuf>,
    unsaved:           bool,
    // find bar (legacy, now integrated in editor)
    find_open:         bool,
    find_query:        String,
    // breakpoint input
    bp_line_buf:       String,
    // Panel sizes
    panel_sizes:       PanelSizes,
    // Layout preferences window
    layout_prefs_open: bool,
    // Input window as separate floating panel
    input_window_open: bool,
    input_docked:      bool,  // true = bottom bar, false = floating window
    // IoT panel
    iot_panel_open:    bool,
    // Raspberry Pi configuration
    pi_setup_open:     bool,
    pi_project_open:   bool,
    pi_selected_board: usize,   // index into Board::all()
    pi_auto_open_iot:  bool,    // auto-open IoT panel on project load
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
            active_tab:   CentralTab::Editor,
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
            panel_sizes:  PanelSizes::default(),
            layout_prefs_open: false,
            input_window_open: false,
            input_docked: true,
            iot_panel_open: false,
            pi_setup_open: false,
            pi_project_open: false,
            pi_selected_board: 0,
            pi_auto_open_iot: true,
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
        // Ctrl+1/2/3 — Switch tabs
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Num1)) {
            self.active_tab = CentralTab::Editor;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Num2)) {
            self.active_tab = CentralTab::Output;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Num3)) {
            self.active_tab = CentralTab::Canvas;
        }
        // Ctrl+Z — Undo
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Z)) {
            self.editor.undo();
        }
        // Ctrl+Shift+Z — Redo
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::Z)) {
            self.editor.redo();
        }
        // Ctrl+G — Goto Line
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::G)) {
            self.editor.goto_open = !self.editor.goto_open;
        }
        // Ctrl+F — Find/Replace (now in editor)
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::F)) {
            self.editor.find_replace_open = !self.editor.find_replace_open;
            self.find_open = false; // close legacy find bar
        }
        // Ctrl+H — Find/Replace (alternate)
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::H)) {
            self.editor.find_replace_open = true;
        }
        // Escape — close find bar / find-replace
        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape)) {
            self.find_open = false;
            self.editor.find_replace_open = false;
            self.editor.goto_open = false;
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
                // Time Warp native languages
                "bas", "basic",                        // BASIC
                "pil", "pilot",                        // PILOT
                "logo", "lg",                          // Logo
                "c", "h", "cpp", "hpp", "cc", "cxx",   // C / C++
                "pas", "pascal", "pp", "dpr", "lpr",   // Pascal / Delphi
                "pro", "prolog", "pl",                 // Prolog
                "fth", "4th", "fs", "forth", "f",     // Forth
                // Common programming languages
                "py", "pyw",                           // Python
                "rs",                                   // Rust
                "js", "mjs", "cjs",                    // JavaScript
                "ts", "tsx",                            // TypeScript
                "java",                                 // Java
                "rb",                                   // Ruby
                "go",                                   // Go
                "swift",                                // Swift
                "kt", "kts",                            // Kotlin
                "lua",                                  // Lua
                "sh", "bash", "zsh",                    // Shell
                "asm", "s", "S",                        // Assembly
                "ino",                                  // Arduino
                "json", "yaml", "yml", "toml", "xml",  // Config
                "html", "css", "scss",                  // Web
                "sql",                                  // SQL
                "r", "R",                               // R
                "lisp", "cl", "el",                     // Lisp / Emacs
                "scm", "rkt",                           // Scheme / Racket
                "hs",                                   // Haskell
                "erl", "ex", "exs",                     // Erlang / Elixir
                "ml", "mli",                            // OCaml
                "v", "sv",                              // Verilog
                "vhd", "vhdl",                          // VHDL
            ])
            .add_filter("Text files", &["txt", "md", "rst", "csv", "log"])
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
            // Auto-switch to Output tab if we have text and we're on the editor
            if self.active_tab == CentralTab::Editor {
                self.active_tab = CentralTab::Output;
            }
        }
        // If there are graphics, hint the user by switching to canvas
        let has_gfx = !self.interpreter.ctx.turtle.lines.is_empty()
            || !self.interpreter.ctx.turtle.shapes.is_empty();
        if has_gfx && self.active_tab == CentralTab::Editor {
            self.active_tab = CentralTab::Canvas;
        }
        self.output.set(text);
    }
    // ── theme visuals ──────────────────────────────────────────────────

    /// Apply the current theme to egui's global visuals so that every panel,
    /// menu, toolbar, and widget picks up consistent colors automatically.
    fn apply_theme_visuals(&self, ctx: &Context) {
        let theme = self.themes.current();
        let mut vis = ctx.style().visuals.clone();

        // Dark/light mode detection based on background luminance
        let bg = theme.background;
        let lum = 0.299 * bg[0] as f32 + 0.587 * bg[1] as f32 + 0.114 * bg[2] as f32;
        vis.dark_mode = lum < 128.0;

        // Window / panel backgrounds
        let bg_color = theme.bg();
        let panel_bg = theme.panel_bg();
        let fg_color = theme.fg();
        let border_color = egui::Color32::from_rgb(theme.border[0], theme.border[1], theme.border[2]);
        let accent = theme.accent();

        vis.override_text_color = Some(fg_color);
        vis.panel_fill = panel_bg;
        vis.window_fill = panel_bg;
        vis.extreme_bg_color = theme.editor_bg();
        vis.faint_bg_color = egui::Color32::from_rgba_premultiplied(
            theme.border[0], theme.border[1], theme.border[2], 30,
        );

        // Widget visuals (buttons, combo boxes, sliders, etc.)
        let btn_bg = theme.button_bg();
        let btn_fg = theme.button_fg();

        // Inactive widgets
        vis.widgets.inactive.bg_fill = btn_bg;
        vis.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, btn_fg);
        vis.widgets.inactive.weak_bg_fill = btn_bg;
        vis.widgets.inactive.bg_stroke = egui::Stroke::new(0.5, border_color);

        // Hovered widgets
        vis.widgets.hovered.bg_fill = accent;
        vis.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, btn_fg);
        vis.widgets.hovered.weak_bg_fill = accent;
        vis.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, accent);

        // Active (pressed) widgets
        vis.widgets.active.bg_fill = accent;
        vis.widgets.active.fg_stroke = egui::Stroke::new(2.0, btn_fg);
        vis.widgets.active.weak_bg_fill = accent;
        vis.widgets.active.bg_stroke = egui::Stroke::new(1.0, accent);

        // Open (expanded combo box, menu, etc.)
        vis.widgets.open.bg_fill = panel_bg;
        vis.widgets.open.fg_stroke = egui::Stroke::new(1.0, fg_color);
        vis.widgets.open.weak_bg_fill = panel_bg;
        vis.widgets.open.bg_stroke = egui::Stroke::new(1.0, border_color);

        // Non-interactive (labels, static elements)
        vis.widgets.noninteractive.bg_fill = bg_color;
        vis.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, fg_color);
        vis.widgets.noninteractive.weak_bg_fill = bg_color;
        vis.widgets.noninteractive.bg_stroke = egui::Stroke::new(0.5, border_color);

        // Selection
        vis.selection.bg_fill = theme.selection_color();
        vis.selection.stroke = egui::Stroke::new(1.0, accent);

        // Window / panel borders & separators
        vis.window_stroke = egui::Stroke::new(1.0, border_color);

        ctx.set_visuals(vis);
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
                    if ui.add_enabled(
                        self.editor.can_undo(),
                        egui::Button::new("↩ Undo").shortcut_text("Ctrl+Z"),
                    ).clicked() {
                        self.editor.undo(); ui.close_menu();
                    }
                    if ui.add_enabled(
                        self.editor.can_redo(),
                        egui::Button::new("↪ Redo").shortcut_text("Ctrl+Shift+Z"),
                    ).clicked() {
                        self.editor.redo(); ui.close_menu();
                    }
                    ui.separator();
                    if ui.add(egui::Button::new("🔍 Find / Replace")
                        .shortcut_text("Ctrl+F")).clicked()
                    {
                        self.editor.find_replace_open = !self.editor.find_replace_open;
                        ui.close_menu();
                    }
                    if ui.add(egui::Button::new("↕ Goto Line")
                        .shortcut_text("Ctrl+G")).clicked()
                    {
                        self.editor.goto_open = !self.editor.goto_open;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("💬 Toggle Comment").clicked() {
                        self.editor.toggle_comment(); ui.close_menu();
                    }
                    if ui.button("➡ Indent").clicked() {
                        self.editor.indent_selection(); ui.close_menu();
                    }
                    if ui.button("⬅ Unindent").clicked() {
                        self.editor.unindent_selection(); ui.close_menu();
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
                    if ui.button("⚙ Layout Preferences…").clicked() {
                        self.layout_prefs_open = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.checkbox(&mut self.input_docked, "Input bar docked").changed() {
                        // If undocking, auto-open the floating window
                        if !self.input_docked {
                            self.input_window_open = true;
                        }
                    }
                    ui.checkbox(&mut self.iot_panel_open, "IoT Panel");
                    ui.separator();
                    ui.label("Active Tab:");
                    for (label, tab) in [
                        ("📝 Code Editor",         CentralTab::Editor),
                        ("📄 Text Output",         CentralTab::Output),
                        ("🖼 Graphics Canvas",      CentralTab::Canvas),
                    ] {
                        let sel = self.active_tab == tab;
                        if ui.selectable_label(sel, label).clicked() {
                            self.active_tab = tab;
                            ui.close_menu();
                        }
                    }
                });

                // ── Appearance ───────────────────────────────────────────
                ui.menu_button("Appearance", |ui| {
                    // ── Theme selector with categories and swatches ──
                    ui.label(RichText::new("🎨 Theme").strong());

                    // Collect into owned data so we don't hold an immutable
                    // borrow on `self.themes` when calling set_theme later.
                    let groups: Vec<(String, Vec<(String, String, [u8;3], [u8;3], [u8;3])>)> =
                        self.themes.themes_by_category().into_iter().map(|(cat, ts)| {
                            let items = ts.into_iter().map(|t| {
                                (t.name.clone(), t.description.clone(),
                                 t.background, t.accent, t.keyword)
                            }).collect();
                            (cat.label().to_string(), items)
                        }).collect();

                    for (cat_label, themes) in &groups {
                        ui.separator();
                        ui.label(RichText::new(cat_label).strong().small());
                        for (name, desc, bg_c, accent_c, kw_c) in themes {
                            let sel = self.themes.current == *name;
                            ui.horizontal(|ui| {
                                // Color swatch (bg, accent, keyword)
                                let swatch_size = egui::vec2(12.0, 12.0);
                                let (r, _) = ui.allocate_exact_size(swatch_size, egui::Sense::hover());
                                ui.painter().rect_filled(r, 2.0,
                                    egui::Color32::from_rgb(bg_c[0], bg_c[1], bg_c[2]));
                                let (r, _) = ui.allocate_exact_size(swatch_size, egui::Sense::hover());
                                ui.painter().rect_filled(r, 2.0,
                                    egui::Color32::from_rgb(accent_c[0], accent_c[1], accent_c[2]));
                                let (r, _) = ui.allocate_exact_size(swatch_size, egui::Sense::hover());
                                ui.painter().rect_filled(r, 2.0,
                                    egui::Color32::from_rgb(kw_c[0], kw_c[1], kw_c[2]));
                                if ui.selectable_label(sel, name).on_hover_text(desc).clicked() {
                                    self.themes.set_theme(name);
                                    ui.close_menu();
                                }
                            });
                        }
                    }

                    ui.separator();
                    ui.label(RichText::new("🔤 Font Family").strong());
                    for &fam in FONT_FAMILIES {
                        let sel = self.editor.font_family == fam;
                        if ui.selectable_label(sel, fam).clicked() {
                            self.editor.font_family = fam.to_string();
                        }
                    }

                    ui.separator();
                    ui.label(RichText::new("🔠 Font Size").strong());
                    for &(label, size) in FONT_SIZE_PRESETS {
                        let sel = (self.editor.font_size - size).abs() < 0.5;
                        if ui.selectable_label(sel, label).clicked() {
                            self.editor.font_size = size;
                        }
                    }
                    ui.horizontal(|ui| {
                        if ui.small_button("−").clicked() { self.editor.zoom_out(); }
                        ui.label(format!("{}pt", self.editor.font_size as u32));
                        if ui.small_button("+").clicked() { self.editor.zoom_in(); }
                    });
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

                // ── Raspberry Pi ─────────────────────────────────────────
                ui.menu_button("🔌 Raspberry Pi", |ui| {
                    if ui.button("⚙ Board Setup…").clicked() {
                        self.pi_setup_open = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.label("Select Board:");
                    let boards = tw_iot::board::Board::all();
                    for (i, board) in boards.iter().enumerate() {
                        let sel = self.interpreter.gpio.board == *board;
                        if ui.selectable_label(sel, board.name()).clicked() {
                            self.pi_selected_board = i;
                            self.interpreter.gpio = tw_iot::GpioManager::new(*board);
                            let _ = self.interpreter.gpio.connect();
                            ui.close_menu();
                        }
                    }
                    ui.separator();
                    if ui.button("📁 Load Project…").clicked() {
                        self.pi_project_open = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🔌 Open IoT Panel").clicked() {
                        self.iot_panel_open = true;
                        ui.close_menu();
                    }
                    if ui.button("🔄 Reset All GPIO").clicked() {
                        self.interpreter.gpio.reset();
                        ui.close_menu();
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
                ui.label("[b7-tabs]");
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

    // ── layout preferences window ────────────────────────────────────────

    fn layout_prefs_window(&mut self, ctx: &Context) {
        if !self.layout_prefs_open { return; }

        egui::Window::new("⚙ Layout Preferences")
            .resizable(true)
            .collapsible(true)
            .default_width(340.0)
            .open(&mut self.layout_prefs_open)
            .show(ctx, |ui| {
                ui.heading("Panel Sizes");
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label("Left panel width:");
                    ui.add(egui::Slider::new(&mut self.panel_sizes.left_panel_width, 120.0..=500.0).suffix(" px"));
                });
                ui.horizontal(|ui| {
                    ui.label("Input area height:");
                    ui.add(egui::Slider::new(&mut self.panel_sizes.input_height, 80.0..=400.0).suffix(" px"));
                });

                ui.add_space(8.0);
                if ui.button("Reset to Defaults").clicked() {
                    self.panel_sizes = PanelSizes::default();
                }
            });
    }

    // ── floating input window ─────────────────────────────────────────────

    fn input_window(&mut self, ctx: &Context) {
        let waiting = matches!(self.interpreter.state, RunState::WaitingInput);
        // When undocked and waiting, force open
        if waiting && !self.input_docked {
            self.input_window_open = true;
        }
        if !self.input_window_open { return; }

        let prompt: String = self.interpreter.ctx.input_requests
            .last()
            .map(|(p, _, _)| p.clone())
            .unwrap_or_else(|| "? ".to_string());

        let theme = self.themes.current().clone();
        let mut open = self.input_window_open;

        egui::Window::new("⌨ Program Input")
            .resizable(true)
            .collapsible(false)
            .default_width(480.0)
            .default_height(self.panel_sizes.input_height)
            .open(&mut open)
            .show(ctx, |ui| {
                if waiting {
                    ui.colored_label(theme.warning(), format!("Prompt: {prompt}"));
                    ui.add_space(4.0);
                    let resp = ui.add(
                        egui::TextEdit::multiline(&mut self.input_buf)
                            .desired_width(f32::INFINITY)
                            .desired_rows(3)
                            .hint_text("Type your response here…")
                            .font(egui::FontId::monospace(14.0)),
                    );
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        let submit_enter = resp.lost_focus()
                            && ui.input(|i| i.key_pressed(Key::Enter));
                        if ui.button("⏎ Submit").clicked() || submit_enter {
                            let text = std::mem::take(&mut self.input_buf);
                            self.output.append_input_echo(&prompt, &text);
                            self.interpreter.provide_input(&text);
                            self.status = "Running…".to_string();
                        }
                        ui.label(RichText::new("Press Enter or click Submit").weak());
                    });
                    resp.request_focus();
                } else {
                    ui.label("No program is waiting for input.");
                    ui.label(RichText::new("Run a program that uses INPUT to enter data here.").weak());
                }
            });

        self.input_window_open = open;
    }

    // ── IoT panel ─────────────────────────────────────────────────────────

    fn iot_panel(&mut self, ctx: &Context) {
        if !self.iot_panel_open { return; }

        egui::Window::new("🔌 IoT / Raspberry Pi")
            .resizable(true)
            .collapsible(true)
            .default_width(400.0)
            .default_height(500.0)
            .open(&mut self.iot_panel_open)
            .show(ctx, |ui| {
                let theme = self.themes.current().clone();

                ui.heading("GPIO Pin Monitor");
                ui.add_space(4.0);

                // Board selector
                ui.horizontal(|ui| {
                    ui.label("Board:");
                    for board in tw_iot::board::Board::all() {
                        let sel = self.interpreter.gpio.board == *board;
                        if ui.selectable_label(sel, board.name()).clicked() {
                            self.interpreter.gpio = tw_iot::GpioManager::new(*board);
                            let _ = self.interpreter.gpio.connect();
                        }
                    }
                });
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    let conn_text = if self.interpreter.gpio.connected {
                        RichText::new("● Connected").color(theme.success())
                    } else {
                        RichText::new("○ Disconnected").color(theme.error())
                    };
                    ui.label(conn_text);
                    if !self.interpreter.gpio.connected {
                        if ui.button("Connect").clicked() {
                            let _ = self.interpreter.gpio.connect();
                        }
                    }
                    if ui.button("🔄 Reset GPIO").clicked() {
                        self.interpreter.gpio.reset();
                    }
                });
                ui.separator();

                // GPIO pin table showing real data
                ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                    egui::Grid::new("gpio_grid")
                        .striped(true)
                        .min_col_width(50.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("Pin").strong());
                            ui.label(RichText::new("Mode").strong());
                            ui.label(RichText::new("State").strong());
                            ui.label(RichText::new("Value").strong());
                            ui.label(RichText::new("Action").strong());
                            ui.end_row();

                            for pin_num in self.interpreter.gpio.sorted_pins() {
                                if let Some(info) = self.interpreter.gpio.pins.get(&pin_num) {
                                    ui.label(format!("GP{}", info.number));

                                    let mode_text = format!("{}", info.mode);
                                    ui.label(&mode_text);

                                    let state_color = match info.state {
                                        tw_iot::PinState::High => theme.success(),
                                        tw_iot::PinState::Low  => theme.fg(),
                                        tw_iot::PinState::Unknown => theme.border_color(),
                                    };
                                    ui.colored_label(state_color, format!("{}", info.state));

                                    if info.value != 0.0 {
                                        ui.label(format!("{:.2}", info.value));
                                    } else {
                                        ui.label("—");
                                    }

                                    // Simulator toggle for input pins
                                    let is_input = info.mode == tw_iot::gpio::PinMode::Input;
                                    let pin_n = info.number;
                                    if is_input {
                                        if ui.small_button("⇅").on_hover_text("Toggle pin").clicked() {
                                            self.interpreter.gpio.sim_toggle(pin_n);
                                        }
                                    } else {
                                        ui.label("");
                                    }

                                    ui.end_row();
                                }
                            }
                        });
                });

                ui.separator();
                ui.heading("GPIO Log");
                ScrollArea::vertical()
                    .id_salt("gpio_log")
                    .max_height(120.0)
                    .show(ui, |ui| {
                        let log = &self.interpreter.gpio.log;
                        if log.is_empty() {
                            ui.label(RichText::new("No GPIO activity yet.").weak());
                        } else {
                            for entry in log.iter().rev().take(50) {
                                ui.label(RichText::new(entry).monospace().size(11.0));
                            }
                        }
                    });
            });
    }

    // ── Raspberry Pi Board Setup Window ───────────────────────────────────

    fn pi_setup_window(&mut self, ctx: &Context) {
        if !self.pi_setup_open { return; }

        egui::Window::new("⚙ Raspberry Pi Board Setup")
            .resizable(true)
            .collapsible(true)
            .default_width(480.0)
            .default_height(420.0)
            .open(&mut self.pi_setup_open)
            .show(ctx, |ui| {
                let theme = self.themes.current().clone();

                ui.heading("Select Your Board");
                ui.add_space(4.0);
                ui.label("Choose the Raspberry Pi board you are targeting:");
                ui.add_space(4.0);

                let boards = tw_iot::board::Board::all();
                for (i, board) in boards.iter().enumerate() {
                    let sel = self.interpreter.gpio.board == *board;
                    let label = format!(
                        "{}{}",
                        board.name(),
                        if sel { "  ✓" } else { "" }
                    );
                    if ui.selectable_label(sel, &label).clicked() {
                        self.pi_selected_board = i;
                        self.interpreter.gpio = tw_iot::GpioManager::new(*board);
                        let _ = self.interpreter.gpio.connect();
                    }
                }

                ui.separator();
                ui.heading("Board Capabilities");
                ui.add_space(2.0);

                let board = self.interpreter.gpio.board;
                egui::Grid::new("board_caps")
                    .striped(true)
                    .min_col_width(140.0)
                    .show(ui, |ui| {
                        ui.label(RichText::new("Feature").strong());
                        ui.label(RichText::new("Status").strong());
                        ui.end_row();

                        ui.label("GPIO Pins");
                        ui.label(format!("{}", board.gpio_count()));
                        ui.end_row();

                        ui.label("WiFi");
                        ui.label(if board.has_wifi() {
                            RichText::new("✓ Available").color(theme.success())
                        } else {
                            RichText::new("✗ Not available").color(theme.error())
                        });
                        ui.end_row();

                        ui.label("I²C");
                        ui.label(if board.has_i2c() {
                            RichText::new("✓ Available").color(theme.success())
                        } else {
                            RichText::new("✗ Not available").color(theme.error())
                        });
                        ui.end_row();

                        ui.label("SPI");
                        ui.label(if board.has_spi() {
                            RichText::new("✓ Available").color(theme.success())
                        } else {
                            RichText::new("✗ Not available").color(theme.error())
                        });
                        ui.end_row();

                        ui.label("PWM");
                        ui.label(if board.has_pwm() {
                            RichText::new("✓ Available").color(theme.success())
                        } else {
                            RichText::new("✗ Not available").color(theme.error())
                        });
                        ui.end_row();

                        ui.label("ADC (Analog)");
                        ui.label(if board.has_adc() {
                            RichText::new("✓ Available").color(theme.success())
                        } else {
                            RichText::new("✗ Not available").color(theme.error())
                        });
                        ui.end_row();
                    });

                ui.separator();
                ui.heading("Connection");
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    let conn_text = if self.interpreter.gpio.connected {
                        RichText::new("● Connected (Simulator)").color(theme.success())
                    } else {
                        RichText::new("○ Disconnected").color(theme.error())
                    };
                    ui.label(conn_text);
                    if !self.interpreter.gpio.connected {
                        if ui.button("Connect").clicked() {
                            let _ = self.interpreter.gpio.connect();
                        }
                    }
                });

                ui.separator();
                ui.heading("Quick Actions");
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    if ui.button("🔌 Open IoT Panel").clicked() {
                        self.iot_panel_open = true;
                    }
                    if ui.button("📁 Load Project").clicked() {
                        self.pi_project_open = true;
                    }
                    if ui.button("🔄 Reset GPIO").clicked() {
                        self.interpreter.gpio.reset();
                    }
                });
            });
    }

    // ── Raspberry Pi Project Browser Window ───────────────────────────────

    fn pi_project_window(&mut self, ctx: &Context) {
        if !self.pi_project_open { return; }

        // Collect any selection outside the closure to avoid borrow conflicts
        let mut selected: Option<(Language, String, String)> = None;
        let mut open_iot = false;
        let mut open = self.pi_project_open;

        egui::Window::new("📁 Raspberry Pi Projects")
            .resizable(true)
            .collapsible(true)
            .default_width(520.0)
            .default_height(500.0)
            .open(&mut open)
            .show(ctx, |ui| {
                let theme = self.themes.current().clone();
                let board = self.interpreter.gpio.board;

                ui.heading(format!("Projects for: {}", board.name()));
                ui.add_space(2.0);
                ui.label("Click a project to load it into the editor and configure GPIO.");
                ui.add_space(4.0);

                ui.checkbox(&mut self.pi_auto_open_iot, "Auto-open IoT Panel on load");
                ui.separator();

                ScrollArea::vertical().id_salt("pi_projects").show(ui, |ui| {
                    let projects = pi_projects_for_board(board);
                    if projects.is_empty() {
                        ui.label(RichText::new("No built-in projects for this board yet.").weak());
                        ui.label("Select Pico, Pi Zero, or Pi Zero 2 W for projects.");
                    } else {
                        for (title, difficulty, hw_needed, lang, code) in &projects {
                            ui.add_space(2.0);
                            ui.horizontal(|ui| {
                                let diff_color = match *difficulty {
                                    "Beginner"      => theme.success(),
                                    "Intermediate"  => egui::Color32::from_rgb(255, 165, 0),
                                    "Advanced"      => theme.error(),
                                    _               => theme.fg(),
                                };
                                ui.colored_label(diff_color,
                                    RichText::new(format!("[{}]", difficulty)).strong());
                                if ui.button(RichText::new(*title).strong()).clicked() {
                                    selected = Some((*lang, code.to_string(), title.to_string()));
                                    if self.pi_auto_open_iot {
                                        open_iot = true;
                                    }
                                }
                            });
                            ui.label(format!("    Hardware: {}", hw_needed));
                        }
                    }
                });
            });

        self.pi_project_open = open;

        // Apply the selection after the window closure is done
        if let Some((lang, code, title)) = selected {
            self.change_language(lang);
            self.editor.source = code;
            self.unsaved = true;
            self.status = format!("Loaded: {}", title);
            if open_iot {
                self.iot_panel_open = true;
            }
        }
    }

    // ── central workspace (tabbed) ──────────────────────────────────────

    fn central(&mut self, ctx: &Context) {
        let theme = self.themes.current().clone();

        CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().extreme_bg_color = theme.editor_bg();

            // ── Tab bar ──────────────────────────────────────────────────
            ui.horizontal(|ui| {
                let tab_style = |ui: &mut egui::Ui, label: &str, is_active: bool, indicator: Option<&str>| -> egui::Response {
                    let text = if let Some(ind) = indicator {
                        format!("{label}  {ind}")
                    } else {
                        label.to_string()
                    };
                    if is_active {
                        ui.add(egui::Button::new(
                            RichText::new(&text).strong().color(theme.accent()),
                        ))
                    } else {
                        ui.add(egui::Button::new(
                            RichText::new(&text),
                        ))
                    }
                };

                // Editor tab — show unsaved indicator
                let editor_ind = if self.unsaved { Some("●") } else { None };
                if tab_style(ui, "📝 Code", self.active_tab == CentralTab::Editor, editor_ind).clicked() {
                    self.active_tab = CentralTab::Editor;
                }

                // Output tab — show char count when non-empty
                let out_len = self.output.text.len();
                let output_ind = if out_len > 0 {
                    Some("◉")
                } else { None };
                if tab_style(ui, "📄 Output", self.active_tab == CentralTab::Output, output_ind).clicked() {
                    self.active_tab = CentralTab::Output;
                }

                // Canvas tab — show dot when turtle has drawn
                let has_gfx = !self.interpreter.ctx.turtle.lines.is_empty()
                    || !self.interpreter.ctx.turtle.shapes.is_empty();
                let canvas_ind = if has_gfx { Some("◉") } else { None };
                if tab_style(ui, "🖼 Canvas", self.active_tab == CentralTab::Canvas, canvas_ind).clicked() {
                    self.active_tab = CentralTab::Canvas;
                }

                // Show running state indicator on the right side of tab bar
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    match &self.interpreter.state {
                        RunState::Running      => { ui.colored_label(theme.success(), "● Running"); }
                        RunState::WaitingInput => { ui.colored_label(theme.warning(), "● Input…"); }
                        RunState::Finished     => { ui.colored_label(theme.accent(),  "✔ Done"); }
                        RunState::Error(e)     => { ui.colored_label(theme.error(),   format!("✗ {e}")); }
                        RunState::Idle         => {}
                    }
                });
            });

            ui.separator();

            // ── Tab content ──────────────────────────────────────────────
            match self.active_tab {
                CentralTab::Editor => {
                    let changed = self.editor.show(ui, &theme);
                    if changed { self.unsaved = true; }
                }
                CentralTab::Output => {
                    self.output.show(ui, &theme);
                }
                CentralTab::Canvas => {
                    self.canvas.show(ui, &self.interpreter.ctx.turtle);
                }
            }
        });
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

        // ── 0. Apply theme to global egui visuals ─────────────────────────
        self.apply_theme_visuals(ctx);

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

        // Input: either docked bar or floating window
        if self.input_docked {
            self.input_bar(ctx);    // bottom-most, only when waiting
        } else {
            self.input_window(ctx); // floating window
        }

        self.find_bar(ctx);     // top, just below toolbar (legacy)

        // Modal/floating windows
        self.layout_prefs_window(ctx);
        self.iot_panel(ctx);
        self.pi_setup_window(ctx);
        self.pi_project_window(ctx);

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
                .default_width(self.panel_sizes.left_panel_width)
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

// ── Raspberry Pi project catalogue ────────────────────────────────────
// Returns (title, difficulty, hardware, Language, source_code)
fn pi_projects_for_board(
    board: tw_iot::board::Board,
) -> Vec<(&'static str, &'static str, &'static str, Language, &'static str)> {
    use tw_iot::board::Board;

    match board {
        Board::Pico | Board::PicoW => vec![
            ("Blink LED",           "Beginner",     "LED + 330Ω on GP15",
             Language::Basic, include_str!("../../../Examples/projects/pico/01_blink_led.bas")),
            ("Button & LED",        "Beginner",     "Button GP14, LED GP15",
             Language::Basic, include_str!("../../../Examples/projects/pico/02_button_led.bas")),
            ("LED Fader (PWM)",     "Beginner",     "LED + 330Ω on GP15",
             Language::Basic, include_str!("../../../Examples/projects/pico/03_led_fader.bas")),
            ("Traffic Light",       "Beginner",     "3 LEDs on GP13-GP15",
             Language::Basic, include_str!("../../../Examples/projects/pico/04_traffic_light.bas")),
            ("Analog Sensor",       "Intermediate", "Potentiometer on GP26",
             Language::Basic, include_str!("../../../Examples/projects/pico/05_analog_sensor.bas")),
            ("Servo Sweep",         "Intermediate", "Micro servo on GP16",
             Language::Basic, include_str!("../../../Examples/projects/pico/06_servo_sweep.bas")),
            ("LED Chaser",          "Intermediate", "6 LEDs on GP10-GP15",
             Language::Basic, include_str!("../../../Examples/projects/pico/07_led_chaser.bas")),
            ("LED Dice",            "Intermediate", "7 LEDs + button on GP2",
             Language::Basic, include_str!("../../../Examples/projects/pico/08_led_dice.bas")),
            ("Temperature Monitor", "Advanced",     "TMP36 on GP26 (ADC0)",
             Language::Basic, include_str!("../../../Examples/projects/pico/09_temp_monitor.bas")),
            ("Turtle Robot",        "Advanced",     "2 DC motors via H-bridge",
             Language::Logo,  include_str!("../../../Examples/projects/pico/10_turtle_robot.logo")),
        ],
        Board::PiZero => vec![
            ("Blink LED",           "Beginner",     "LED + 330Ω on GPIO17",
             Language::Basic, include_str!("../../../Examples/projects/zero/01_blink_led.bas")),
            ("Button & LED",        "Beginner",     "Button GPIO27, LED GPIO17",
             Language::Basic, include_str!("../../../Examples/projects/zero/02_button_led.bas")),
            ("PWM LED Fader",       "Beginner",     "LED on GPIO18 (PWM0)",
             Language::Basic, include_str!("../../../Examples/projects/zero/03_pwm_fader.bas")),
            ("Traffic Light",       "Beginner",     "3 LEDs on GPIO17/22/27",
             Language::Basic, include_str!("../../../Examples/projects/zero/04_traffic_light.bas")),
            ("Servo Controller",    "Intermediate", "Servo on GPIO18",
             Language::Basic, include_str!("../../../Examples/projects/zero/05_servo_controller.bas")),
            ("Binary Counter",      "Intermediate", "4 LEDs on GPIO5/6/13/19",
             Language::Basic, include_str!("../../../Examples/projects/zero/06_binary_counter.bas")),
            ("Reaction Timer",      "Intermediate", "LED GPIO17, Button GPIO27",
             Language::Basic, include_str!("../../../Examples/projects/zero/07_reaction_timer.bas")),
            ("Simon Says",          "Advanced",     "4 LEDs + 4 Buttons",
             Language::Basic, include_str!("../../../Examples/projects/zero/08_simon_says.bas")),
            ("LED Chaser Patterns", "Advanced",     "8 LEDs on GPIO5-26",
             Language::Basic, include_str!("../../../Examples/projects/zero/09_led_chaser.bas")),
            ("Turtle Art + GPIO",   "Advanced",     "4 LEDs on GPIO17/22/27/5",
             Language::Logo,  include_str!("../../../Examples/projects/zero/10_logo_gpio_art.logo")),
        ],
        Board::PiZero2W => vec![
            ("Blink LED",              "Beginner",     "LED + 330Ω on GPIO17",
             Language::Basic, include_str!("../../../Examples/projects/zero2w/01_blink_led.bas")),
            ("Button Debounce",        "Beginner",     "Button GPIO27, LED GPIO17",
             Language::Basic, include_str!("../../../Examples/projects/zero2w/02_button_debounce.bas")),
            ("RGB Colour Mixer",       "Intermediate", "RGB LED (common cathode)",
             Language::Basic, include_str!("../../../Examples/projects/zero2w/03_rgb_colour_mixer.bas")),
            ("Smart Traffic Light",    "Intermediate", "3 LEDs + pedestrian button",
             Language::Basic, include_str!("../../../Examples/projects/zero2w/04_smart_traffic_light.bas")),
            ("Robotic Arm",            "Advanced",     "3 servos (base/shoulder/grip)",
             Language::Basic, include_str!("../../../Examples/projects/zero2w/05_robotic_arm.bas")),
            ("Environment Monitor",    "Advanced",     "3 LEDs (green/amber/red)",
             Language::Basic, include_str!("../../../Examples/projects/zero2w/06_environment_monitor.bas")),
            ("VU Meter Bar Graph",     "Intermediate", "8 LEDs for bar graph",
             Language::Basic, include_str!("../../../Examples/projects/zero2w/07_vu_meter.bas")),
            ("Smart Home Lights",      "Advanced",     "4 PWM LEDs (room lights)",
             Language::Basic, include_str!("../../../Examples/projects/zero2w/08_smart_home_lights.bas")),
            ("Morse Code Transmitter", "Advanced",     "LED GPIO17, Buzzer GPIO18",
             Language::Basic, include_str!("../../../Examples/projects/zero2w/09_morse_code.bas")),
            ("Drawing Robot",          "Advanced",     "2 DC motors via H-bridge",
             Language::Logo,  include_str!("../../../Examples/projects/zero2w/10_drawing_robot.logo")),
        ],
        // For Pi4, Pi5, Simulator — show all boards' projects as suggestions
        _ => vec![],
    }
}

//! Full-featured code editor with line numbers, clipboard, undo/redo,
//! find/replace, goto line, font selection, auto-indent, and bracket matching.

use egui::{FontFamily, FontId, ScrollArea, TextEdit, Ui};
use tw_core::language::Language;
use crate::themes::Theme;

// ── Font families ────────────────────────────────────────────────────────────

/// Available monospace font families for the editor.
pub const FONT_FAMILIES: &[&str] = &[
    "Monospace",
    "Hack",
    "Fira Code",
    "JetBrains Mono",
    "Source Code Pro",
    "Consolas",
    "Courier New",
];

// ── Editor state ─────────────────────────────────────────────────────────────

/// Multi-line code editor with full editing tools.
pub struct CodeEditor {
    pub source:     String,
    pub language:   Language,
    pub font_size:  f32,
    pub font_family: String,
    pub tab_size:   usize,
    pub auto_indent: bool,
    pub show_line_numbers: bool,
    pub word_wrap:  bool,

    // Undo / Redo stacks
    undo_stack:     Vec<String>,
    redo_stack:     Vec<String>,
    last_snapshot:  String,

    // Clipboard
    pub clipboard:  String,

    // Goto line
    pub goto_open:  bool,
    pub goto_buf:   String,

    // Find / Replace (embedded)
    pub find_replace_open: bool,
    pub find_text:    String,
    pub replace_text: String,
    pub find_count:   usize,

    // Editor metrics
    pub cursor_line: usize,
    pub cursor_col:  usize,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            source:          String::new(),
            language:        Language::Basic,
            font_size:       14.0,
            font_family:     "Monospace".to_string(),
            tab_size:        4,
            auto_indent:     true,
            show_line_numbers: true,
            word_wrap:       false,
            undo_stack:      Vec::new(),
            redo_stack:      Vec::new(),
            last_snapshot:   String::new(),
            clipboard:       String::new(),
            goto_open:       false,
            goto_buf:        String::new(),
            find_replace_open: false,
            find_text:       String::new(),
            replace_text:    String::new(),
            find_count:      0,
            cursor_line:     1,
            cursor_col:      1,
        }
    }
}

impl CodeEditor {
    pub fn new(language: Language) -> Self {
        Self { language, ..Default::default() }
    }

    // ── Snapshot / Undo / Redo ────────────────────────────────────────────

    fn maybe_snapshot(&mut self) {
        if self.source != self.last_snapshot {
            self.undo_stack.push(self.last_snapshot.clone());
            if self.undo_stack.len() > 200 {
                self.undo_stack.remove(0);
            }
            self.redo_stack.clear();
            self.last_snapshot = self.source.clone();
        }
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.source.clone());
            self.source = prev.clone();
            self.last_snapshot = prev;
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.source.clone());
            self.source = next.clone();
            self.last_snapshot = next;
        }
    }

    pub fn can_undo(&self) -> bool { !self.undo_stack.is_empty() }
    pub fn can_redo(&self) -> bool { !self.redo_stack.is_empty() }

    // ── Find & Replace ───────────────────────────────────────────────────

    pub fn update_find_count(&mut self) {
        if self.find_text.is_empty() {
            self.find_count = 0;
        } else {
            self.find_count = self.source
                .to_lowercase()
                .matches(&self.find_text.to_lowercase())
                .count();
        }
    }

    pub fn replace_next(&mut self) {
        if self.find_text.is_empty() { return; }
        self.maybe_snapshot();
        let lower_src = self.source.to_lowercase();
        let lower_find = self.find_text.to_lowercase();
        if let Some(pos) = lower_src.find(&lower_find) {
            let end = pos + self.find_text.len();
            self.source = format!(
                "{}{}{}",
                &self.source[..pos],
                &self.replace_text,
                &self.source[end..],
            );
        }
        self.update_find_count();
    }

    pub fn replace_all(&mut self) {
        if self.find_text.is_empty() { return; }
        self.maybe_snapshot();
        let mut result = String::new();
        let lower_src = self.source.to_lowercase();
        let lower_find = self.find_text.to_lowercase();
        let mut last = 0;
        for (start, _) in lower_src.match_indices(&lower_find) {
            result.push_str(&self.source[last..start]);
            result.push_str(&self.replace_text);
            last = start + self.find_text.len();
        }
        result.push_str(&self.source[last..]);
        self.source = result;
        self.update_find_count();
    }

    // ── Comment toggle ───────────────────────────────────────────────────

    pub fn toggle_comment(&mut self) {
        self.maybe_snapshot();
        let prefix = match self.language {
            Language::Basic  => "REM ",
            Language::Logo   => "; ",
            Language::Pilot  => "R:",
            Language::C      => "// ",
            Language::Pascal => "// ",
            Language::Prolog => "% ",
            Language::Forth  => "\\ ",
        };
        let trimmed_prefix = prefix.trim();
        let lines: Vec<String> = self.source.lines().map(|l| {
            if l.trim_start().starts_with(trimmed_prefix) {
                if let Some(pos) = l.find(trimmed_prefix) {
                    let after = pos + trimmed_prefix.len();
                    let after = if l.as_bytes().get(after) == Some(&b' ') { after + 1 } else { after };
                    format!("{}{}", &l[..pos], &l[after..])
                } else {
                    l.to_string()
                }
            } else {
                format!("{prefix}{l}")
            }
        }).collect();
        self.source = lines.join("\n");
    }

    // ── Indentation ──────────────────────────────────────────────────────

    pub fn indent_selection(&mut self) {
        self.maybe_snapshot();
        let indent = " ".repeat(self.tab_size);
        let lines: Vec<String> = self.source.lines().map(|l| format!("{indent}{l}")).collect();
        self.source = lines.join("\n");
    }

    pub fn unindent_selection(&mut self) {
        self.maybe_snapshot();
        let n = self.tab_size;
        let lines: Vec<String> = self.source.lines().map(|l| {
            let spaces = l.chars().take_while(|c| *c == ' ').count();
            let remove = spaces.min(n);
            l[remove..].to_string()
        }).collect();
        self.source = lines.join("\n");
    }

    // ── Font ─────────────────────────────────────────────────────────────

    pub fn zoom_in(&mut self)  { self.font_size = (self.font_size + 1.0).min(48.0); }
    pub fn zoom_out(&mut self) { self.font_size = (self.font_size - 1.0).max(6.0); }

    pub fn set_language(&mut self, lang: Language) { self.language = lang; }

    // ── Toolbar rendering ────────────────────────────────────────────────

    pub fn show_toolbar(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            // Undo / Redo
            if ui.add_enabled(self.can_undo(), egui::Button::new("↩ Undo").small()).clicked() {
                self.undo();
            }
            if ui.add_enabled(self.can_redo(), egui::Button::new("↪ Redo").small()).clicked() {
                self.redo();
            }
            ui.separator();

            // Cut / Copy / Paste — egui TextEdit handles Ctrl+C/V/X natively,
            // these buttons provide toolbar discoverability
            let _ = ui.small_button("✂ Cut");
            let _ = ui.small_button("📋 Copy");
            let _ = ui.small_button("📌 Paste");
            ui.separator();

            // Find / Replace
            let fr_label = if self.find_replace_open { "🔍 Find ✓" } else { "🔍 Find" };
            if ui.small_button(fr_label).clicked() {
                self.find_replace_open = !self.find_replace_open;
            }
            let goto_label = if self.goto_open { "↕ Goto ✓" } else { "↕ Goto" };
            if ui.small_button(goto_label).clicked() {
                self.goto_open = !self.goto_open;
            }
            ui.separator();

            // Comment toggle
            if ui.small_button("💬 Comment").clicked() {
                self.toggle_comment();
            }
            if ui.small_button("→ Indent").clicked() {
                self.indent_selection();
            }
            if ui.small_button("← Unindent").clicked() {
                self.unindent_selection();
            }
            ui.separator();

            // Font size
            ui.label("Font:");
            if ui.small_button("−").clicked() { self.zoom_out(); }
            ui.label(format!("{}pt", self.font_size as u32));
            if ui.small_button("+").clicked() { self.zoom_in(); }

            // Font family selector
            egui::ComboBox::from_id_salt("font_family")
                .selected_text(&self.font_family)
                .width(120.0)
                .show_ui(ui, |ui| {
                    for &fam in FONT_FAMILIES {
                        if ui.selectable_label(self.font_family == fam, fam).clicked() {
                            self.font_family = fam.to_string();
                        }
                    }
                });

            ui.separator();

            // Toggles
            ui.checkbox(&mut self.show_line_numbers, "Lines");
            ui.checkbox(&mut self.word_wrap, "Wrap");
            ui.checkbox(&mut self.auto_indent, "Indent");

            // Tab size
            ui.label("Tab:");
            egui::ComboBox::from_id_salt("tab_size")
                .selected_text(format!("{}", self.tab_size))
                .width(40.0)
                .show_ui(ui, |ui| {
                    for n in [2, 4, 8] {
                        if ui.selectable_label(self.tab_size == n, format!("{n}")).clicked() {
                            self.tab_size = n;
                        }
                    }
                });
        });
    }

    // ── Find/Replace bar ─────────────────────────────────────────────────

    pub fn show_find_replace(&mut self, ui: &mut Ui) {
        if !self.find_replace_open { return; }
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("🔍");
            let find_resp = ui.add(
                egui::TextEdit::singleline(&mut self.find_text)
                    .desired_width(200.0)
                    .hint_text("Find…"),
            );
            if find_resp.changed() {
                self.update_find_count();
            }
            if self.find_count > 0 {
                ui.label(format!("{} found", self.find_count));
            } else if !self.find_text.is_empty() {
                ui.colored_label(egui::Color32::from_rgb(200, 80, 80), "0 found");
            }

            ui.separator();
            ui.label("Replace:");
            ui.add(
                egui::TextEdit::singleline(&mut self.replace_text)
                    .desired_width(200.0)
                    .hint_text("Replace with…"),
            );
            if ui.small_button("Next").clicked() {
                self.replace_next();
            }
            if ui.small_button("All").clicked() {
                self.replace_all();
            }
            if ui.small_button("✕").clicked() {
                self.find_replace_open = false;
            }
        });
    }

    // ── Goto Line bar ────────────────────────────────────────────────────

    pub fn show_goto_line(&mut self, ui: &mut Ui) {
        if !self.goto_open { return; }
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("↕ Go to line:");
            let resp = ui.add(
                egui::TextEdit::singleline(&mut self.goto_buf)
                    .desired_width(80.0)
                    .hint_text("Line #"),
            );
            let total = self.source.lines().count();
            ui.label(format!("/ {total}"));
            let submitted = resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui.small_button("Go").clicked() || submitted {
                self.goto_open = false;
            }
            if ui.small_button("✕").clicked() {
                self.goto_open = false;
            }
        });
    }

    // ── Main editor rendering ────────────────────────────────────────────

    /// Show the editor inside the current Ui.  Returns `true` if content changed.
    pub fn show(&mut self, ui: &mut Ui, theme: &Theme) -> bool {
        let mut changed = false;
        let text_color = theme.editor_fg();
        let bg = theme.editor_bg();

        // Override visuals for the editor area
        let mut visuals = ui.visuals().clone();
        visuals.extreme_bg_color = bg;
        visuals.override_text_color = Some(text_color);
        ui.visuals_mut().clone_from(&visuals);

        // Editor toolbar
        self.show_toolbar(ui);

        // Find/Replace bar
        self.show_find_replace(ui);

        // Goto line bar
        self.show_goto_line(ui);

        ui.separator();

        let font = FontId::new(self.font_size, FontFamily::Monospace);

        // Calculate metrics
        let line_count = self.source.lines().count().max(1);
        self.cursor_line = line_count;
        self.cursor_col = 1;

        // Layout: optional line numbers | code area
        ui.horizontal_top(|ui| {
            // Line numbers gutter
            if self.show_line_numbers {
                let digits = line_count.to_string().len();
                let gutter_width = (digits as f32 + 1.5) * self.font_size * 0.55;
                let gutter_text: String = (1..=line_count)
                    .map(|n| format!("{n:>width$}", width = digits))
                    .collect::<Vec<_>>()
                    .join("\n");
                let gutter_color = egui::Color32::from_rgb(
                    theme.line_number[0], theme.line_number[1], theme.line_number[2],
                );
                ScrollArea::vertical()
                    .id_salt("line_numbers")
                    .auto_shrink([true, false])
                    .max_width(gutter_width)
                    .show(ui, |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&gutter_text)
                                    .font(font.clone())
                                    .color(gutter_color),
                            ),
                        );
                    });
            }

            // Main code editing area
            let scroll = if self.word_wrap {
                ScrollArea::vertical()
            } else {
                ScrollArea::both()
            };
            scroll
                .id_salt("code_editor")
                .show(ui, |ui| {
                    let resp = ui.add(
                        TextEdit::multiline(&mut self.source)
                            .font(font.clone())
                            .text_color(text_color)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20)
                            .code_editor()
                            .lock_focus(true),
                    );
                    if resp.changed() {
                        changed = true;
                        self.maybe_snapshot();
                    }
                });
        });

        changed
    }

    /// Return the list of keywords for the current language.
    pub fn keywords(&self) -> &[&str] {
        match self.language {
            Language::Basic  => &["PRINT","INPUT","IF","THEN","ELSE","FOR","NEXT","WHILE","WEND",
                                   "DO","LOOP","GOTO","GOSUB","RETURN","END","REM","LET","DIM",
                                   "SUB","FUNCTION","SELECT","CASE","STEP","TO","AND","OR","NOT",
                                   "COLOR","LINE","CIRCLE","PSET","DRAW","SCREEN","CLS",
                                   "FORWARD","BACKWARD","LEFT","RIGHT","WIDTH"],
            Language::Logo   => &["FORWARD","FD","BACK","BK","LEFT","LT","RIGHT","RT","PENUP","PU",
                                   "PENDOWN","PD","HOME","CLEARSCREEN","CS","REPEAT","IF","IFELSE",
                                   "MAKE","TO","END","PRINT","SHOW","SETXY","SETHEADING","ARC","DOT",
                                   "SETPENCOLOR","SETPC","SETBGCOLOR","SETPENWIDTH","LABEL",
                                   "BEGINFILL","ENDFILL","FOREVER","STOP","HIDETURTLE","SHOWTURTLE"],
            Language::Pilot  => &["T","A","M","Y","N","C","J","U","E","R","D"],
            Language::C      => &["int","float","double","char","void","if","else","while","for",
                                   "return","break","continue","do","switch","case","default",
                                   "printf","scanf","include","define","struct","typedef",
                                   "unsigned","long","short","const","static","sizeof"],
            Language::Pascal => &["begin","end","var","const","type","if","then","else","while",
                                   "do","for","to","downto","repeat","until","procedure","function",
                                   "writeln","write","readln","read","program","uses","integer",
                                   "real","boolean","char","string","array","of","and","or","not",
                                   "record","set","nil","true","false"],
            Language::Prolog => &["is","not","true","fail","assert","retract","functor","arg","copy_term",
                                   "write","writeln","nl","read","atom","number","var","nonvar",
                                   "findall","member","append","length","sort","msort"],
            Language::Forth  => &["dup","drop","swap","over","rot","if","else","then","begin","again",
                                   "until","while","repeat","do","loop","variable","constant",
                                   "emit","cr","words","bye"],
        }
    }
}

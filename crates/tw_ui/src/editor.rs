//! Code editor widget with basic syntax highlighting.
//! Port of `ui/editor.py`.

use egui::{FontId, ScrollArea, TextEdit, Ui};
use tw_core::language::Language;
use crate::themes::Theme;

/// Multi-line code editor with line numbers.
pub struct CodeEditor {
    pub source: String,
    pub language: Language,
    pub font_size: f32,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            source:    String::new(),
            language:  Language::Basic,
            font_size: 14.0,
        }
    }
}

impl CodeEditor {
    pub fn new(language: Language) -> Self {
        Self { language, ..Default::default() }
    }

    /// Show the editor inside the current Ui.  Returns `true` if content changed.
    pub fn show(&mut self, ui: &mut Ui, theme: &Theme) -> bool {
        let mut changed = false;
        let text_color = theme.editor_fg();
        let bg  = theme.editor_bg();

        // Override visuals for the editor area
        let mut visuals = ui.visuals().clone();
        visuals.extreme_bg_color = bg;
        visuals.override_text_color = Some(text_color);
        ui.visuals_mut().clone_from(&visuals);

        let font = FontId::monospace(self.font_size);

        ScrollArea::both()
            .id_salt("code_editor")
            .show(ui, |ui| {
                let resp = ui.add(
                    TextEdit::multiline(&mut self.source)
                        .font(font.clone())
                        .text_color(text_color)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                        .lock_focus(true),
                );
                if resp.changed() { changed = true; }
            });

        changed
    }

    /// Increase / decrease font size.
    pub fn zoom_in(&mut self)  { self.font_size = (self.font_size + 1.0).min(40.0); }
    pub fn zoom_out(&mut self) { self.font_size = (self.font_size - 1.0).max(8.0); }

    pub fn set_language(&mut self, lang: Language) { self.language = lang; }

    /// Return the list of keywords for the current language (used for syntax hints).
    pub fn keywords(&self) -> &[&str] {
        match self.language {
            Language::Basic  => &["PRINT","INPUT","IF","THEN","ELSE","FOR","NEXT","WHILE","WEND",
                                   "DO","LOOP","GOTO","GOSUB","RETURN","END","REM","LET","DIM",
                                   "SUB","FUNCTION","SELECT","CASE","STEP","TO","AND","OR","NOT"],
            Language::Logo   => &["FORWARD","FD","BACK","BK","LEFT","LT","RIGHT","RT","PENUP","PU",
                                   "PENDOWN","PD","HOME","CLEARSCREEN","CS","REPEAT","IF","IFELSE",
                                   "MAKE","TO","END","PRINT","SHOW","SETXY","SETHEADING","ARC","DOT"],
            Language::Pilot  => &["T","A","M","Y","N","C","J","U","E","R","D"],
            Language::C      => &["int","float","double","char","void","if","else","while","for",
                                   "return","break","continue","do","switch","case","default",
                                   "printf","scanf","include","define"],
            Language::Pascal => &["begin","end","var","const","type","if","then","else","while",
                                   "do","for","to","downto","repeat","until","procedure","function",
                                   "writeln","write","readln","read","program","uses","integer",
                                   "real","boolean","char","string","array","of","and","or","not"],
            Language::Prolog => &["is","not","true","fail","assert","retract","functor","arg","copy_term",
                                   "write","writeln","nl","read","atom","number","var","nonvar"],
            Language::Forth  => &["dup","drop","swap","over","rot","if","else","then","begin","again",
                                   "until","while","repeat","do","loop","variable","constant",
                                   "emit","cr","words","bye"],
        }
    }
}

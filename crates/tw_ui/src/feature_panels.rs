//! Feature panels — lessons, examples browser, theme editor, about.
//! Port of `ui/feature_panels.py`.

use egui::{ScrollArea, Ui};
use tw_core::language::Language;
use crate::themes::ThemeManager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Panel {
    Lessons,
    Examples,
    Themes,
    About,
}

pub struct FeaturePanels {
    pub active: Panel,
    /// Selected example text (if any) ready to load into editor.
    pub pending_load: Option<(Language, String)>,
    example_filter: String,
}

impl Default for FeaturePanels {
    fn default() -> Self {
        Self {
            active: Panel::Examples,
            pending_load: None,
            example_filter: String::new(),
        }
    }
}

impl FeaturePanels {
    pub fn new() -> Self { Self::default() }

    /// Render the panel.  Returns an optional (language, source) to load.
    pub fn show(&mut self, ui: &mut Ui, themes: &mut ThemeManager) {
        self.pending_load = None;

        ui.horizontal(|ui| {
            for (label, panel) in [
                ("📚 Lessons", Panel::Lessons),
                ("📁 Examples", Panel::Examples),
                ("🎨 Themes", Panel::Themes),
                ("ℹ About", Panel::About),
            ] {
                let selected = self.active == panel;
                if ui.selectable_label(selected, label).clicked() {
                    self.active = panel;
                }
            }
        });
        ui.separator();

        match &self.active {
            Panel::Lessons  => self.show_lessons(ui),
            Panel::Examples => self.show_examples(ui),
            Panel::Themes   => self.show_themes(ui, themes),
            Panel::About    => self.show_about(ui),
        }
    }

    // ── Lessons ────────────────────────────────────────────────────────────

    fn show_lessons(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().id_salt("lessons").show(ui, |ui| {
            ui.heading("📚 Lessons");
            ui.label("Choose a language and follow the interactive lessons.");
            ui.separator();

            for lang in Language::all() {
                ui.collapsing(lang.friendly_name(), |ui| {
                    lessons_for(lang, ui);
                    if ui.button("▶ Load starter program").clicked() {
                        self.pending_load = Some((*lang, lang.sample_program().to_string()));
                    }
                });
            }
        });
    }

    // ── Examples ───────────────────────────────────────────────────────────

    fn show_examples(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut self.example_filter);
        });
        ScrollArea::vertical().id_salt("examples").show(ui, |ui| {
            for lang in Language::all() {
                for (title, code) in builtin_examples(*lang) {
                    let filter = self.example_filter.to_lowercase();
                    if !filter.is_empty()
                        && !title.to_lowercase().contains(&filter)
                        && !lang.friendly_name().to_lowercase().contains(&filter)
                    {
                        continue;
                    }
                    ui.horizontal(|ui| {
                        ui.label(format!("[{}]", lang.friendly_name()));
                        if ui.button(title).clicked() {
                            self.pending_load = Some((*lang, code.to_string()));
                        }
                    });
                }
            }
        });
    }

    // ── Themes ─────────────────────────────────────────────────────────────

    fn show_themes(&mut self, ui: &mut Ui, themes: &mut ThemeManager) {
        ui.heading("🎨 Themes");
        let names: Vec<String> = themes.theme_names().into_iter().map(|s| s.to_string()).collect();
        for name in &names {
            let selected = themes.current == *name;
            if ui.selectable_label(selected, name).clicked() {
                themes.set_theme(name);
            }
        }
    }

    // ── About ──────────────────────────────────────────────────────────────

    fn show_about(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().id_salt("about").show(ui, |ui| {
            ui.heading("⏱ Time Warp Rusted");
            ui.label("Version 2.0 — Rust Edition");
            ui.separator();
            ui.label("An educational multi-language IDE supporting:");
            for lang in Language::all() {
                ui.label(format!("  • {}", lang.friendly_name()));
            }
            ui.separator();
            ui.label("Built with:");
            ui.label("  • Rust");
            ui.label("  • eframe / egui");
            ui.label("  • glow (OpenGL)");
            ui.separator();
            ui.label("License: MIT");
        });
    }
}

// ── helpers ────────────────────────────────────────────────────────────────────

fn lessons_for(lang: &Language, ui: &mut Ui) {
    let lessons: &[&str] = match lang {
        Language::Basic  => &["1. Hello World","2. Variables","3. Input","4. Loops","5. Graphics"],
        Language::Logo   => &["1. Moving the turtle","2. Drawing shapes","3. Procedures","4. Recursion"],
        Language::Pilot  => &["1. T: command","2. A: command","3. M: matching","4. Branching"],
        Language::C      => &["1. Hello World","2. Variables","3. printf/scanf","4. Loops","5. Functions"],
        Language::Pascal => &["1. Hello World","2. Variables","3. writeln","4. Loops","5. Procedures"],
        Language::Prolog => &["1. Facts","2. Rules","3. Queries","4. Unification","5. Backtracking"],
        Language::Forth  => &["1. Stack basics","2. Arithmetic","3. Words","4. Loops","5. Variables"],
    };
    for l in lessons { ui.label(*l); }
}

fn builtin_examples(lang: Language) -> Vec<(&'static str, &'static str)> {
    match lang {
        Language::Basic => vec![
            ("Hello World", "10 PRINT \"Hello, World!\"\n20 END\n"),
            ("Count to 10", "10 FOR I = 1 TO 10\n20 PRINT I\n30 NEXT I\n40 END\n"),
            ("Fibonacci", r#"10 A = 0 : B = 1
20 FOR I = 1 TO 10
30   PRINT A
40   C = A + B
50   A = B
60   B = C
70 NEXT I
80 END
"#),
            ("Square spiral", r#"10 FOR I = 1 TO 40
20   FORWARD I * 2
30   RIGHT 89
40 NEXT I
50 END
"#),
        ],
        Language::Logo => vec![
            ("Square", "REPEAT 4 [FORWARD 100 RIGHT 90]\n"),
            ("Star", "REPEAT 5 [FORWARD 150 RIGHT 144]\n"),
            ("Spiral", r#"REPEAT 100 [FORWARD REPCOUNT * 2 RIGHT 91]
"#),
            ("Tree (recursive)", r#"TO TREE :SIZE
  IF :SIZE < 5 [STOP]
  FORWARD :SIZE
  LEFT 30  TREE :SIZE * 0.7
  RIGHT 60 TREE :SIZE * 0.7
  LEFT 30
  BACK :SIZE
END
TREE 80
"#),
        ],
        Language::Pilot => vec![
            ("Greeting", "T:What is your name?\nA:NAME\nT:Hello, #NAME!\nE:\n"),
            ("Addition quiz", r#"T:Enter two numbers to add.
A:X
A:Y
C:SUM = X + Y
T:#X + #Y = #SUM
E:
"#),
        ],
        Language::C => vec![
            ("Hello World", "#include <stdio.h>\nint main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}\n"),
            ("For loop", "#include <stdio.h>\nint main() {\n    int i;\n    for (i = 1; i <= 10; i++) {\n        printf(\"%d\\n\", i);\n    }\n    return 0;\n}\n"),
        ],
        Language::Pascal => vec![
            ("Hello World", "program Hello;\nbegin\n  writeln('Hello, World!');\nend.\n"),
            ("Sum 1..10", "program Sum;\nvar i, s : integer;\nbegin\n  s := 0;\n  for i := 1 to 10 do\n    s := s + i;\n  writeln(s);\nend.\n"),
        ],
        Language::Prolog => vec![
            ("Family", "parent(tom, bob).\nparent(bob, ann).\nancestor(X,Y) :- parent(X,Y).\nancestor(X,Y) :- parent(X,Z), ancestor(Z,Y).\n?- ancestor(tom, ann).\n"),
        ],
        Language::Forth => vec![
            ("Square", ": SQ DUP * ;\n5 SQ . CR\n"),
            ("Fibonacci", r#": FIB
  DUP 2 < IF EXIT THEN
  DUP 1 - RECURSE
  SWAP 2 - RECURSE +
;
10 FIB . CR
"#),
        ],
    }
}

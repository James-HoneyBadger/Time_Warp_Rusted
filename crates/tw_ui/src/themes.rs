//! Theme system — port of `ui/themes.py`.
//!
//! Eight built-in themes, selectable at runtime.

use egui::Color32;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete colour theme for the IDE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name:        String,
    // Window / panel
    pub background:  [u8; 3],
    pub foreground:  [u8; 3],
    pub panel_bg:    [u8; 3],
    pub border:      [u8; 3],
    // Editor
    pub editor_bg:   [u8; 3],
    pub editor_fg:   [u8; 3],
    pub line_number: [u8; 3],
    pub cursor:      [u8; 3],
    pub selection:   [u8; 4],   // RGBA
    // Syntax
    pub keyword:     [u8; 3],
    pub comment:     [u8; 3],
    pub string:      [u8; 3],
    pub number:      [u8; 3],
    pub operator:    [u8; 3],
    pub function:    [u8; 3],
    pub builtin:     [u8; 3],
    // Output / canvas
    pub output_bg:   [u8; 3],
    pub output_fg:   [u8; 3],
    pub canvas_bg:   [u8; 3],
    // UI controls
    pub button_bg:   [u8; 3],
    pub button_fg:   [u8; 3],
    pub accent:      [u8; 3],
    pub error:       [u8; 3],
    pub warning:     [u8; 3],
    pub success:     [u8; 3],
}

impl Theme {
    pub fn bg(&self)     -> Color32 { rgb(self.background) }
    pub fn fg(&self)     -> Color32 { rgb(self.foreground) }
    pub fn editor_bg(&self) -> Color32 { rgb(self.editor_bg) }
    pub fn editor_fg(&self) -> Color32 { rgb(self.editor_fg) }
    pub fn keyword_color(&self)  -> Color32 { rgb(self.keyword)  }
    pub fn comment_color(&self)  -> Color32 { rgb(self.comment)  }
    pub fn string_color(&self)   -> Color32 { rgb(self.string)   }
    pub fn number_color(&self)   -> Color32 { rgb(self.number)   }
    pub fn canvas_bg(&self)  -> Color32 { rgb(self.canvas_bg) }
    pub fn output_bg(&self)  -> Color32 { rgb(self.output_bg) }
    pub fn output_fg(&self)  -> Color32 { rgb(self.output_fg) }
    pub fn accent(&self)     -> Color32 { rgb(self.accent) }
    pub fn error(&self)      -> Color32 { rgb(self.error) }
    pub fn success(&self)    -> Color32 { rgb(self.success) }
    pub fn warning(&self)    -> Color32 { rgb(self.warning) }
    pub fn border_color(&self) -> Color32 { rgb(self.border) }
    pub fn panel_bg(&self)   -> Color32 { rgb(self.panel_bg) }
    pub fn button_bg(&self)  -> Color32 { rgb(self.button_bg) }
    pub fn button_fg(&self)  -> Color32 { rgb(self.button_fg) }
    pub fn selection_color(&self) -> Color32 {
        Color32::from_rgba_premultiplied(
            self.selection[0], self.selection[1], self.selection[2], self.selection[3]
        )
    }
}

fn rgb(c: [u8; 3]) -> Color32 { Color32::from_rgb(c[0], c[1], c[2]) }

// ── built-in themes ────────────────────────────────────────────────────────────

pub fn dracula() -> Theme { Theme {
    name: "Dracula".into(),
    background:  [40,  42,  54],
    foreground:  [248,248,242],
    panel_bg:    [33,  34,  44],
    border:      [68,  71,  90],
    editor_bg:   [40,  42,  54],
    editor_fg:   [248,248,242],
    line_number: [100,100,120],
    cursor:      [248,248,242],
    selection:   [68, 71, 90, 150],
    keyword:     [255,121,198],
    comment:     [98, 114,164],
    string:      [241,250,140],
    number:      [189,147,249],
    operator:    [255,121,198],
    function:    [80, 250,123],
    builtin:     [139,233,253],
    output_bg:   [22,  22,  30],
    output_fg:   [248,248,242],
    canvas_bg:   [22,  22,  30],
    button_bg:   [68,  71,  90],
    button_fg:   [248,248,242],
    accent:      [189,147,249],
    error:       [255, 85, 85],
    warning:     [255,184, 99],
    success:     [80, 250,123],
}}

pub fn monokai() -> Theme { Theme {
    name: "Monokai".into(),
    background:  [39, 40, 34],
    foreground:  [248,248,242],
    panel_bg:    [30, 31, 26],
    border:      [62, 61, 50],
    editor_bg:   [39, 40, 34],
    editor_fg:   [248,248,242],
    line_number: [117,113, 94],
    cursor:      [248,248,242],
    selection:   [73,72,62,180],
    keyword:     [249, 38,114],
    comment:     [117,113, 94],
    string:      [230,219,116],
    number:      [174,129,255],
    operator:    [249, 38,114],
    function:    [166,226, 46],
    builtin:     [102,217,239],
    output_bg:   [20, 20, 16],
    output_fg:   [248,248,242],
    canvas_bg:   [20, 20, 16],
    button_bg:   [62, 61, 50],
    button_fg:   [248,248,242],
    accent:      [174,129,255],
    error:       [249, 38,114],
    warning:     [230,219,116],
    success:     [166,226, 46],
}}

pub fn vscode_dark() -> Theme { Theme {
    name: "VS Code Dark".into(),
    background:  [30, 30, 30],
    foreground:  [212,212,212],
    panel_bg:    [24, 24, 24],
    border:      [60, 60, 60],
    editor_bg:   [30, 30, 30],
    editor_fg:   [212,212,212],
    line_number: [133,133,133],
    cursor:      [212,212,212],
    selection:   [38,79,120,200],
    keyword:     [86, 156,214],
    comment:     [106,153, 85],
    string:      [206,145,120],
    number:      [181,206,168],
    operator:    [212,212,212],
    function:    [220,220,170],
    builtin:     [156,220,254],
    output_bg:   [12, 12, 12],
    output_fg:   [204,204,204],
    canvas_bg:   [18, 18, 18],
    button_bg:   [45, 45, 45],
    button_fg:   [212,212,212],
    accent:      [0, 120,215],
    error:       [244, 71, 71],
    warning:     [255,184, 99],
    success:     [73, 193,109],
}}

pub fn solarized_dark() -> Theme { Theme {
    name: "Solarized Dark".into(),
    background:  [0,  43, 54],
    foreground:  [131,148,150],
    panel_bg:    [0,  36, 46],
    border:      [7,  54, 66],
    editor_bg:   [0,  43, 54],
    editor_fg:   [131,148,150],
    line_number: [88, 110,117],
    cursor:      [253,246,227],
    selection:   [7,54,66,200],
    keyword:     [133, 82,143],
    comment:     [88, 110,117],
    string:      [42, 161,152],
    number:      [181,137,  0],
    operator:    [211, 54,130],
    function:    [38, 139,210],
    builtin:     [108,113,196],
    output_bg:   [0,  29, 38],
    output_fg:   [131,148,150],
    canvas_bg:   [0,  29, 38],
    button_bg:   [7,  54, 66],
    button_fg:   [131,148,150],
    accent:      [38, 139,210],
    error:       [220, 50, 47],
    warning:     [181,137,  0],
    success:     [133,153,  0],
}}

pub fn ocean_blue() -> Theme { Theme {
    name: "Ocean Blue".into(),
    background:  [13, 27, 42],
    foreground:  [200,220,240],
    panel_bg:    [10, 20, 35],
    border:      [30, 60, 90],
    editor_bg:   [13, 27, 42],
    editor_fg:   [200,220,240],
    line_number: [80, 100,130],
    cursor:      [200,220,240],
    selection:   [30,80,140,180],
    keyword:     [100,180,255],
    comment:     [80, 120,160],
    string:      [120,220,180],
    number:      [255,200,100],
    operator:    [200,150,255],
    function:    [80, 200,220],
    builtin:     [150,255,200],
    output_bg:   [5,  12, 22],
    output_fg:   [180,210,240],
    canvas_bg:   [5,  12, 22],
    button_bg:   [20, 50, 80],
    button_fg:   [200,220,240],
    accent:      [0, 150,255],
    error:       [255, 80, 80],
    warning:     [255,200, 60],
    success:     [80, 220,120],
}}

pub fn spring() -> Theme { Theme {
    name: "Spring".into(),
    background:  [245,250,240],
    foreground:  [40,  60, 40],
    panel_bg:    [235,245,230],
    border:      [180,210,170],
    editor_bg:   [250,255,247],
    editor_fg:   [40,  60, 40],
    line_number: [150,180,140],
    cursor:      [40,  80, 40],
    selection:   [150,220,150,160],
    keyword:     [60, 140, 80],
    comment:     [140,180,130],
    string:      [200,100, 50],
    number:      [120, 80,180],
    operator:    [100,140, 60],
    function:    [0,  120,160],
    builtin:     [180, 80,120],
    output_bg:   [240,248,235],
    output_fg:   [40,  60, 40],
    canvas_bg:   [255,255,255],
    button_bg:   [180,220,160],
    button_fg:   [40,  60, 40],
    accent:      [60, 180, 80],
    error:       [200, 40, 40],
    warning:     [180,140, 20],
    success:     [40, 160, 60],
}}

pub fn sunset() -> Theme { Theme {
    name: "Sunset".into(),
    background:  [25, 15, 10],
    foreground:  [255,220,180],
    panel_bg:    [20, 12, 8],
    border:      [80, 40, 20],
    editor_bg:   [30, 18, 10],
    editor_fg:   [255,220,180],
    line_number: [120, 80, 50],
    cursor:      [255,220,180],
    selection:   [100,50,20,180],
    keyword:     [255,120, 80],
    comment:     [120, 80, 50],
    string:      [255,200, 80],
    number:      [200,120,255],
    operator:    [255,160, 60],
    function:    [255,100,100],
    builtin:     [200,200,100],
    output_bg:   [10, 6, 4],
    output_fg:   [240,210,170],
    canvas_bg:   [10, 6, 4],
    button_bg:   [60, 30, 15],
    button_fg:   [255,220,180],
    accent:      [255,100, 50],
    error:       [255, 60, 60],
    warning:     [255,180, 40],
    success:     [120,220, 80],
}}

pub fn candy() -> Theme { Theme {
    name: "Candy".into(),
    background:  [255,240,250],
    foreground:  [80,  20, 80],
    panel_bg:    [250,235,248],
    border:      [230,180,230],
    editor_bg:   [255,245,255],
    editor_fg:   [60,  20, 60],
    line_number: [200,150,200],
    cursor:      [180, 0,180],
    selection:   [255,180,255,160],
    keyword:     [220, 0,160],
    comment:     [180,130,180],
    string:      [200, 80, 20],
    number:      [100, 20,200],
    operator:    [200, 0,120],
    function:    [0,  140,200],
    builtin:     [200, 80,200],
    output_bg:   [250,240,255],
    output_fg:   [80,  20, 80],
    canvas_bg:   [255,255,255],
    button_bg:   [255,200,230],
    button_fg:   [80,  20, 80],
    accent:      [220, 0,160],
    error:       [220, 20, 20],
    warning:     [200,130, 0],
    success:     [0,  160, 80],
}}

// ── theme manager ─────────────────────────────────────────────────────────────

pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    pub current: String,
}

impl Default for ThemeManager {
    fn default() -> Self {
        let mut m = Self { themes: HashMap::new(), current: "Dracula".to_string() };
        for t in [dracula(), monokai(), vscode_dark(), solarized_dark(),
                  ocean_blue(), spring(), sunset(), candy()]
        {
            m.themes.insert(t.name.clone(), t);
        }
        m
    }
}

impl ThemeManager {
    pub fn new() -> Self { Self::default() }

    pub fn current(&self) -> &Theme {
        self.themes.get(&self.current)
            .unwrap_or_else(|| self.themes.values().next().unwrap())
    }

    pub fn set_theme(&mut self, name: &str) {
        if self.themes.contains_key(name) {
            self.current = name.to_string();
        }
    }

    pub fn theme_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.themes.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }

    pub fn get(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }
}

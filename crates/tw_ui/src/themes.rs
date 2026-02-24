//! Theme system for the Time Warp Studio IDE.
//!
//! 20 built-in themes organised into categories: Retro, Dark, and Light.

use egui::Color32;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme category for grouping in the selector UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThemeCategory {
    Retro,
    Dark,
    Light,
}

impl ThemeCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Retro => "🕹 Retro",
            Self::Dark  => "🌙 Dark",
            Self::Light => "☀ Light",
        }
    }
    pub fn all() -> &'static [ThemeCategory] {
        &[Self::Retro, Self::Dark, Self::Light]
    }
}

/// A complete colour theme for the IDE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name:        String,
    pub category:    ThemeCategory,
    pub description: String,
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

// ─── RETRO themes ──────────────────────────────────────────────────────────────

pub fn amber_terminal() -> Theme { Theme {
    name: "Amber Terminal".into(),
    category: ThemeCategory::Retro,
    description: "Classic amber phosphor CRT monitor".into(),
    background:  [20, 12, 0],
    foreground:  [255,176, 0],
    panel_bg:    [15, 8, 0],
    border:      [80, 55, 0],
    editor_bg:   [20, 12, 0],
    editor_fg:   [255,176, 0],
    line_number: [120, 82, 0],
    cursor:      [255,200, 50],
    selection:   [80, 55, 0, 180],
    keyword:     [255,210, 80],
    comment:     [140, 96, 0],
    string:      [255,200, 50],
    number:      [255,220,120],
    operator:    [255,176, 0],
    function:    [255,230,140],
    builtin:     [255,190, 30],
    output_bg:   [10, 6, 0],
    output_fg:   [255,176, 0],
    canvas_bg:   [10, 6, 0],
    button_bg:   [50, 34, 0],
    button_fg:   [255,176, 0],
    accent:      [255,200, 50],
    error:       [255, 80, 0],
    warning:     [255,200, 50],
    success:     [200,180, 0],
}}

pub fn green_screen() -> Theme { Theme {
    name: "Green Screen".into(),
    category: ThemeCategory::Retro,
    description: "Classic green phosphor terminal".into(),
    background:  [0, 15, 0],
    foreground:  [0, 255, 0],
    panel_bg:    [0, 10, 0],
    border:      [0, 80, 0],
    editor_bg:   [0, 15, 0],
    editor_fg:   [0, 255, 0],
    line_number: [0, 100, 0],
    cursor:      [50, 255, 50],
    selection:   [0, 80, 0, 180],
    keyword:     [80, 255, 80],
    comment:     [0, 130, 0],
    string:      [50, 255, 50],
    number:      [120, 255, 120],
    operator:    [0, 220, 0],
    function:    [140, 255, 140],
    builtin:     [0, 230, 0],
    output_bg:   [0, 8, 0],
    output_fg:   [0, 255, 0],
    canvas_bg:   [0, 8, 0],
    button_bg:   [0, 45, 0],
    button_fg:   [0, 255, 0],
    accent:      [50, 255, 50],
    error:       [200, 80, 0],
    warning:     [180, 255, 0],
    success:     [0, 255, 0],
}}

pub fn c64() -> Theme { Theme {
    name: "Commodore 64".into(),
    category: ThemeCategory::Retro,
    description: "Commodore 64 blue and light blue".into(),
    background:  [64, 50, 133],
    foreground:  [134, 122, 222],
    panel_bg:    [50, 38, 110],
    border:      [88, 74, 160],
    editor_bg:   [64, 50, 133],
    editor_fg:   [134, 122, 222],
    line_number: [100, 88, 170],
    cursor:      [167, 155, 255],
    selection:   [88, 74, 160, 180],
    keyword:     [167, 155, 255],
    comment:     [100, 88, 170],
    string:      [110, 205, 120],
    number:      [255, 255, 134],
    operator:    [134, 122, 222],
    function:    [134, 210, 255],
    builtin:     [200, 170, 255],
    output_bg:   [40, 30, 100],
    output_fg:   [134, 122, 222],
    canvas_bg:   [40, 30, 100],
    button_bg:   [88, 74, 160],
    button_fg:   [167, 155, 255],
    accent:      [134, 122, 222],
    error:       [255, 100, 100],
    warning:     [255, 255, 134],
    success:     [110, 205, 120],
}}

pub fn borland_turbo() -> Theme { Theme {
    name: "Borland Turbo".into(),
    category: ThemeCategory::Retro,
    description: "Borland Turbo Pascal / Turbo C IDE".into(),
    background:  [0, 0, 168],
    foreground:  [255, 255, 85],
    panel_bg:    [0, 0, 128],
    border:      [0, 168, 168],
    editor_bg:   [0, 0, 168],
    editor_fg:   [255, 255, 85],
    line_number: [0, 168, 168],
    cursor:      [255, 255, 255],
    selection:   [0, 168, 0, 180],
    keyword:     [255, 255, 255],
    comment:     [0, 168, 168],
    string:      [255, 85, 255],
    number:      [85, 255, 255],
    operator:    [255, 255, 85],
    function:    [85, 255, 85],
    builtin:     [255, 85, 85],
    output_bg:   [0, 0, 100],
    output_fg:   [255, 255, 85],
    canvas_bg:   [0, 0, 100],
    button_bg:   [0, 168, 168],
    button_fg:   [0, 0, 0],
    accent:      [0, 168, 168],
    error:       [255, 85, 85],
    warning:     [255, 255, 85],
    success:     [85, 255, 85],
}}

pub fn cga() -> Theme { Theme {
    name: "CGA".into(),
    category: ThemeCategory::Retro,
    description: "IBM CGA 4-color palette (Palette 1)".into(),
    background:  [0, 0, 0],
    foreground:  [255, 255, 255],
    panel_bg:    [10, 10, 10],
    border:      [85, 85, 85],
    editor_bg:   [0, 0, 0],
    editor_fg:   [255, 255, 255],
    line_number: [85, 85, 85],
    cursor:      [255, 255, 255],
    selection:   [85, 255, 255, 160],
    keyword:     [255, 85, 255],
    comment:     [85, 85, 85],
    string:      [85, 255, 255],
    number:      [255, 85, 255],
    operator:    [255, 255, 255],
    function:    [85, 255, 255],
    builtin:     [255, 85, 255],
    output_bg:   [0, 0, 0],
    output_fg:   [255, 255, 255],
    canvas_bg:   [0, 0, 0],
    button_bg:   [85, 85, 85],
    button_fg:   [255, 255, 255],
    accent:      [85, 255, 255],
    error:       [255, 85, 85],
    warning:     [255, 255, 85],
    success:     [85, 255, 85],
}}

pub fn apple_ii() -> Theme { Theme {
    name: "Apple ][".into(),
    category: ThemeCategory::Retro,
    description: "Apple II hi-res green and black".into(),
    background:  [0, 0, 0],
    foreground:  [0, 230, 0],
    panel_bg:    [5, 5, 5],
    border:      [0, 100, 0],
    editor_bg:   [0, 0, 0],
    editor_fg:   [0, 230, 0],
    line_number: [0, 90, 0],
    cursor:      [0, 255, 0],
    selection:   [0, 80, 0, 160],
    keyword:     [100, 255, 100],
    comment:     [0, 120, 0],
    string:      [150, 100, 255],
    number:      [255, 100, 0],
    operator:    [0, 200, 0],
    function:    [255, 255, 255],
    builtin:     [0, 200, 255],
    output_bg:   [0, 0, 0],
    output_fg:   [0, 230, 0],
    canvas_bg:   [0, 0, 0],
    button_bg:   [0, 60, 0],
    button_fg:   [0, 230, 0],
    accent:      [0, 200, 0],
    error:       [255, 50, 50],
    warning:     [255, 200, 0],
    success:     [0, 255, 0],
}}

// ─── DARK themes ───────────────────────────────────────────────────────────────

pub fn dracula() -> Theme { Theme {
    name: "Dracula".into(),
    category: ThemeCategory::Dark,
    description: "Popular dark theme with vibrant colors".into(),
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
    category: ThemeCategory::Dark,
    description: "Iconic dark theme from Sublime Text".into(),
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
    category: ThemeCategory::Dark,
    description: "Default dark theme from VS Code".into(),
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
    category: ThemeCategory::Dark,
    description: "Ethan Schoonover's precision color scheme".into(),
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
    category: ThemeCategory::Dark,
    description: "Deep ocean-inspired dark palette".into(),
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

pub fn sunset() -> Theme { Theme {
    name: "Sunset".into(),
    category: ThemeCategory::Dark,
    description: "Warm sunset colors on a dark canvas".into(),
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

pub fn nord() -> Theme { Theme {
    name: "Nord".into(),
    category: ThemeCategory::Dark,
    description: "Arctic, north-bluish clean palette".into(),
    background:  [46, 52, 64],
    foreground:  [216,222,233],
    panel_bg:    [36, 40, 50],
    border:      [59, 66, 82],
    editor_bg:   [46, 52, 64],
    editor_fg:   [216,222,233],
    line_number: [76, 86,106],
    cursor:      [216,222,233],
    selection:   [67, 76, 94, 200],
    keyword:     [129,161,193],
    comment:     [76, 86,106],
    string:      [163,190,140],
    number:      [180,142,173],
    operator:    [129,161,193],
    function:    [136,192,208],
    builtin:     [143,188,187],
    output_bg:   [36, 40, 50],
    output_fg:   [216,222,233],
    canvas_bg:   [36, 40, 50],
    button_bg:   [59, 66, 82],
    button_fg:   [216,222,233],
    accent:      [136,192,208],
    error:       [191, 97,106],
    warning:     [235,203,139],
    success:     [163,190,140],
}}

pub fn gruvbox_dark() -> Theme { Theme {
    name: "Gruvbox Dark".into(),
    category: ThemeCategory::Dark,
    description: "Retro groove color scheme".into(),
    background:  [40, 40, 40],
    foreground:  [235,219,178],
    panel_bg:    [29, 32, 33],
    border:      [60, 56, 54],
    editor_bg:   [40, 40, 40],
    editor_fg:   [235,219,178],
    line_number: [124,111, 100],
    cursor:      [235,219,178],
    selection:   [80, 73, 69, 200],
    keyword:     [251, 73, 52],
    comment:     [146,131,116],
    string:      [184,187, 38],
    number:      [211,134,155],
    operator:    [254,128, 25],
    function:    [250,189, 47],
    builtin:     [131,165,152],
    output_bg:   [29, 32, 33],
    output_fg:   [235,219,178],
    canvas_bg:   [29, 32, 33],
    button_bg:   [60, 56, 54],
    button_fg:   [235,219,178],
    accent:      [254,128, 25],
    error:       [251, 73, 52],
    warning:     [250,189, 47],
    success:     [184,187, 38],
}}

pub fn tokyo_night() -> Theme { Theme {
    name: "Tokyo Night".into(),
    category: ThemeCategory::Dark,
    description: "Clean dark theme inspired by Tokyo nights".into(),
    background:  [26, 27, 38],
    foreground:  [169,177,214],
    panel_bg:    [22, 22, 30],
    border:      [41, 46, 66],
    editor_bg:   [26, 27, 38],
    editor_fg:   [169,177,214],
    line_number: [60, 65, 90],
    cursor:      [169,177,214],
    selection:   [41, 46, 66, 200],
    keyword:     [187,154,247],
    comment:     [86, 95,137],
    string:      [158,206,106],
    number:      [255,158,100],
    operator:    [137,221,255],
    function:    [125,207,255],
    builtin:     [42, 195,222],
    output_bg:   [22, 22, 30],
    output_fg:   [169,177,214],
    canvas_bg:   [22, 22, 30],
    button_bg:   [41, 46, 66],
    button_fg:   [169,177,214],
    accent:      [125,207,255],
    error:       [247,118,142],
    warning:     [224,175,104],
    success:     [158,206,106],
}}

pub fn one_dark() -> Theme { Theme {
    name: "One Dark".into(),
    category: ThemeCategory::Dark,
    description: "Atom's iconic dark color scheme".into(),
    background:  [40, 44, 52],
    foreground:  [171,178,191],
    panel_bg:    [33, 37, 43],
    border:      [50, 56, 66],
    editor_bg:   [40, 44, 52],
    editor_fg:   [171,178,191],
    line_number: [76, 82, 99],
    cursor:      [171,178,191],
    selection:   [62, 68, 81, 200],
    keyword:     [198,120,221],
    comment:     [92, 99,112],
    string:      [152,195,121],
    number:      [209,154,102],
    operator:    [86, 182,194],
    function:    [97, 175,239],
    builtin:     [224,108,117],
    output_bg:   [33, 37, 43],
    output_fg:   [171,178,191],
    canvas_bg:   [33, 37, 43],
    button_bg:   [50, 56, 66],
    button_fg:   [171,178,191],
    accent:      [97, 175,239],
    error:       [224,108,117],
    warning:     [229,192,123],
    success:     [152,195,121],
}}

// ─── LIGHT themes ──────────────────────────────────────────────────────────────

pub fn spring() -> Theme { Theme {
    name: "Spring".into(),
    category: ThemeCategory::Light,
    description: "Fresh green spring garden palette".into(),
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

pub fn candy() -> Theme { Theme {
    name: "Candy".into(),
    category: ThemeCategory::Light,
    description: "Sweet pink and pastel tones".into(),
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

pub fn solarized_light() -> Theme { Theme {
    name: "Solarized Light".into(),
    category: ThemeCategory::Light,
    description: "Solarized warm light variant".into(),
    background:  [253,246,227],
    foreground:  [101,123,131],
    panel_bg:    [238,232,213],
    border:      [147,161,161],
    editor_bg:   [253,246,227],
    editor_fg:   [101,123,131],
    line_number: [147,161,161],
    cursor:      [0, 43, 54],
    selection:   [238,232,213,200],
    keyword:     [133, 82,143],
    comment:     [147,161,161],
    string:      [42, 161,152],
    number:      [181,137,  0],
    operator:    [211, 54,130],
    function:    [38, 139,210],
    builtin:     [108,113,196],
    output_bg:   [238,232,213],
    output_fg:   [101,123,131],
    canvas_bg:   [253,246,227],
    button_bg:   [238,232,213],
    button_fg:   [88, 110,117],
    accent:      [38, 139,210],
    error:       [220, 50, 47],
    warning:     [181,137,  0],
    success:     [133,153,  0],
}}

pub fn github_light() -> Theme { Theme {
    name: "GitHub Light".into(),
    category: ThemeCategory::Light,
    description: "Clean and minimal, inspired by GitHub".into(),
    background:  [255,255,255],
    foreground:  [36, 41, 47],
    panel_bg:    [246,248,250],
    border:      [208,215,222],
    editor_bg:   [255,255,255],
    editor_fg:   [36, 41, 47],
    line_number: [139,148,158],
    cursor:      [36, 41, 47],
    selection:   [172,213,255,180],
    keyword:     [207, 34, 46],
    comment:     [110,119,129],
    string:      [10, 48,105],
    number:      [8, 109,221],
    operator:    [36, 41, 47],
    function:    [130, 80,223],
    builtin:     [5, 80,174],
    output_bg:   [246,248,250],
    output_fg:   [36, 41, 47],
    canvas_bg:   [255,255,255],
    button_bg:   [235,238,242],
    button_fg:   [36, 41, 47],
    accent:      [9, 105,218],
    error:       [207, 34, 46],
    warning:     [155, 80, 0],
    success:     [26,127, 55],
}}

// ── font size presets ──────────────────────────────────────────────────────────

/// Named font-size presets for quick selection.
pub const FONT_SIZE_PRESETS: &[(&str, f32)] = &[
    ("Tiny (10pt)",     10.0),
    ("Small (12pt)",    12.0),
    ("Medium (14pt)",   14.0),
    ("Large (16pt)",    16.0),
    ("X-Large (20pt)",  20.0),
    ("Huge (24pt)",     24.0),
];

// ── theme manager ─────────────────────────────────────────────────────────────

pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    pub current: String,
}

impl Default for ThemeManager {
    fn default() -> Self {
        let mut m = Self { themes: HashMap::new(), current: "Dracula".to_string() };
        for t in [
            // Retro
            amber_terminal(), green_screen(), c64(), borland_turbo(), cga(), apple_ii(),
            // Dark
            dracula(), monokai(), vscode_dark(), solarized_dark(), ocean_blue(),
            sunset(), nord(), gruvbox_dark(), tokyo_night(), one_dark(),
            // Light
            spring(), candy(), solarized_light(), github_light(),
        ] {
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

    /// Return themes grouped by category, sorted within each group.
    pub fn themes_by_category(&self) -> Vec<(ThemeCategory, Vec<&Theme>)> {
        let mut groups: Vec<(ThemeCategory, Vec<&Theme>)> = Vec::new();
        for cat in ThemeCategory::all() {
            let mut ts: Vec<&Theme> = self.themes.values()
                .filter(|t| t.category == *cat)
                .collect();
            ts.sort_by(|a, b| a.name.cmp(&b.name));
            groups.push((*cat, ts));
        }
        groups
    }

    pub fn get(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }
}

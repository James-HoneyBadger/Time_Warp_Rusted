//! Turtle graphics state — faithful Rust port of `turtle_state.py`.
//!
//! Canvas coordinates: (0, 0) is centre; Y-axis is inverted (up = negative).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── colour helpers ───────────────────────────────────────────────────────────

pub type Rgb = (u8, u8, u8);

/// Named colour palette — comprehensive set covering standard Logo, BASIC,
/// HTML/CSS and common educational-programming colour names.
pub fn named_colors() -> HashMap<&'static str, Rgb> {
    let mut m = HashMap::new();
    // Core colours
    m.insert("BLACK",       (0,   0,   0));
    m.insert("WHITE",       (255, 255, 255));
    m.insert("RED",         (255, 0,   0));
    m.insert("GREEN",       (0,   255, 0));
    m.insert("BLUE",        (0,   0,   255));
    m.insert("YELLOW",      (255, 255, 0));
    m.insert("CYAN",        (0,   255, 255));
    m.insert("MAGENTA",     (255, 0,   255));
    m.insert("ORANGE",      (255, 165, 0));
    m.insert("PURPLE",      (128, 0,   128));
    m.insert("BROWN",       (165, 42,  42));
    m.insert("PINK",        (255, 192, 203));
    m.insert("GRAY",        (128, 128, 128));
    m.insert("GREY",        (128, 128, 128));

    // Extended / web-standard colours
    m.insert("LIME",        (0,   255, 0));
    m.insert("AQUA",        (0,   255, 255));
    m.insert("TEAL",        (0,   128, 128));
    m.insert("NAVY",        (0,   0,   128));
    m.insert("MAROON",      (128, 0,   0));
    m.insert("OLIVE",       (128, 128, 0));
    m.insert("SILVER",      (192, 192, 192));
    m.insert("GOLD",        (255, 215, 0));
    m.insert("VIOLET",      (238, 130, 238));
    m.insert("INDIGO",      (75,  0,   130));
    m.insert("CORAL",       (255, 127, 80));
    m.insert("SALMON",      (250, 128, 114));
    m.insert("TURQUOISE",   (64,  224, 208));
    m.insert("TAN",         (210, 180, 140));
    m.insert("CRIMSON",     (220, 20,  60));
    m.insert("TOMATO",      (255, 99,  71));
    m.insert("SKYBLUE",     (135, 206, 235));
    m.insert("CHARTREUSE",  (127, 255, 0));
    m.insert("KHAKI",       (240, 230, 140));
    m.insert("PLUM",        (221, 160, 221));
    m.insert("ORCHID",      (218, 112, 214));
    m.insert("SIENNA",      (160, 82,  45));
    m.insert("PERU",        (205, 133, 63));
    m.insert("BEIGE",       (245, 245, 220));
    m.insert("IVORY",       (255, 255, 240));
    m.insert("LAVENDER",    (230, 230, 250));
    m.insert("FUCHSIA",     (255, 0,   255));

    // Light / dark variants commonly used by students
    m.insert("LIGHTBLUE",   (173, 216, 230));
    m.insert("LIGHTGREEN",  (144, 238, 144));
    m.insert("LIGHTGRAY",   (211, 211, 211));
    m.insert("LIGHTGREY",   (211, 211, 211));
    m.insert("LIGHTPINK",   (255, 182, 193));
    m.insert("LIGHTYELLOW", (255, 255, 224));
    m.insert("LIGHTCYAN",   (224, 255, 255));
    m.insert("DARKBLUE",    (0,   0,   139));
    m.insert("DARKGREEN",   (0,   100, 0));
    m.insert("DARKRED",     (139, 0,   0));
    m.insert("DARKGRAY",    (169, 169, 169));
    m.insert("DARKGREY",    (169, 169, 169));
    m.insert("DARKCYAN",    (0,   139, 139));
    m.insert("DARKMAGENTA", (139, 0,   139));
    m.insert("DARKORANGE",  (255, 140, 0));
    m.insert("DARKVIOLET",  (148, 0,   211));

    // Aliases: traditional Logo uses "GREEN" for bright green (0,255,0)
    // rather than the HTML "GREEN" which is (0,128,0).  Provide both:
    m.insert("BRIGHTGREEN", (0,   255, 0));
    m.insert("BRIGHTRED",   (255, 0,   0));
    m.insert("BRIGHTBLUE",  (0,   0,   255));

    m
}

/// Parse a colour string into an RGB triple.
/// Accepts:
///  - Named colours (case-insensitive)
///  - `#RRGGBB` hex strings
///  - Decimal palette indices 0–15
///  - Logo-style quoted strings: `"RED"` or `"RED`
pub fn parse_color(s: &str) -> Option<Rgb> {
    // Strip surrounding/leading/trailing quote marks (Logo uses " as word prefix,
    // and example files sometimes write "RED" with both quotes)
    let cleaned = s.trim().trim_matches('"');
    let up = cleaned.to_uppercase();

    // Named colours
    if let Some(&c) = named_colors().get(up.as_str()) {
        return Some(c);
    }

    // Hex colour
    if up.starts_with('#') && up.len() == 7 {
        let r = u8::from_str_radix(&up[1..3], 16).ok()?;
        let g = u8::from_str_radix(&up[3..5], 16).ok()?;
        let b = u8::from_str_radix(&up[5..7], 16).ok()?;
        return Some((r, g, b));
    }

    // Palette index 0–15 (GW-BASIC / CGA palette)
    if let Ok(idx) = up.parse::<u8>() {
        return default_palette_16(idx);
    }

    None
}

/// 16-colour CGA palette matching the Python `DEFAULT_PALETTE_16`.
pub fn default_palette_16(idx: u8) -> Option<Rgb> {
    let palette: [Rgb; 16] = [
        (0,   0,   0),   // 0  black
        (0,   0,   170), // 1  blue
        (0,   170, 0),   // 2  green
        (0,   170, 170), // 3  cyan
        (170, 0,   0),   // 4  red
        (170, 0,   170), // 5  magenta
        (170, 85,  0),   // 6  brown
        (170, 170, 170), // 7  light grey
        (85,  85,  85),  // 8  dark grey
        (85,  85,  255), // 9  bright blue
        (85,  255, 85),  // 10 bright green
        (85,  255, 255), // 11 bright cyan
        (255, 85,  85),  // 12 bright red
        (255, 85,  255), // 13 bright magenta
        (255, 255, 85),  // 14 bright yellow
        (255, 255, 255), // 15 white
    ];
    palette.get(idx as usize).copied()
}

// ── drawing primitives ───────────────────────────────────────────────────────

/// A single line segment drawn by the turtle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurtleLine {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x:   f64,
    pub end_y:   f64,
    pub color:   Rgb,
    pub width:   f64,
}

/// Shape type tag for high-level drawing primitives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeType {
    Arc,
    Polygon,
    Text,
    Dot,
}

/// A generic shape (arc, polygon, label, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurtleShape {
    pub shape_type: ShapeType,
    pub points:     Vec<(f64, f64)>,
    pub color:      Rgb,
    pub width:      f64,
    pub fill_color: Option<Rgb>,
    pub text:       Option<String>,
    pub font_size:  u32,
    pub align:      String,
    /// Additional float data used by arcs: [cx, cy, radius, start_angle_deg, span_deg]
    pub arc_data:   Option<[f64; 5]>,
}

// ── turtle state ─────────────────────────────────────────────────────────────

/// Complete turtle graphics state — port of Python `TurtleState`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurtleState {
    pub x:            f64,
    pub y:            f64,
    /// Heading in degrees: 0 = up (north), 90 = right (east).
    pub heading:      f64,
    pub pen_down:     bool,
    pub pen_color:    Rgb,
    pub pen_width:    f64,
    pub bg_color:     Rgb,
    pub canvas_width: f64,
    pub canvas_height: f64,
    pub is_visible:   bool,
    pub lines:        Vec<TurtleLine>,
    pub shapes:       Vec<TurtleShape>,
    /// Fill mode: collecting outline for a filled polygon.
    filling:          bool,
    fill_color:       Rgb,
    fill_poly:        Vec<(f64, f64)>,
}

impl Default for TurtleState {
    fn default() -> Self {
        Self {
            x:             0.0,
            y:             0.0,
            heading:       0.0,
            pen_down:      true,
            pen_color:     (255, 255, 255),
            pen_width:     2.0,
            bg_color:      (40,  42,  54),
            canvas_width:  800.0,
            canvas_height: 600.0,
            is_visible:    true,
            lines:         Vec::new(),
            shapes:        Vec::new(),
            filling:       false,
            fill_color:    (255, 255, 255),
            fill_poly:     Vec::new(),
        }
    }
}

impl TurtleState {
    pub fn new() -> Self {
        Self::default()
    }

    // ── movement ─────────────────────────────────────────────────────────

    /// Move forward by `distance` pixels, drawing a line when pen is down.
    pub fn forward(&mut self, distance: f64) {
        let rad = self.heading.to_radians();
        // Heading 0 = north; x increases east, y increases south on screen
        let dx =  distance * rad.sin();
        let dy = -distance * rad.cos();
        self.move_to(self.x + dx, self.y + dy);
    }

    /// Move backward by `distance` pixels.
    pub fn backward(&mut self, distance: f64) {
        self.forward(-distance);
    }

    /// Turn right (clockwise) by `angle` degrees.
    pub fn right(&mut self, angle: f64) {
        self.heading = (self.heading + angle) % 360.0;
        if self.heading < 0.0 {
            self.heading += 360.0;
        }
    }

    /// Turn left (counter-clockwise) by `angle` degrees.
    pub fn left(&mut self, angle: f64) {
        self.right(-angle);
    }

    /// Move turtle to absolute position, optionally drawing.
    pub fn move_to(&mut self, nx: f64, ny: f64) {
        if self.pen_down {
            self.lines.push(TurtleLine {
                start_x: self.x,
                start_y: self.y,
                end_x:   nx,
                end_y:   ny,
                color:   self.pen_color,
                width:   self.pen_width,
            });
        }
        // Always collect fill polygon points when filling,
        // regardless of pen state (matches standard turtle behaviour).
        if self.filling {
            if self.fill_poly.is_empty() {
                self.fill_poly.push((self.x, self.y));
            }
            self.fill_poly.push((nx, ny));
        }
        self.x = nx;
        self.y = ny;
    }

    /// Teleport to position without drawing.
    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Set just the X coordinate without drawing.
    pub fn set_x(&mut self, x: f64) {
        let y = self.y;
        self.move_to(x, y);
    }

    /// Set just the Y coordinate without drawing.
    pub fn set_y(&mut self, y: f64) {
        let x = self.x;
        self.move_to(x, y);
    }

    /// Return to home position (0, 0), heading north; no line drawn.
    pub fn home(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.heading = 0.0;
    }

    // ── heading ──────────────────────────────────────────────────────────

    /// Set absolute heading in degrees.
    pub fn set_heading(&mut self, angle: f64) {
        self.heading = angle % 360.0;
        if self.heading < 0.0 {
            self.heading += 360.0;
        }
    }

    // ── pen ──────────────────────────────────────────────────────────────

    pub fn pen_up(&mut self)   { self.pen_down = false; }
    pub fn pen_down_cmd(&mut self) { self.pen_down = true; }

    pub fn set_pen_color(&mut self, color: Rgb) { self.pen_color = color; }
    pub fn set_pen_width(&mut self, w: f64)     { self.pen_width = w.max(0.5); }
    pub fn set_bg_color(&mut self, color: Rgb)  { self.bg_color = color; }

    // ── display ──────────────────────────────────────────────────────────

    pub fn hide_turtle(&mut self) { self.is_visible = false; }
    pub fn show_turtle(&mut self) { self.is_visible = true; }

    // ── clear ────────────────────────────────────────────────────────────

    /// Erase all drawing; keep turtle position & heading.
    pub fn clear_screen(&mut self) {
        self.lines.clear();
        self.shapes.clear();
        self.fill_poly.clear();
        self.filling = false;
    }

    /// Full reset to defaults.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    // ── arcs / shapes ────────────────────────────────────────────────────

    /// Draw an arc of `radius` pixels spanning `angle` degrees.
    pub fn arc(&mut self, radius: f64, angle: f64) {
        let steps = (angle.abs() / 5.0).max(4.0) as usize;
        let d_angle = angle / steps as f64;
        for _ in 0..steps {
            self.forward(2.0 * std::f64::consts::PI * radius / (360.0 / d_angle.abs()));
            self.right(d_angle);
        }
    }

    /// Draw a dot at the current position with given radius and colour.
    pub fn dot(&mut self, radius: f64, color: Option<Rgb>) {
        let c = color.unwrap_or(self.pen_color);
        self.shapes.push(TurtleShape {
            shape_type: ShapeType::Dot,
            points: vec![(self.x, self.y)],
            color: c,
            width: radius * 2.0,
            fill_color: Some(c),
            text: None,
            font_size: 12,
            align: "left".to_string(),
            arc_data: None,
        });
    }

    /// Place text label at the current position.
    pub fn label(&mut self, text: &str, font_size: u32) {
        self.shapes.push(TurtleShape {
            shape_type: ShapeType::Text,
            points: vec![(self.x, self.y)],
            color: self.pen_color,
            width: 1.0,
            fill_color: None,
            text: Some(text.to_string()),
            font_size,
            align: "left".to_string(),
            arc_data: None,
        });
    }

    // ── fill ─────────────────────────────────────────────────────────────

    pub fn begin_fill(&mut self, color: Rgb) {
        self.filling = true;
        self.fill_color = color;
        self.fill_poly = vec![(self.x, self.y)];
    }

    pub fn end_fill(&mut self) {
        if self.filling && self.fill_poly.len() >= 3 {
            self.shapes.push(TurtleShape {
                shape_type: ShapeType::Polygon,
                points: self.fill_poly.clone(),
                color: self.pen_color,
                width: self.pen_width,
                fill_color: Some(self.fill_color),
                text: None,
                font_size: 12,
                align: "left".to_string(),
                arc_data: None,
            });
        }
        self.filling = false;
        self.fill_poly.clear();
    }

    // ── queries ──────────────────────────────────────────────────────────

    pub fn position(&self) -> (f64, f64) { (self.x, self.y) }
    pub fn heading(&self) -> f64         { self.heading }

    /// Distance from current position to `(tx, ty)`.
    pub fn distance_to(&self, tx: f64, ty: f64) -> f64 {
        ((self.x - tx).powi(2) + (self.y - ty).powi(2)).sqrt()
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── parse_color ──────────────────────────────────────────────────────

    #[test]
    fn parse_named_colors_all() {
        // Every named colour in the map must round-trip through parse_color.
        for (name, expected) in named_colors() {
            let result = parse_color(name);
            assert_eq!(result, Some(expected), "parse_color({name:?}) failed");
        }
    }

    #[test]
    fn parse_named_case_insensitive() {
        assert_eq!(parse_color("red"),    Some((255, 0, 0)));
        assert_eq!(parse_color("Red"),    Some((255, 0, 0)));
        assert_eq!(parse_color("RED"),    Some((255, 0, 0)));
        assert_eq!(parse_color("rEd"),    Some((255, 0, 0)));
        assert_eq!(parse_color("green"),  Some((0, 255, 0)));
        assert_eq!(parse_color("BLUE"),   Some((0, 0, 255)));
        assert_eq!(parse_color("yellow"), Some((255, 255, 0)));
    }

    #[test]
    fn parse_logo_quoted_colors() {
        // Logo `"RED"` — both quotes
        assert_eq!(parse_color("\"RED\""),   Some((255, 0, 0)));
        // Logo `"RED` — leading quote only
        assert_eq!(parse_color("\"RED"),     Some((255, 0, 0)));
        // Already stripped quotes
        assert_eq!(parse_color("RED"),       Some((255, 0, 0)));
        // Mixed case with quotes
        assert_eq!(parse_color("\"Green\""), Some((0, 255, 0)));
        assert_eq!(parse_color("\"blue"),    Some((0, 0, 255)));
    }

    #[test]
    fn parse_hex_colors() {
        assert_eq!(parse_color("#FF0000"), Some((255, 0, 0)));
        assert_eq!(parse_color("#00FF00"), Some((0, 255, 0)));
        assert_eq!(parse_color("#0000FF"), Some((0, 0, 255)));
        assert_eq!(parse_color("#FFFFFF"), Some((255, 255, 255)));
        assert_eq!(parse_color("#000000"), Some((0, 0, 0)));
        assert_eq!(parse_color("#ff8040"), Some((255, 128, 64)));
        // With Logo quotes
        assert_eq!(parse_color("\"#FF0000\""), Some((255, 0, 0)));
        assert_eq!(parse_color("\"#00FFFF"),   Some((0, 255, 255)));
    }

    #[test]
    fn parse_palette_index() {
        assert_eq!(parse_color("0"),  Some((0, 0, 0)));       // black
        assert_eq!(parse_color("1"),  Some((0, 0, 170)));     // blue
        assert_eq!(parse_color("4"),  Some((170, 0, 0)));     // red
        assert_eq!(parse_color("15"), Some((255, 255, 255))); // white
    }

    #[test]
    fn parse_palette_16_complete() {
        // All 16 indices must return a colour.
        for i in 0u8..16 {
            assert!(default_palette_16(i).is_some(), "palette index {i} returned None");
        }
        // Beyond 15 must be None.
        assert_eq!(default_palette_16(16), None);
        assert_eq!(default_palette_16(255), None);
    }

    #[test]
    fn parse_invalid_returns_none() {
        assert_eq!(parse_color(""), None);
        assert_eq!(parse_color("NOTACOLOR"), None);
        assert_eq!(parse_color("999"), None);  // valid u8 but beyond palette 16
        assert_eq!(parse_color("#GG0000"), None); // invalid hex
        assert_eq!(parse_color("#FF00"),   None); // too short
    }

    #[test]
    fn parse_whitespace_trimmed() {
        assert_eq!(parse_color("  RED  "), Some((255, 0, 0)));
        assert_eq!(parse_color(" #FF0000 "), Some((255, 0, 0)));
        assert_eq!(parse_color(" 4 "), Some((170, 0, 0)));
    }

    // ── Extended colours ────────────────────────────────────────────────

    #[test]
    fn extended_colors_present() {
        assert_eq!(parse_color("lime"),      Some((0, 255, 0)));
        assert_eq!(parse_color("navy"),      Some((0, 0, 128)));
        assert_eq!(parse_color("teal"),      Some((0, 128, 128)));
        assert_eq!(parse_color("maroon"),    Some((128, 0, 0)));
        assert_eq!(parse_color("olive"),     Some((128, 128, 0)));
        assert_eq!(parse_color("silver"),    Some((192, 192, 192)));
        assert_eq!(parse_color("gold"),      Some((255, 215, 0)));
        assert_eq!(parse_color("violet"),    Some((238, 130, 238)));
        assert_eq!(parse_color("indigo"),    Some((75, 0, 130)));
        assert_eq!(parse_color("coral"),     Some((255, 127, 80)));
        assert_eq!(parse_color("turquoise"), Some((64, 224, 208)));
        assert_eq!(parse_color("fuchsia"),   Some((255, 0, 255)));
    }

    #[test]
    fn light_dark_variants() {
        assert!(parse_color("lightblue").is_some());
        assert!(parse_color("lightgreen").is_some());
        assert!(parse_color("darkblue").is_some());
        assert!(parse_color("darkgreen").is_some());
        assert!(parse_color("darkred").is_some());
        assert!(parse_color("LIGHTGRAY").is_some());
        assert!(parse_color("DARKGREY").is_some());
    }

    // ── TurtleState colour integration ───────────────────────────────────

    #[test]
    fn set_pen_color_applies() {
        let mut t = TurtleState::new();
        assert_eq!(t.pen_color, (255, 255, 255)); // default white
        t.set_pen_color((255, 0, 0));
        assert_eq!(t.pen_color, (255, 0, 0));
    }

    #[test]
    fn lines_carry_pen_color() {
        let mut t = TurtleState::new();
        t.set_pen_color((0, 255, 0));
        t.forward(10.0);
        assert_eq!(t.lines.len(), 1);
        assert_eq!(t.lines[0].color, (0, 255, 0));
    }

    #[test]
    fn pen_color_changes_mid_draw() {
        let mut t = TurtleState::new();
        t.set_pen_color((255, 0, 0));
        t.forward(10.0);
        t.set_pen_color((0, 0, 255));
        t.forward(10.0);
        assert_eq!(t.lines[0].color, (255, 0, 0));
        assert_eq!(t.lines[1].color, (0, 0, 255));
    }

    #[test]
    fn fill_uses_fill_color() {
        let mut t = TurtleState::new();
        t.set_pen_color((255, 0, 0)); // pen = red
        t.begin_fill((0, 255, 0));    // fill = green
        t.forward(50.0);
        t.right(90.0);
        t.forward(50.0);
        t.right(90.0);
        t.forward(50.0);
        t.end_fill();
        assert_eq!(t.shapes.len(), 1);
        assert_eq!(t.shapes[0].fill_color, Some((0, 255, 0))); // fill is green
        assert_eq!(t.shapes[0].color, (255, 0, 0)); // outline is pen colour (red)
    }

    #[test]
    fn dot_uses_pen_color() {
        let mut t = TurtleState::new();
        t.set_pen_color((255, 128, 0));
        t.dot(5.0, None);
        assert_eq!(t.shapes.len(), 1);
        assert_eq!(t.shapes[0].color, (255, 128, 0));
    }

    #[test]
    fn label_uses_pen_color() {
        let mut t = TurtleState::new();
        t.set_pen_color((0, 200, 200));
        t.label("test", 12);
        assert_eq!(t.shapes.len(), 1);
        assert_eq!(t.shapes[0].color, (0, 200, 200));
    }

    #[test]
    fn bg_color_independent() {
        let mut t = TurtleState::new();
        let orig_pen = t.pen_color;
        t.set_bg_color((100, 100, 100));
        assert_eq!(t.bg_color, (100, 100, 100));
        assert_eq!(t.pen_color, orig_pen); // unchanged
    }
}

//! Turtle graphics state — faithful Rust port of `turtle_state.py`.
//!
//! Canvas coordinates: (0, 0) is centre; Y-axis is inverted (up = negative).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── colour helpers ───────────────────────────────────────────────────────────

pub type Rgb = (u8, u8, u8);

/// Named colour palette matching the Python implementation.
pub fn named_colors() -> HashMap<&'static str, Rgb> {
    let mut m = HashMap::new();
    m.insert("BLACK",   (0,   0,   0));
    m.insert("WHITE",   (255, 255, 255));
    m.insert("RED",     (255, 0,   0));
    m.insert("GREEN",   (0,   255, 0));
    m.insert("BLUE",    (0,   0,   255));
    m.insert("YELLOW",  (255, 255, 0));
    m.insert("CYAN",    (0,   255, 255));
    m.insert("MAGENTA", (255, 0,   255));
    m.insert("PINK",    (255, 192, 203));
    m.insert("GRAY",    (128, 128, 128));
    m.insert("GREY",    (128, 128, 128));
    m.insert("ORANGE",  (255, 165, 0));
    m.insert("PURPLE",  (128, 0,   128));
    m.insert("BROWN",   (165, 42,  42));
    m
}

/// Parse a colour string into an RGB triple.
/// Accepts:
///  - Named colours (case-insensitive)
///  - `#RRGGBB` hex strings
///  - Decimal palette indices 0–15
pub fn parse_color(s: &str) -> Option<Rgb> {
    let up = s.trim().to_uppercase();

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
            if self.filling {
                if self.fill_poly.is_empty() {
                    self.fill_poly.push((self.x, self.y));
                }
                self.fill_poly.push((nx, ny));
            }
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

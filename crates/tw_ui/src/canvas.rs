//! Turtle canvas renderer using `egui::Painter`.
//! Port of `ui/canvas.py`.

use egui::{Color32, Painter, Pos2, Rect, Sense, Stroke, Vec2};
use tw_graphics::turtle::{ShapeType, TurtleState, TurtleShape};

/// Render the current `TurtleState` onto an egui canvas region.
pub struct TurtleCanvas {
    pub zoom:    f32,
    pub pan:     Vec2,
    dragging:    bool,
    drag_start:  Pos2,
    pan_start:   Vec2,
}

impl Default for TurtleCanvas {
    fn default() -> Self {
        Self { zoom: 1.0, pan: Vec2::ZERO, dragging: false,
               drag_start: Pos2::ZERO, pan_start: Vec2::ZERO }
    }
}

impl TurtleCanvas {
    pub fn new() -> Self { Self::default() }

    pub fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.pan  = Vec2::ZERO;
    }

    /// Draw the canvas.  Call this inside a `Frame` or `CentralPanel`.
    pub fn show(&mut self, ui: &mut egui::Ui, turtle: &TurtleState) {
        // Guard against NaN / infinite / degenerate available sizes
        let avail = ui.available_size();
        let safe_w = if avail.x.is_finite() && avail.x > 1.0 { avail.x } else { 200.0 };
        let safe_h = if avail.y.is_finite() && avail.y > 1.0 { avail.y } else { 200.0 };
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(safe_w, safe_h), Sense::drag());
        if rect.any_nan() { return; }

        // Background
        let bg = {
            let c = turtle.bg_color;
            Color32::from_rgb(c.0, c.1, c.2)
        };
        ui.painter().rect_filled(rect, 0.0, bg);

        // Handle zoom via scroll
        let scroll = ui.input(|i| i.raw_scroll_delta.y);
        if rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or(Pos2::ZERO))) && scroll != 0.0 {
            self.zoom = (self.zoom * (1.0 + scroll * 0.001)).clamp(0.05, 20.0);
        }

        // Handle pan via drag
        if resp.drag_started() {
            self.dragging = true;
            self.drag_start  = ui.input(|i| i.pointer.press_origin().unwrap_or(Pos2::ZERO));
            self.pan_start   = self.pan;
        }
        if self.dragging {
            let cur = ui.input(|i| i.pointer.interact_pos().unwrap_or(self.drag_start));
            self.pan = self.pan_start + (cur - self.drag_start);
        }
        if resp.drag_stopped() { self.dragging = false; }

        let painter = ui.painter_at(rect);
        let center  = rect.center() + self.pan;

        // Convert turtle coordinate → screen
        let to_screen = |tx: f64, ty: f64| -> Pos2 {
            Pos2::new(
                center.x + (tx as f32) * self.zoom,
                center.y - (ty as f32) * self.zoom,  // Y-axis flip
            )
        };

        // Draw lines
        for line in &turtle.lines {
            let c = line.color;
            let col = Color32::from_rgb(c.0, c.1, c.2);
            let w   = (line.width as f32) * self.zoom;
            let a   = to_screen(line.start_x, line.start_y);
            let b   = to_screen(line.end_x, line.end_y);
            painter.line_segment([a, b], Stroke::new(w.max(1.0), col));
        }

        // Draw shapes (filled polygons / circles / dots / arcs / text)
        for shape in &turtle.shapes {
            draw_shape(&painter, shape, self.zoom, &to_screen);
        }

                // Draw turtle cursor (triangle arrow)
        draw_turtle_cursor(&painter, turtle, &to_screen);

        // Grid overlay (shown when zoomed in enough)
        if self.zoom > 3.0 {
            draw_grid(&painter, rect, center, self.zoom);
        }
    }
}

fn draw_shape(
    painter: &Painter,
    shape: &TurtleShape,
    zoom: f32,
    to_screen: &impl Fn(f64, f64) -> Pos2,
) {
    let c = shape.color;
    let col = Color32::from_rgb(c.0, c.1, c.2);
    let fill_col = shape.fill_color.map(|fc| Color32::from_rgb(fc.0, fc.1, fc.2));

    match shape.shape_type {
        ShapeType::Polygon => {
            if shape.points.len() >= 3 {
                let screen_pts: Vec<Pos2> = shape.points.iter()
                    .map(|(px, py)| to_screen(*px, *py))
                    .collect();
                let fill = fill_col.unwrap_or(col);
                painter.add(egui::Shape::convex_polygon(screen_pts, fill, Stroke::NONE));
            }
        }
        ShapeType::Dot => {
            if let Some(&(px, py)) = shape.points.first() {
                // radius stored in arc_data[2] or width
                let radius = shape.arc_data.map(|d| d[2]).unwrap_or(shape.width);
                painter.circle_filled(to_screen(px, py), (radius as f32) * zoom, col);
            }
        }
        ShapeType::Arc => {
            if let Some([cx, cy, radius, start_deg, span_deg]) = shape.arc_data {
                let steps = (span_deg.abs() as usize).min(360).max(4);
                let step = span_deg / steps as f64;
                let pts: Vec<Pos2> = (0..=steps).map(|i| {
                    let ang = (start_deg + step * i as f64).to_radians();
                    to_screen(cx + radius * ang.cos(), cy + radius * ang.sin())
                }).collect();
                for pair in pts.windows(2) {
                    painter.line_segment([pair[0], pair[1]], Stroke::new(zoom.max(1.0), col));
                }
            }
        }
        ShapeType::Text => {
            if let Some(text) = &shape.text {
                if let Some(&(px, py)) = shape.points.first() {
                    painter.text(
                        to_screen(px, py),
                        egui::Align2::LEFT_BOTTOM,
                        text,
                        egui::FontId::proportional((shape.font_size as f32) * zoom),
                        col,
                    );
                }
            }
        }
    }
}

fn draw_turtle_cursor(
    painter: &Painter,
    turtle: &TurtleState,
    to_screen: &impl Fn(f64, f64) -> Pos2,
) {
    if !turtle.is_visible { return; }
    let pos = to_screen(turtle.x, turtle.y);
    let angle_rad = ((turtle.heading - 90.0) as f32).to_radians();
    let size = 10.0f32;
    let tip = Pos2::new(
        pos.x + angle_rad.cos() * size,
        pos.y + angle_rad.sin() * size,
    );
    let left_angle  = angle_rad + std::f32::consts::PI * 2.0 / 3.0;
    let right_angle = angle_rad - std::f32::consts::PI * 2.0 / 3.0;
    let back_left  = Pos2::new(pos.x + left_angle.cos()  * size * 0.7, pos.y + left_angle.sin()  * size * 0.7);
    let back_right = Pos2::new(pos.x + right_angle.cos() * size * 0.7, pos.y + right_angle.sin() * size * 0.7);

    painter.add(egui::Shape::convex_polygon(
        vec![tip, back_left, back_right],
        Color32::from_rgba_premultiplied(100, 220, 100, 200),
        Stroke::new(1.0, Color32::WHITE),
    ));
}

fn draw_grid(painter: &Painter, rect: Rect, center: Pos2, zoom: f32) {
    let step = zoom * 10.0;
    let col  = Color32::from_rgba_premultiplied(100, 100, 100, 40);
    let s    = Stroke::new(0.5, col);

    let mut x = center.x % step;
    while x < rect.right() {
        painter.line_segment([Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())], s);
        x += step;
    }
    let mut y = center.y % step;
    while y < rect.bottom() {
        painter.line_segment([Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)], s);
        y += step;
    }
    // Axis
    let axis = Stroke::new(1.0, Color32::from_rgba_premultiplied(150, 150, 150, 80));
    painter.line_segment([Pos2::new(center.x, rect.top()), Pos2::new(center.x, rect.bottom())], axis);
    painter.line_segment([Pos2::new(rect.left(), center.y), Pos2::new(rect.right(), center.y)], axis);
}

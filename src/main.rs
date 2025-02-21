#![allow(unused, dead_code)]

use gpui::*;
use std::{f32::consts::PI, time::Duration};

struct ColoredPoint {
    position: Point<Pixels>,
    color: gpui::Hsla,
}

struct FractalViewer {
    quads: Vec<gpui::PaintQuad>,
    temp_quads: Vec<gpui::PaintQuad>,
    paths: Vec<(gpui::Path<Pixels>, gpui::Hsla)>,
    formula_points: Vec<ColoredPoint>,
    epoch: u64,
}

impl FractalViewer {
    fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(|this, mut cx| async move {
            loop {
                Timer::after(Duration::from_millis(8)).await;
                this.update(&mut cx, |this, cx| {
                    this.update_epoch(cx);
                })
                .ok();
            }
        })
        .detach();

        Self {
            quads: vec![],
            temp_quads: vec![],
            paths: vec![],
            formula_points: Vec::new(),
            epoch: 0,
        }
    }
    fn update_epoch(&mut self, cx: &mut Context<Self>) {
        const START_EPOCH: u64 = 0;
        const MAX_EPOCH: u64 = 512;
        let mut max_cycles = 1;

        if self.epoch == MAX_EPOCH - 1 {
            max_cycles -= 1;
            if max_cycles == 0 {
                return;
            }
        }

        let direction = if (self.epoch / MAX_EPOCH) % 2 == 0 {
            1
        } else {
            -1
        };
        self.epoch = (self.epoch as i64 + direction).rem_euclid(MAX_EPOCH as i64) as u64;

        if self.epoch == START_EPOCH {
            // Clear all quads at the start of each cycle
            self.quads.clear();
            self.temp_quads.clear();
        } else if self.epoch % 8 == 0 {
            // Keep the current quads and add them to the permanent set
            self.quads.append(&mut self.temp_quads);
        } else {
            // Clear only the temporary quads
            self.temp_quads.clear();
        }

        let epoch = self.epoch as f32;
        let center_x = 384.0;
        let center_y = 384.0;
        let radius = 200.0;
        let num_points = 16;
        let angle_step = 2.0 * PI / num_points as f32;

        self.generate_formula_points(
            |t| {
                let mut angle = t * angle_step + (epoch as f32 / 128.0);

                // Rotate points on odd epochs
                if epoch % 2.0 != 0.0 {
                    angle += PI;
                }

                // Add some chaotic variation to the radius
                let chaotic_factor = (epoch * t).sin() * 0.2;
                let base_radius = radius * (1.0 + 0.5 * (epoch / 128.0).sin() + chaotic_factor);

                // Apply a spiraling effect
                let spiral_factor = (epoch / 32.0) * 0.1;
                let x = center_x + base_radius * (angle + spiral_factor).cos();
                let y = center_y + base_radius * (angle + spiral_factor).sin();

                // Calculate color based on angle and epoch with more variation
                let hue = (t + (epoch / 64.0).sin()) % 1.0;
                let saturation =
                    0.5 + 0.5 * ((epoch / MAX_EPOCH as f32).sin() * (t * 2.0 * PI).cos());
                let lightness = 0.5 + 0.3 * ((epoch / 64.0).cos() * (t * 3.0 * PI).sin());

                let color = gpui::hsla(hue, saturation, lightness, 1.0);

                ColoredPoint {
                    position: point(px(x), px(y)),
                    color,
                }
            },
            (0.0, num_points as f32),
            1.0,
        );

        // Add lines casting from the radial shape
        for i in 0..num_points {
            let angle = i as f32 * angle_step + (epoch as f32 / 75.0);
            let base_radius = radius * (1.0 + 0.5 * (epoch / 128.0).sin()); // Increased variation
            let start_x = center_x + base_radius * angle.cos();
            let start_y = center_y + base_radius * angle.sin();
            let line_length = radius * (0.5 + 1.0 * (epoch / 64.0).cos()); // Greatly increased variation
            let end_x = start_x + line_length * angle.cos();
            let end_y = start_y + line_length * angle.sin();

            for t in 0..4 {
                let t = t as f32 / 20.0;
                let x = start_x + t * (end_x - start_x);
                let y = start_y + t * (end_y - start_y);

                let hue = i as f32 / num_points as f32;
                let saturation = 0.8;
                let lightness = 0.5;
                let opacity = (1.0 - t) * 0.8; // Increased opacity
                let color = gpui::hsla(hue, saturation, lightness, opacity);

                let pixel = shapes::pixel(point(px(x), px(y))).color(color);
                self.temp_quads.push(pixel.quad());
            }
        }

        self.draw_formula();
        cx.notify();
    }

    fn generate_formula_points(
        &mut self,
        formula: impl Fn(f32) -> ColoredPoint,
        range: (f32, f32),
        step: f32,
    ) {
        self.formula_points.clear();
        let (start, end) = range;
        let mut t = start;
        while t <= end {
            let colored_point = formula(t);
            self.formula_points.push(colored_point);
            t += step;
        }
    }

    fn draw_formula(&mut self) {
        for colored_point in self.formula_points.iter() {
            let quad = shapes::pixel(colored_point.position)
                .color(colored_point.color)
                .quad();
            if self.epoch % 8 == 0 {
                self.quads.push(quad);
            } else {
                self.temp_quads.push(quad);
            }
        }
    }
}

impl Render for FractalViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut all_quads = self.quads.clone();
        all_quads.extend(self.temp_quads.clone());
        let paths = self.paths.clone();

        div()
            .bg(gpui::black())
            .size_full()
            .child(render_canvas(all_quads, paths, window, cx))
    }
}

fn render_canvas(
    quads: Vec<gpui::PaintQuad>,
    paths: Vec<(gpui::Path<Pixels>, gpui::Hsla)>,
    _window: &mut Window,
    _cx: &mut App,
) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |_, _, window, _| {
            for quad in quads.iter() {
                window.paint_quad(quad.clone());
            }
            for (path, color) in paths.iter() {
                window.paint_path(path.clone(), *color);
            }
        },
    )
    .size_full()
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("gpui".into()),
                    ..Default::default()
                }),
                window_bounds: Some(gpui::WindowBounds::Windowed(Bounds::new(
                    point(px(0.), px(0.)),
                    Size::new(px(768.), px(768.)),
                ))),
                focus: true,
                ..Default::default()
            },
            |_, cx| cx.new(|cx| FractalViewer::new(cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}

mod shapes {

    use super::*;

    pub struct Stroke {
        width: Pixels,
        color: gpui::Hsla,
    }

    pub struct ShapeProperties {
        fill: Background,
        position: Point<Pixels>,
        size: Pixels,
        stroke: Stroke,
    }

    impl From<Hsla> for Stroke {
        fn from(color: Hsla) -> Self {
            Stroke {
                width: px(1.),
                color,
            }
        }
    }

    pub fn circle(radius: impl Into<Pixels>, position: Point<Pixels>) -> Circle {
        Circle::new(radius.into(), position)
    }

    pub fn pixel(position: Point<Pixels>) -> Pixel {
        Pixel::new(position)
    }

    pub fn line(start: Point<Pixels>, end: Point<Pixels>) -> Line {
        Line::new(start, end)
    }

    pub fn triangle(p1: Point<Pixels>, p2: Point<Pixels>, p3: Point<Pixels>) -> Triangle {
        Triangle::new(p1, p2, p3)
    }

    pub struct Circle {
        fill: gpui::Background,
        position: Point<Pixels>,
        size: Pixels,
        stroke: Stroke,
    }

    pub struct Pixel {
        position: Point<Pixels>,
        color: gpui::Hsla,
    }

    pub struct Line {
        start: Point<Pixels>,
        end: Point<Pixels>,
        stroke: Stroke,
    }

    pub struct Triangle {
        p1: Point<Pixels>,
        p2: Point<Pixels>,
        p3: Point<Pixels>,
        stroke: Stroke,
    }

    impl Circle {
        pub fn new(radius: Pixels, position: Point<Pixels>) -> Self {
            let stroke: Stroke = gpui::white().into();
            let fill: gpui::Background = gpui::transparent_black().into();

            Circle {
                stroke,
                fill,
                size: radius * 2.0,
                position,
            }
        }

        pub fn stroke_width(mut self, width: impl Into<Pixels>) -> Self {
            self.stroke.width = width.into();
            self
        }

        pub fn stroke_color(mut self, color: impl Into<gpui::Hsla>) -> Self {
            self.stroke.color = color.into();
            self
        }

        pub fn fill(mut self, fill: impl Into<gpui::Background>) -> Self {
            self.fill = fill.into();
            self
        }

        pub fn no_stroke(mut self) -> Self {
            self.stroke = Stroke {
                width: px(0.),
                color: gpui::transparent_black(),
            };
            self
        }

        pub fn quad(&self) -> gpui::PaintQuad {
            let center = point(self.position.x, self.position.y);

            let bounds = Bounds::centered_at(
                center,
                Size {
                    width: self.size,
                    height: self.size,
                },
            );
            let background = self.fill;
            let border_color = self.stroke.color;
            let border_width = self.stroke.width;
            let corner_radii = Corners::all(self.size / 2.0);

            gpui::PaintQuad {
                bounds,
                corner_radii,
                background,
                border_widths: Edges::all(border_width),
                border_color,
            }
        }
    }

    impl Pixel {
        pub fn new(position: Point<Pixels>) -> Self {
            Pixel {
                position,
                color: gpui::white(),
            }
        }

        pub fn color(mut self, color: impl Into<gpui::Hsla>) -> Self {
            self.color = color.into();
            self
        }

        pub fn quad(&self) -> gpui::PaintQuad {
            let bounds = Bounds::from_corner_and_size(
                gpui::Corner::TopLeft,
                self.position,
                Size {
                    width: px(1.),
                    height: px(1.),
                },
            );
            let background = self.color.into();

            gpui::PaintQuad {
                bounds,
                corner_radii: Corners::default(),
                background,
                border_widths: Edges::default(),
                border_color: gpui::transparent_black(),
            }
        }
    }

    impl Line {
        pub fn new(start: Point<Pixels>, end: Point<Pixels>) -> Self {
            Line {
                start,
                end,
                stroke: gpui::white().into(),
            }
        }

        pub fn stroke_width(mut self, width: impl Into<Pixels>) -> Self {
            self.stroke.width = width.into();
            self
        }

        pub fn stroke_color(mut self, color: impl Into<gpui::Hsla>) -> Self {
            self.stroke.color = color.into();
            self
        }

        pub fn paint(&self) -> (gpui::Path<Pixels>, gpui::Hsla) {
            let mut path = Path::new(self.start);
            path.line_to(self.end);
            (path, self.stroke.color)
        }
    }

    impl Triangle {
        pub fn new(p1: Point<Pixels>, p2: Point<Pixels>, p3: Point<Pixels>) -> Self {
            Triangle {
                p1,
                p2,
                p3,
                stroke: gpui::white().into(),
            }
        }

        pub fn stroke_width(mut self, width: impl Into<Pixels>) -> Self {
            self.stroke.width = width.into();
            self
        }

        pub fn stroke_color(mut self, color: impl Into<gpui::Hsla>) -> Self {
            self.stroke.color = color.into();
            self
        }

        pub fn paint(&self) -> (gpui::Path<Pixels>, gpui::Hsla) {
            let mut path = Path::new(self.p1);
            path.line_to(self.p2);
            path.line_to(self.p3);
            path.line_to(self.p1);
            (path, self.stroke.color)
        }
    }
}

mod julia_set {
    use super::*;
    use gpui::hsla;
    use num_complex::Complex;

    pub fn generate(
        width: usize,
        height: usize,
        c: Complex<f32>,
        max_iterations: u32,
    ) -> Vec<gpui::PaintQuad> {
        let mut quads = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let scaled_x = (x as f32 / width as f32) * 4.0 - 2.0;
                let scaled_y = (y as f32 / height as f32) * 4.0 - 2.0;
                let mut z = Complex::new(scaled_x, scaled_y);

                let mut i = 0;
                while i < max_iterations && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }

                if i < max_iterations {
                    let color = hsla((i as f32 / max_iterations as f32) * 360.0, 100.0, 50.0, 1.0);
                    let pixel = shapes::pixel(point(px(x as f32), px(y as f32))).color(color);
                    quads.push(pixel.quad());
                }
            }
        }

        quads
    }
}

mod circular_sierpinski2 {
    use super::*;

    pub fn carpet(
        center: Point<Pixels>,
        radius: Pixels,
        depth: u32,
        angle: f32,
    ) -> Vec<gpui::PaintQuad> {
        recursive(center, radius, depth, angle)
    }

    fn recursive(
        center: Point<Pixels>,
        radius: Pixels,
        depth: u32,
        angle: f32,
    ) -> Vec<gpui::PaintQuad> {
        let mut quads = Vec::new();

        if depth == 0 {
            return quads;
        }

        quads.push(shapes::circle(radius, center).quad());

        let inner_radius = radius / 3.0;
        let offset = radius * 2.0 / 3.0;

        for i in 0..8 {
            let circle_angle = i as f32 * PI / 4.0 + angle;
            let x = center.x + px(offset.0 * circle_angle.cos());
            let y = center.y + px(offset.0 * circle_angle.sin());
            quads.extend(recursive(Point { x, y }, inner_radius, depth - 1, angle));
        }

        quads.extend(recursive(center, inner_radius, depth - 1, angle));

        quads
    }
}

mod dragon {
    use super::*;

    pub fn curve(
        start: Point<Pixels>,
        end: Point<Pixels>,
        iterations: u32,
    ) -> Vec<(gpui::Path<Pixels>, gpui::Hsla)> {
        let mut paths = Vec::new();
        recursive(&mut paths, start, end, iterations, true);
        paths
    }

    fn recursive(
        paths: &mut Vec<(gpui::Path<Pixels>, gpui::Hsla)>,
        start: Point<Pixels>,
        end: Point<Pixels>,
        iterations: u32,
        is_right: bool,
    ) {
        if iterations == 0 {
            paths.push(shapes::line(start, end).paint());
        } else {
            let mid = Point {
                x: (start.x + end.x) / 2.0
                    + (end.y - start.y) / 2.0 * if is_right { -1.0 } else { 1.0 },
                y: (start.y + end.y) / 2.0
                    + (start.x - end.x) / 2.0 * if is_right { -1.0 } else { 1.0 },
            };
            recursive(paths, start, mid, iterations - 1, true);
            recursive(paths, mid, end, iterations - 1, false);
        }
    }
}

mod koch {
    use super::*;

    pub fn snowflake(
        start: Point<Pixels>,
        side_length: f32,
        iterations: u32,
    ) -> Vec<(gpui::Path<Pixels>, gpui::Hsla)> {
        let mut paths = Vec::new();
        let height = side_length * 3f32.sqrt() / 2.0;

        let p1 = start;
        let p2 = start + point(px(side_length), px(0.0));
        let p3 = start + point(px(side_length / 2.0), px(height));

        side(&mut paths, p1, p2, iterations);
        side(&mut paths, p2, p3, iterations);
        side(&mut paths, p3, p1, iterations);

        paths
    }

    fn side(
        paths: &mut Vec<(gpui::Path<Pixels>, gpui::Hsla)>,
        start: Point<Pixels>,
        end: Point<Pixels>,
        iterations: u32,
    ) {
        if iterations == 0 {
            paths.push(shapes::line(start, end).paint());
        } else {
            let delta = end - start;
            let third = Point {
                x: delta.x / 3.0,
                y: delta.y / 3.0,
            };

            let p1 = start;
            let p2 = start + third;
            let p3 = {
                let angle = PI / 3.0;
                let rotated_third = Point {
                    x: third.x * angle.cos() - third.y * angle.sin(),
                    y: third.x * angle.sin() + third.y * angle.cos(),
                };
                p2 + rotated_third
            };
            let p4 = start
                + Point {
                    x: 2.0 * third.x,
                    y: 2.0 * third.y,
                };
            let p5 = end;

            side(paths, p1, p2, iterations - 1);
            side(paths, p2, p3, iterations - 1);
            side(paths, p3, p4, iterations - 1);
            side(paths, p4, p5, iterations - 1);
        }
    }
}

mod sierpinski {
    use gpui::Point;

    use super::*;

    pub fn triangle(
        start: Point<Pixels>,
        side_length: f32,
        iterations: u32,
    ) -> Vec<(gpui::Path<Pixels>, gpui::Hsla)> {
        let mut paths = Vec::new();
        recursive(&mut paths, start, side_length, iterations);
        paths
    }

    fn recursive(
        paths: &mut Vec<(gpui::Path<Pixels>, gpui::Hsla)>,
        start: Point<Pixels>,
        side_length: f32,
        iterations: u32,
    ) {
        if iterations == 0 {
            let height = side_length * 3f32.sqrt() / 2.0;
            let p1 = start;
            let p2 = start + point(px(side_length), px(0.0));
            let p3 = start + point(px(side_length / 2.0), px(height));
            paths.push(shapes::triangle(p1, p2, p3).paint());
        } else {
            let new_side = side_length / 2.0;
            recursive(paths, start, new_side, iterations - 1);
            recursive(
                paths,
                start + point(px(new_side), px(0.0)),
                new_side,
                iterations - 1,
            );
            recursive(
                paths,
                start + point(px(new_side / 2.0), px(new_side * 3f32.sqrt() / 2.0)),
                new_side,
                iterations - 1,
            );
        }
    }
}

mod pythagoras {
    use gpui::Point;

    use super::*;

    pub fn tree(
        start: Point<Pixels>,
        size: f32,
        angle: f32,
        iterations: u32,
    ) -> Vec<(gpui::Path<Pixels>, gpui::Hsla)> {
        let mut paths = Vec::new();
        recursive(&mut paths, start, size, angle, iterations);
        paths
    }

    fn recursive(
        paths: &mut Vec<(gpui::Path<Pixels>, gpui::Hsla)>,
        start: Point<Pixels>,
        size: f32,
        angle: f32,
        iterations: u32,
    ) {
        if iterations == 0 {
            return;
        }

        let end = Point {
            x: start.x + px(size * angle.cos()),
            y: start.y - px(size * angle.sin()),
        };

        paths.push(shapes::line(start, end).paint());

        let new_size = size / 2f32.sqrt();
        let new_angle1 = angle + PI / 4.0;
        let new_angle2 = angle - PI / 4.0;

        recursive(paths, end, new_size, new_angle1, iterations - 1);
        recursive(paths, end, new_size, new_angle2, iterations - 1);
    }
}

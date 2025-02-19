#![allow(dead_code)]

use gpui::{point, px, Path, Pixels, Point};
use std::f32::consts::PI;

pub mod shapes {
    use gpui::{Bounds, Corners, Edges, Hsla, Size, Window};

    use super::*;

    pub struct Stroke {
        width: Pixels,
        color: gpui::Hsla,
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

pub mod julia_set {
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

pub mod circular_sierpinski2 {
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

pub mod dragon {
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

pub mod koch {
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

pub mod sierpinski {
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

pub mod pythagoras {
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

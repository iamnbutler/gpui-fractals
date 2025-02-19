#![allow(dead_code)]

use gpui::{point, px, Path, PathBuilder, Pixels, Point};
use std::f32::consts::PI;

pub mod shapes2 {
    use gpui::{Bounds, Corners, Edges, Hsla, Size};

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

    pub struct Circle {
        fill: gpui::Background,
        position: Point<Pixels>,
        size: Pixels,
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
            // let half_size = self.size / 2.0;
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

        quads.push(shapes2::circle(radius, center).quad());

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

mod shapes {
    use super::*;

    pub fn circle(builder: &mut PathBuilder, center: Point<Pixels>, radius: Pixels, segments: u32) {
        let angle_step = 2.0 * PI / segments as f32;

        let start = Point {
            x: center.x + radius,
            y: center.y,
        };
        builder.move_to(start);

        for i in 1..=segments {
            let angle = i as f32 * angle_step;
            let point = Point {
                x: center.x + px(radius.0 * angle.cos()),
                y: center.y + px(radius.0 * angle.sin()),
            };
            builder.line_to(point);
        }

        builder.close();
    }

    pub fn line(builder: &mut PathBuilder, start: Point<Pixels>, end: Point<Pixels>) {
        builder.move_to(start);
        builder.line_to(end);
    }

    pub fn triangle(
        builder: &mut PathBuilder,
        p1: Point<Pixels>,
        p2: Point<Pixels>,
        p3: Point<Pixels>,
    ) {
        builder.move_to(p1);
        builder.line_to(p2);
        builder.line_to(p3);
        builder.close();
    }
}

pub mod dragon {
    use super::*;

    pub fn curve(
        start: Point<Pixels>,
        end: Point<Pixels>,
        iterations: u32,
    ) -> Option<Path<Pixels>> {
        let mut builder = PathBuilder::stroke(px(1.));
        recursive(&mut builder, start, end, iterations, true);
        builder.build().ok()
    }

    fn recursive(
        builder: &mut PathBuilder,
        start: Point<Pixels>,
        end: Point<Pixels>,
        iterations: u32,
        is_right: bool,
    ) {
        if iterations == 0 {
            shapes::line(builder, start, end);
        } else {
            let mid = Point {
                x: (start.x + end.x) / 2.0
                    + (end.y - start.y) / 2.0 * if is_right { -1.0 } else { 1.0 },
                y: (start.y + end.y) / 2.0
                    + (start.x - end.x) / 2.0 * if is_right { -1.0 } else { 1.0 },
            };
            recursive(builder, start, mid, iterations - 1, true);
            recursive(builder, mid, end, iterations - 1, false);
        }
    }
}

pub mod koch {
    use super::*;

    pub fn snowflake(
        start: Point<Pixels>,
        side_length: f32,
        iterations: u32,
    ) -> Option<Path<Pixels>> {
        let height = side_length * 3f32.sqrt() / 2.0;
        let mut builder = PathBuilder::stroke(px(1.));

        let p1 = start;
        let p2 = start + point(px(side_length), px(0.0));
        let p3 = start + point(px(side_length / 2.0), px(height));

        side(&mut builder, p1, p2, iterations);
        side(&mut builder, p2, p3, iterations);
        side(&mut builder, p3, p1, iterations);

        builder.build().ok()
    }

    fn side(builder: &mut PathBuilder, start: Point<Pixels>, end: Point<Pixels>, iterations: u32) {
        if iterations == 0 {
            shapes::line(builder, start, end);
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

            side(builder, p1, p2, iterations - 1);
            side(builder, p2, p3, iterations - 1);
            side(builder, p3, p4, iterations - 1);
            side(builder, p4, p5, iterations - 1);
        }
    }
}

pub mod sierpinski {
    use super::*;

    pub fn triangle(
        start: Point<Pixels>,
        side_length: f32,
        iterations: u32,
    ) -> Option<Path<Pixels>> {
        let mut builder = PathBuilder::stroke(px(1.));
        recursive(&mut builder, start, side_length, iterations);
        builder.build().ok()
    }

    fn recursive(
        builder: &mut PathBuilder,
        start: Point<Pixels>,
        side_length: f32,
        iterations: u32,
    ) {
        if iterations == 0 {
            let height = side_length * 3f32.sqrt() / 2.0;
            let p1 = start;
            let p2 = start + point(px(side_length), px(0.0));
            let p3 = start + point(px(side_length / 2.0), px(height));
            shapes::triangle(builder, p1, p2, p3);
        } else {
            let new_side = side_length / 2.0;
            recursive(builder, start, new_side, iterations - 1);
            recursive(
                builder,
                start + point(px(new_side), px(0.0)),
                new_side,
                iterations - 1,
            );
            recursive(
                builder,
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
    ) -> Option<Path<Pixels>> {
        let mut builder = PathBuilder::stroke(px(1.));
        recursive(&mut builder, start, size, angle, iterations);
        builder.build().ok()
    }

    fn recursive(
        builder: &mut PathBuilder,
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

        shapes::line(builder, start, end);

        let new_size = size / 2f32.sqrt();
        let new_angle1 = angle + PI / 4.0;
        let new_angle2 = angle - PI / 4.0;

        recursive(builder, end, new_size, new_angle1, iterations - 1);
        recursive(builder, end, new_size, new_angle2, iterations - 1);
    }
}

pub mod circular_sierpinski {
    use super::*;

    pub fn carpet(
        center: Point<Pixels>,
        radius: Pixels,
        depth: u32,
        circle_segments: u32,
    ) -> Option<Path<Pixels>> {
        let mut builder = PathBuilder::stroke(px(1.));
        recursive(&mut builder, center, radius, depth, circle_segments);
        builder.build().ok()
    }

    fn recursive(
        builder: &mut PathBuilder,
        center: Point<Pixels>,
        radius: Pixels,
        depth: u32,
        circle_segments: u32,
    ) {
        if depth == 0 {
            return;
        }

        shapes::circle(builder, center, radius, circle_segments);

        let inner_radius = radius * 1.0 / 3.0;
        let offset = radius * 2.0 / 3.0;

        for i in 0..8 {
            let angle = i as f32 * PI / 4.0;
            let x = center.x + px(offset.0 * angle.cos());
            let y = center.y + px(offset.0 * angle.sin());
            recursive(
                builder,
                Point { x, y },
                inner_radius,
                depth - 1,
                circle_segments,
            );
        }

        recursive(builder, center, inner_radius, depth - 1, circle_segments);
    }
}

use gpui::{point, px, Path, PathBuilder, Pixels, Point};
use std::f32::consts::PI;

// Helper functions for drawing basic shapes
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

// Dragon Curve
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

// Koch Snowflake
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

// Sierpinski Triangle
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

// Pythagoras Tree
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

// Circular Sierpinski Carpet
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

        // Draw inner circles
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

        // Draw center circle
        recursive(builder, center, inner_radius, depth - 1, circle_segments);
    }
}

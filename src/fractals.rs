use gpui::{point, px, Path, PathBuilder, Pixels, Point};

pub fn dragon_curve(
    start: Point<Pixels>,
    end: Point<Pixels>,
    iterations: u32,
) -> Option<Path<Pixels>> {
    let mut builder = PathBuilder::stroke(px(1.));
    dragon_recursive(&mut builder, start, end, iterations, true);
    builder.build().ok()
}

pub fn dragon_recursive(
    builder: &mut PathBuilder,
    start: Point<Pixels>,
    end: Point<Pixels>,
    iterations: u32,
    is_right: bool,
) {
    if iterations == 0 {
        builder.move_to(start);
        builder.line_to(end);
    } else {
        let mid = Point {
            x: (start.x + end.x) / 2.0
                + (end.y - start.y) / 2.0 * if is_right { -1.0 } else { 1.0 },
            y: (start.y + end.y) / 2.0
                + (start.x - end.x) / 2.0 * if is_right { -1.0 } else { 1.0 },
        };
        dragon_recursive(builder, start, mid, iterations - 1, true);
        dragon_recursive(builder, mid, end, iterations - 1, false);
    }
}

pub fn koch_snowflake(start: Point<Pixels>, side_length: f32, iterations: u32) -> Path<Pixels> {
    let height = side_length * 3f32.sqrt() / 2.0;
    let mut builder = PathBuilder::stroke(px(1.));

    let p1 = start;
    let p2 = start + point(px(side_length), px(0.0));
    let p3 = start + point(px(side_length / 2.0), px(height));

    koch_side(&mut builder, p1, p2, iterations);
    koch_side(&mut builder, p2, p3, iterations);
    koch_side(&mut builder, p3, p1, iterations);

    builder.build().unwrap()
}

pub fn koch_side(
    builder: &mut PathBuilder,
    start: Point<Pixels>,
    end: Point<Pixels>,
    iterations: u32,
) {
    if iterations == 0 {
        builder.move_to(start);
        builder.line_to(end);
    } else {
        let delta = end - start;
        let third = Point {
            x: delta.x / 3.0,
            y: delta.y / 3.0,
        };

        let p1 = start;
        let p2 = start + third;
        let p3 = {
            let angle = std::f32::consts::PI / 3.0;
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

        koch_side(builder, p1, p2, iterations - 1);
        koch_side(builder, p2, p3, iterations - 1);
        koch_side(builder, p3, p4, iterations - 1);
        koch_side(builder, p4, p5, iterations - 1);
    }
}

pub fn sierpinski_triangle(
    start: Point<Pixels>,
    side_length: f32,
    iterations: u32,
) -> Path<Pixels> {
    let mut builder = PathBuilder::stroke(px(1.));
    sierpinski_recursive(&mut builder, start, side_length, iterations);
    builder.build().unwrap()
}

pub fn sierpinski_recursive(
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

        builder.move_to(p1);
        builder.line_to(p2);
        builder.line_to(p3);
        builder.line_to(p1);
    } else {
        let new_side = side_length / 2.0;
        sierpinski_recursive(builder, start, new_side, iterations - 1);
        sierpinski_recursive(
            builder,
            start + point(px(new_side), px(0.0)),
            new_side,
            iterations - 1,
        );
        sierpinski_recursive(
            builder,
            start + point(px(new_side / 2.0), px(new_side * 3f32.sqrt() / 2.0)),
            new_side,
            iterations - 1,
        );
    }
}

pub fn pythagoras_tree(
    start: Point<Pixels>,
    size: f32,
    angle: f32,
    iterations: u32,
) -> Path<Pixels> {
    let mut builder = PathBuilder::stroke(px(1.));
    pythagoras_recursive(&mut builder, start, size, angle, iterations);
    builder.build().unwrap()
}

pub fn pythagoras_recursive(
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

    builder.move_to(start);
    builder.line_to(end);

    let new_size = size / 2f32.sqrt();
    let new_angle1 = angle + std::f32::consts::PI / 4.0;
    let new_angle2 = angle - std::f32::consts::PI / 4.0;

    pythagoras_recursive(builder, end, new_size, new_angle1, iterations - 1);
    pythagoras_recursive(builder, end, new_size, new_angle2, iterations - 1);
}

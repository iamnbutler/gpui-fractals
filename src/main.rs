use gpui::{
    canvas, div, point, prelude::*, px, App, Application, Bounds, Context, Pixels, Render, Size,
    TitlebarOptions, Window, WindowOptions,
};
use num_complex::Complex;

mod fractals;

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

struct FractalViewer {
    quads: Vec<gpui::PaintQuad>,
    paths: Vec<(gpui::Path<Pixels>, gpui::Hsla)>,
}

impl FractalViewer {
    fn new(_cx: &mut Context<Self>) -> Self {
        let quads = vec![
            fractals::shapes::circle(px(50.0), point(px(100.0), px(100.0))).quad(),
            fractals::shapes::pixel(point(px(200.0), px(200.0)))
                .color(gpui::red())
                .quad(),
        ];
        let paths = vec![
            fractals::shapes::line(point(px(50.0), px(50.0)), point(px(150.0), px(150.0))).paint(),
            fractals::shapes::triangle(
                point(px(300.0), px(300.0)),
                point(px(350.0), px(300.0)),
                point(px(325.0), px(350.0)),
            )
            .paint(),
        ];
        // let paths = fractals::pythagoras::tree(point(px(128.), px(128.)), 128., 0., 8);

        Self { quads, paths }
    }

    // fn create_fractal(c: Complex<f32>) -> Vec<gpui::PaintQuad> {
    //     julia_set::generate(512, 512, c, 25)
    // }
    //
}

impl Render for FractalViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let quads = self.quads.clone();
        let paths = self.paths.clone();

        div()
            .bg(gpui::black())
            .size_full()
            .child(render_canvas(quads, paths, window, cx))
    }
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
                    Size::new(px(512.), px(512.)),
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

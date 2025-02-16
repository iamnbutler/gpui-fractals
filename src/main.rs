use gpui::{
    canvas, div, point, prelude::*, px, Application, Bounds, Context, Path, PathBuilder, Pixels,
    Render, Size, TitlebarOptions, Window, WindowOptions,
};

mod fractals;

use fractals::*;

struct FractalViewer {
    fractal: Path<Pixels>,
}

impl FractalViewer {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let center = point(px(400.), px(300.));
        let radius = px(200.);
        let depth = 5;
        let fractal = circular_sierpinski::carpet(point(px(256.), px(256.)), px(200.), 4, 32)
            .unwrap_or_else(|| PathBuilder::stroke(px(1.)).build().unwrap());
        Self { fractal }
    }
}

impl Render for FractalViewer {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let fractal = self.fractal.clone();

        div().bg(gpui::black()).size_full().child(
            canvas(
                move |_, _, _| {},
                move |_, _, window, _| {
                    window.paint_path(fractal, gpui::white());
                },
            )
            .size_full(),
        )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("gpui: fractal viewer".into()),
                    ..Default::default()
                }),
                window_bounds: Some(gpui::WindowBounds::Windowed(Bounds::new(
                    point(px(0.), px(0.)),
                    Size::new(px(512.), px(512.)),
                ))),
                focus: true,
                ..Default::default()
            },
            |window, cx| cx.new(|cx| FractalViewer::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}

use gpui::{
    canvas, div, point, prelude::*, px, Application, Bounds, Context, Path, Pixels, Render, Size,
    Window, WindowOptions,
};

mod fractals;

use fractals::*;

struct PaintingViewer {
    dragon: Option<Path<Pixels>>,
}

impl PaintingViewer {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let dragon = dragon_curve(point(px(106.), px(256.)), point(px(406.), px(256.)), 12);

        Self { dragon }
    }
}

impl Render for PaintingViewer {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let dragon = self.dragon.clone();

        div().bg(gpui::black()).size_full().child(
            canvas(
                move |_, _, _| {},
                move |_, _, window, _| {
                    if let Some(dragon) = dragon {
                        window.paint_path(dragon, gpui::white());
                    }
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
                window_bounds: Some(gpui::WindowBounds::Windowed(Bounds::new(
                    point(px(0.), px(0.)),
                    Size::new(px(512.), px(512.)),
                ))),
                focus: true,
                ..Default::default()
            },
            |window, cx| cx.new(|cx| PaintingViewer::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}

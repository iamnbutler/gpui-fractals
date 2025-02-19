use std::time::Duration;

use gpui::{
    canvas, div, point, prelude::*, px, App, Application, Bounds, Context, Path, PathBuilder,
    Pixels, Render, Size, Timer, TitlebarOptions, Window, WindowOptions,
};

mod fractals;

use fractals::*;

fn render_fractal(
    quads: Vec<gpui::PaintQuad>,
    _window: &mut Window,
    _cx: &mut App,
) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |_, _, window, _| {
            for quad in quads.iter() {
                window.paint_quad(quad.clone());
            }
        },
    )
    .size_full()
}

struct FractalViewer {
    quads: Vec<gpui::PaintQuad>,
    size: f32,
}

impl FractalViewer {
    fn new(cx: &mut Context<Self>) -> Self {
        let size = 64.0;
        let quads = Self::create_fractal(size, 0.0);

        cx.spawn(|this, mut cx| async move {
            loop {
                Timer::after(Duration::from_millis(8)).await;
                this.update(&mut cx, |this, cx| {
                    this.grow(cx);
                })
                .ok();
            }
        })
        .detach();

        Self { quads, size }
    }

    fn create_fractal(size: f32, angle: f32) -> Vec<gpui::PaintQuad> {
        circular_sierpinski2::carpet(point(px(256.), px(256.)), px(size), 4, angle)
    }

    fn grow(&mut self, cx: &mut Context<Self>) {
        self.size += 1.0;
        if self.size > 1024.0 {
            self.size = 8.0;
        }
        let angle = (self.size / 256.0) % 360.0;
        self.quads = Self::create_fractal(self.size, angle);
        cx.notify();
    }
}

impl Render for FractalViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let quads = self.quads.clone();

        div()
            .bg(gpui::black())
            .size_full()
            .child(render_fractal(quads, window, cx))
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
            |_, cx| cx.new(|cx| FractalViewer::new(cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}

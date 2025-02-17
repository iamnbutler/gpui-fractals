use std::time::Duration;

use gpui::{
    canvas, div, point, prelude::*, px, Application, Bounds, Context, Path, PathBuilder, Pixels,
    Render, Size, Timer, TitlebarOptions, Window, WindowOptions,
};

mod fractals;

use fractals::*;

struct FractalViewer {
    fractal: Path<Pixels>,
    size: f32,
}

impl FractalViewer {
    fn new(cx: &mut Context<Self>) -> Self {
        let size = 50.0;
        let fractal = Self::create_fractal(size);

        // Set up a timer to grow the fractal
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

        Self { fractal, size }
    }

    fn create_fractal(size: f32) -> Path<Pixels> {
        circular_sierpinski::carpet(point(px(256.), px(256.)), px(size), 4, 32)
            .unwrap_or_else(|| PathBuilder::stroke(px(1.)).build().unwrap())
    }

    fn grow(&mut self, cx: &mut Context<Self>) {
        self.size += 1.0; // Increase size by 0.5 pixels each frame
        if self.size > 1024.0 {
            self.size = 8.0; // Reset size when it gets too large
        }
        self.fractal = Self::create_fractal(self.size);
        cx.notify();
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
            |_, cx| cx.new(|cx| FractalViewer::new(cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}

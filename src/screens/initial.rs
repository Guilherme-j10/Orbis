use std::{f32::consts::PI, rc::Rc, sync::RwLock};

use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::dpi::PhysicalSize;

use crate::interfaces::app::AppState;

pub struct InitialScreen<'a, T: Renderer> {
    canvas: &'a mut Canvas<T>,
    app_state: Rc<RwLock<AppState>>,
    bounds: (f32, f32),
    width: f32,
    height: f32,
}

impl<'a, T: Renderer> InitialScreen<'a, T> {
    pub fn render(
        canvas: &'a mut Canvas<T>,
        app_state: Rc<RwLock<AppState>>,
        bounds: (f32, f32),
        psize: &PhysicalSize<u32>,
    ) -> Self {
        let mut screen = Path::new();
        screen.rect(
            bounds.0,
            bounds.1,
            psize.width as f32 - bounds.0,
            psize.height as f32 - bounds.1,
        );

        canvas.fill_path(&screen, &Paint::color(Color::rgb(10, 10, 14)));

        Self {
            canvas,
            app_state,
            bounds,
            height: psize.height as f32,
            width: psize.width as f32,
        }
    }

    pub fn resolve_font_map(&mut self) -> () {
        let state = self.app_state.read().expect("failed to get app state");
        let have_font_map = false;

        let center_x = (self.bounds.0 + self.width) / 2.0;
        let center_y = (self.bounds.1 + self.height) / 2.0;

        let default_paint = Paint::color(Color::rgb(35, 35, 48));
        let mut logo_path = Path::new();
        logo_path.arc(
            center_x,
            center_y,
            50.0,
            0.0,
            PI * 2.0,
            femtovg::Solidity::Solid,
        );

        let mut small_circle = Path::new();
        small_circle.circle(center_x, center_y, 25.0);

        self.canvas.fill_path(&small_circle, &default_paint);
        self.canvas
            .stroke_path(&logo_path, &default_paint.clone().with_line_width(5.0));

        let text_paint = Paint::color(Color::rgb(50, 50, 69))
            .with_font(&state.font_ids)
            .with_font_italic(false)
            .with_font_weight(500.0);

        let content = "It looks like you don't have a source mapping yet—let's create one!";
        let metrics_data = self
            .canvas
            .measure_text(center_x, 0.0, &content, &text_paint)
            .expect("failed to get font data");
        self.canvas
            .fill_text(
                center_x - (metrics_data.width() / 2.0),
                center_y + 95.0,
                content,
                &text_paint,
            )
            .expect("Failed to fill text");
    }
}

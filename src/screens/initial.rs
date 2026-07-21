use std::f32::consts::PI;

use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::dpi::PhysicalSize;

use crate::interfaces::app::{AppScreens, AppStateType};

pub struct InitialScreen<'a, T: Renderer> {
    canvas: &'a mut Canvas<T>,
    app_state: AppStateType,
    bounds: (f32, f32),
    width: f32,
    height: f32,
}

impl<'a, T: Renderer> InitialScreen<'a, T> {
    pub fn initialize(
        canvas: &'a mut Canvas<T>,
        app_state: AppStateType,
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

    pub fn render(&mut self) -> () {
        let fonts_ids = self.app_state.font_ids.borrow();
        let _have_font_map = false;

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
            .with_font(&fonts_ids)
            .with_font_italic(false)
            .with_font_weight(500.0);

        let title = "It looks like you don't have a source mapping yet—let's create one!";
        let call_action = "Click on the screen to start.";

        let call_action_metrics = self
            .canvas
            .measure_text(center_x, 0.0, &call_action, &text_paint)
            .expect("Failed to get font data");
        let title_metrics = self
            .canvas
            .measure_text(center_x, 0.0, &title, &text_paint)
            .expect("Failed to get font data");

        self.canvas
            .fill_text(
                center_x - (title_metrics.width() / 2.0),
                center_y + 95.0,
                title,
                &text_paint,
            )
            .expect("Failed to fill text");
        self.canvas
            .fill_text(
                center_x - (call_action_metrics.width() / 2.0),
                center_y + 120.0,
                call_action,
                &text_paint,
            )
            .expect("Failed to fill text");

        if self.app_state.had_click() == true {
            self.app_state.current_screen.set(AppScreens::FontEditor);
        }
    }
}

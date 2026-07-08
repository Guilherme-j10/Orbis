use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::dpi::PhysicalSize;

use crate::interfaces::app::AppStateType;

pub struct FontEditorScreen<'a, T: Renderer> {
    canvas: &'a mut Canvas<T>,
    app_state: AppStateType,
    bounds: (f32, f32),
}

impl<'a, T: Renderer> FontEditorScreen<'a, T> {
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
        }
    }
}

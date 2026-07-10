use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::dpi::PhysicalSize;

use crate::{
    font_engine::{dimensions::FontDimension, font::FontPadding, font_mask::FontMask},
    interfaces::app::AppStateType,
};

pub struct FontEditorScreen<'a, T: Renderer> {
    canvas: &'a mut Canvas<T>,
    app_state: AppStateType,
    bounds: (f32, f32),
    psize: &'a PhysicalSize<u32>,
}

impl<'a, T: Renderer> FontEditorScreen<'a, T> {
    pub fn initialize(
        canvas: &'a mut Canvas<T>,
        app_state: AppStateType,
        bounds: (f32, f32),
        psize: &'a PhysicalSize<u32>,
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
            psize,
        }
    }

    pub fn render(&mut self) -> () {
        let horizontal_margin = 300.0;
        let margin_top = 100.0;
        let font_size = 50.0;
        let padding = FontPadding {
            horizontal: 20.0,
            vertical: 5.5,
        };

        let font_dimension = FontDimension::new(font_size, &padding);

        let chars: Vec<&str> = "abcdefghijklmnopqrstuvwxyz0123456789"
            .trim()
            .split("")
            .filter(|f| !f.is_empty())
            .collect();

        let bounds: [(f32, f32); 2] = [
            (horizontal_margin, margin_top), // x, y
            (self.psize.width as f32 - horizontal_margin, margin_top),
        ];

        let total_line_size = self.psize.width as f32 - horizontal_margin * 2.0;
        let total_in_line = total_line_size / font_dimension.get_complete_width().0;

        for (ci, i) in chars
            .chunks_exact(total_in_line.floor() as usize)
            .enumerate()
        {
            for (index, c) in i.iter().enumerate() {
                if !c.is_empty() {
                    let position_x =
                        bounds[0].0 + (font_dimension.get_complete_width().0 * index as f32);
                    let position_y =
                        bounds[0].1 + (font_dimension.get_complete_width().1 * ci as f32);

                    FontMask::initialize(
                        &mut self.canvas,
                        self.app_state.clone(),
                        (position_x, position_y),
                        font_size,
                        Some(padding.clone()),
                        c,
                    );
                }
            }
        }
    }
}

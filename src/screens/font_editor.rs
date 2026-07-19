use std::{cell::Cell, collections::HashMap};

use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::dpi::PhysicalSize;

use crate::{
    font_engine::{
        dimensions::FontDimension,
        font::{FontPadding, OrbParts},
        font_mask::{FontMask, FontMaskProp},
    },
    interfaces::app::AppStateType,
};

pub struct FontEditorScreen<'a, T: Renderer> {
    canvas: &'a mut Canvas<T>,
    app_state: AppStateType,
    _bounds: (f32, f32),
    psize: &'a PhysicalSize<u32>,
    binded_char: Cell<HashMap<String, OrbParts>>,
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
            _bounds: bounds,
            psize,
            binded_char: Cell::new(HashMap::default()),
        }
    }

    pub fn render(&mut self) -> () {
        let draw_bounds_points = false;
        let horizontal_margin = 270.0;
        let margin_top = 100.0;
        let font_size = 50.0;
        let padding = FontPadding {
            horizontal: 20.0,
            vertical: 5.5,
        };

        let font_dimension = FontDimension::new(&font_size, &padding);

        let chars: Vec<&str> = "abcdefghijklmnopqrstuvwxyz0123456789"
            .trim()
            .split("")
            .filter(|f| !f.is_empty())
            .collect();

        let bounds: [(f32, f32); 2] = [
            (horizontal_margin, margin_top), // x, y
            (self.psize.width as f32 - horizontal_margin, margin_top),
        ];

        if draw_bounds_points {
            let mut bounds_path = Path::new();
            bounds_path.rect(bounds[0].0, bounds[0].1, 1.0, 1.0);
            bounds_path.rect(bounds[1].0, bounds[1].1, 1.0, 1.0);
            self.canvas
                .fill_path(&bounds_path, &Paint::color(Color::rgb(255, 255, 255)));
        }

        let total_line_size = self.psize.width as f32 - horizontal_margin * 2.0;
        let total_in_line = total_line_size / font_dimension.get_complete_width().0;

        for (ci, i) in chars.chunks(total_in_line.floor() as usize).enumerate() {
            for (index, c) in i.iter().enumerate() {
                if !c.is_empty() {
                    let position_x =
                        bounds[0].0 + (font_dimension.get_complete_width().0 * index as f32);
                    let position_y =
                        bounds[0].1 + (font_dimension.get_complete_width().1 * ci as f32);

                    let mut font_mask =
                        FontMask::new(self.app_state.clone(), self.binded_char.get_mut(), c);
                    font_mask.initialize(FontMaskProp {
                        canvas: &mut self.canvas,
                        cp: (position_x, position_y),
                        font_size,
                        padding: Some(padding.clone()),
                        draw_box: None,
                    });
                }
            }
        }
    }
}

use std::{rc::Rc, sync::RwLock};

use femtovg::{Canvas, Color, Paint, Renderer};

use crate::{
    font_engine::font::{ContextPoints, FontFillKind, OrbFont, OrbParts},
    interfaces::app::AppState,
};

pub struct FontMask;

// encontrar uma forma de controlar um estado
// encontrar uma forma de obter as coordenadas do mouse

impl FontMask {
    pub fn initialize<T: Renderer>(
        canvas: &mut Canvas<T>,
        state: Rc<RwLock<AppState>>,
        cp: ContextPoints,
        _bind_char: &'static str,
    ) -> () {
        let state = state.read().expect("Fail to read app state");
        let path_list = OrbFont::init(
            canvas,
            50.0,
            Paint::color(Color::rgb(33, 33, 44)).with_line_width(4.0),
            (cp.0, cp.1),
        )
        .with_box(false)
        .with_parts(vec![
            OrbParts::CircleBase,
            OrbParts::CircleSmallCenter,
            OrbParts::LeftLag,
            OrbParts::RightLag,
            OrbParts::TopLag,
            OrbParts::BottomLag,
            OrbParts::TopAngleLeftLag,
            OrbParts::TopAngleRightLag,
            OrbParts::BottomAngleLeftLag,
            OrbParts::BottomAngleRightLag,
            OrbParts::HalfLeftCircle,
            OrbParts::HalfRightCircle,
        ])
        .draw();

        for (_, mut comp) in path_list.into_iter().enumerate() {
            let color = comp.1.with_color(Color::rgb(255, 255, 255));
            let is_in_path = canvas.contains_point(
                &comp.0,
                state.mouse.x as f32,
                state.mouse.y as f32,
                femtovg::FillRule::NonZero,
            );

            if is_in_path == true {
                match comp.2 {
                    FontFillKind::Stroke => {
                        canvas.stroke_path(&comp.0, &color);
                    }
                    FontFillKind::Path => {
                        canvas.fill_path(&comp.0, &color);
                    }
                    _ => {}
                }
            } else {
                match comp.2 {
                    FontFillKind::Rotate(font) => {
                        font.render(
                            state.mouse.x as f32,
                            state.mouse.y as f32,
                            canvas,
                            &mut comp.0,
                            &color,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

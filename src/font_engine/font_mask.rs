use femtovg::{Canvas, Color, Paint, Renderer};

use crate::{
    font_engine::font::{FontFillKind, OrbFont, OrbParts},
    interfaces::app::{AppStateType, ContextPoints},
};

pub struct FontMask;

impl FontMask {
    pub fn initialize<T: Renderer>(
        canvas: &mut Canvas<T>,
        state: AppStateType,
        cp: ContextPoints,
        font_size: f32,
        padding: Option<f32>,
        _bind_char: &'static str,
    ) -> () {
        let mouse_position = state.mouse.borrow();
        let path_list = OrbFont::init(
            canvas,
            font_size,
            padding,
            Paint::color(Color::rgb(33, 33, 44)),
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
                mouse_position.x as f32,
                mouse_position.y as f32,
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
                            mouse_position.x as f32,
                            mouse_position.y as f32,
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

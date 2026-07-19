use femtovg::{Canvas, Color, Paint, Renderer};

use crate::{
    font_engine::font::{FontFillKind, FontPadding, OrbFont, OrbParts},
    interfaces::app::{AppStateType, ContextPoints},
};

pub struct FontMask {
    pub state: AppStateType
}

pub struct FontMaskProp<'a, T: Renderer> {
    pub canvas: &'a mut Canvas<T>,
    //pub state: AppStateType,
    pub cp: ContextPoints,
    pub font_size: f32,
    pub padding: Option<FontPadding>,
    pub bind_char: &'static str,
    pub draw_box: Option<bool>,
}

impl FontMask {
    pub fn new(state: AppStateType) -> Self {
        Self {
            state
        }
    }

    pub fn initialize<'a, T: Renderer>(&self, props: FontMaskProp<'a, T>) -> () {
        let mouse_position = self.state.mouse.borrow();

        let path_list = OrbFont::init(
            props.canvas,
            props.font_size,
            props.padding,
            Paint::color(Color::rgb(33, 33, 44)),
            (props.cp.0, props.cp.1),
        )
        .with_box(props.draw_box.unwrap_or(false))
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

        let fonts_ids = self.state.font_ids.borrow();
        let text_paint = Paint::color(Color::rgb(50, 50, 69))
            .with_font(&fonts_ids)
            .with_font_italic(false)
            .with_font_weight(500.0);

        props
            .canvas
            .fill_text(
                (props.cp.0) + 10.0,
                (props.cp.1) + 10.0,
                props.bind_char,
                &text_paint,
            )
            .expect("Failed to draw bind char");

        for (_, mut comp) in path_list.into_iter().enumerate() {
            let color = comp.1.with_color(Color::rgb(255, 255, 255));
            let is_in_path = props.canvas.contains_point(
                &comp.0,
                mouse_position.x as f32,
                mouse_position.y as f32,
                femtovg::FillRule::NonZero,
            );

            if is_in_path == true {
                match comp.2 {
                    FontFillKind::Stroke => {
                        props.canvas.stroke_path(&comp.0, &color);
                    }
                    FontFillKind::Path => {
                        props.canvas.fill_path(&comp.0, &color);
                    }
                    _ => {}
                }
            } else {
                match comp.2 {
                    FontFillKind::Rotate(font) => {
                        font.render(
                            mouse_position.x as f32,
                            mouse_position.y as f32,
                            props.canvas,
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

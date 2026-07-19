use std::collections::HashMap;

use femtovg::{Canvas, Color, Paint, Renderer};

use crate::{
    font_engine::font::{FontFillKind, FontPadding, OrbFont, OrbParts},
    interfaces::app::{AppStateType, ContextPoints, MousePosition},
};

pub struct FontMask<'f> {
    pub state: AppStateType,
    pub bind_state: &'f mut HashMap<String, OrbParts>,
    pub bind_char: &'static str,
}

pub struct FontMaskProp<'a, T: Renderer> {
    pub canvas: &'a mut Canvas<T>,
    pub cp: ContextPoints,
    pub font_size: f32,
    pub padding: Option<FontPadding>,
    pub draw_box: Option<bool>,
}

impl<'f> FontMask<'f> {
    pub fn new(
        state: AppStateType,
        bind_state: &'f mut HashMap<String, OrbParts>,
        bind_char: &'static str,
    ) -> Self {
        Self {
            state,
            bind_state,
            bind_char,
        }
    }

    pub fn initialize<'a, T: Renderer>(&mut self, props: FontMaskProp<'a, T>) -> () {
        let mouse_position = {
            let mouse = self.state.mouse.borrow();
            MousePosition {
                x: mouse.x,
                y: mouse.y,
            }
        };

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

        let fonts_ids = {
            let ids = self.state.font_ids.borrow();
            ids.clone()
        };
        let text_paint = Paint::color(Color::rgb(50, 50, 69))
            .with_font(&fonts_ids)
            .with_font_italic(false)
            .with_font_weight(500.0);

        props
            .canvas
            .fill_text(
                (props.cp.0) + 10.0,
                (props.cp.1) + 10.0,
                self.bind_char,
                &text_paint,
            )
            .expect("Failed to draw bind char");

        for (_, comp) in path_list.into_iter().enumerate() {
            let mut path = comp.0;
            let color = path.1.with_color(Color::rgb(255, 255, 255));
            let is_in_path = props.canvas.contains_point(
                &path.0,
                mouse_position.x as f32,
                mouse_position.y as f32,
                femtovg::FillRule::NonZero,
            );

            if is_in_path == true {
                match path.2 {
                    FontFillKind::Stroke => {
                        props.canvas.stroke_path(&path.0, &color);
                        self.handle_click_in(&comp.1);
                    }
                    FontFillKind::Path => {
                        props.canvas.fill_path(&path.0, &color);
                        self.handle_click_in(&comp.1);
                    }
                    _ => {}
                }
            } else {
                match path.2 {
                    FontFillKind::Rotate(font) => {
                        font.render(
                            mouse_position.x as f32,
                            mouse_position.y as f32,
                            props.canvas,
                            &mut path.0,
                            &color,
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn handle_click_in(&mut self, part: &OrbParts) -> () {
        if self.state.had_click() == true {
            self.bind_state
                .insert(self.bind_char.to_owned(), part.clone());
        }
    }
}

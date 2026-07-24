use femtovg::{Canvas, Color, Paint, Renderer};

use crate::{
    font_engine::font::{FontFillKind, FontPadding, OrbFont, OrbParts},
    interfaces::app::{AppStateType, ContextPoints, MousePosition, OrbPath, OrbPathBounds},
};

pub struct FontMask {
    pub state: AppStateType,
    pub bind_char: &'static str,
}

pub struct FontMaskProp<'a, T: Renderer> {
    pub canvas: &'a mut Canvas<T>,
    pub cp: ContextPoints,
    pub font_size: f32,
    pub padding: Option<FontPadding>,
    pub draw_box: Option<bool>,
}

impl FontMask {
    pub fn new(state: AppStateType, bind_char: &'static str) -> Self {
        Self { state, bind_char }
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
            let mut orb_path = comp.0;
            let color = (orb_path.paint.clone()).with_color(Color::rgb(255, 255, 255));

            let is_path_active = self.check_path_active(&comp.1);
            let is_hovered = self.is_hover_path(&mouse_position, &orb_path);

            if (is_hovered || is_path_active) == true {
                match orb_path.font_fill_kind {
                    FontFillKind::Stroke => {
                        props.canvas.stroke_path(&orb_path.path, &color);
                    }
                    FontFillKind::Path => {
                        props.canvas.fill_path(&orb_path.path, &color);
                    }
                    _ => {}
                }

                if is_hovered {
                    self.handle_click_in(&comp.1);
                }
            } else {
                match orb_path.font_fill_kind {
                    FontFillKind::Rotate(font) => {
                        font.render(
                            mouse_position.x as f32,
                            mouse_position.y as f32,
                            props.canvas,
                            &mut orb_path.path,
                            &color,
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn is_hover_path(&self, mpos: &MousePosition, opath: &OrbPath) -> bool {
        let mx = mpos.x as f32;
        let my = mpos.y as f32;

        return match opath.bound {
            OrbPathBounds::Arc(cx, cy, r, s, is_h) => {
                if !is_h {
                    let b_out_l = cx - r;
                    let b_in_l = b_out_l + s;

                    if (mx >= b_out_l && mx <= b_in_l) && (my >= cy - r && my <= cy + r) {
                        return true;
                    }

                    let b_out_r = cx + r;
                    let b_in_r = b_out_r - s;

                    if (mx <= b_out_r && mx >= b_in_r) && (my >= cy - r && my <= cy + r) {
                        return true;
                    }

                    let b_out_t = cy - r;
                    let b_in_t = b_out_t + s;

                    if (my >= b_out_t && my <= b_in_t) && (mx >= cx - r && mx <= cx + r) {
                        return true;
                    }

                    let b_out_b = cy + r;
                    let b_in_b = b_out_b - s;

                    if (my <= b_out_b && my >= b_in_b) && (mx >= cx - r && mx <= cx + r) {
                        return true;
                    }
                }

                let b_out_l = cx - r;
                let b_in_l = b_out_l + s;

                if (mx >= b_out_l && mx <= b_in_l) && (my >= cy - r && my <= cy + r) {
                    return true;
                }

                let b_out_r = cx + r;
                let b_in_r = b_out_r - s;

                if (mx <= b_out_r && mx >= b_in_r) && (my >= cy - r && my <= cy + r) {
                    return true;
                }

                false
            }
            OrbPathBounds::Circle(cx, cy, r) => {
                if (mx >= cx - r && my >= cy - r) && (mx <= cx + r && my <= cy + r) {
                    return true;
                }
                false
            }
            OrbPathBounds::Rect(x, y, w, h) => {
                if (mx >= x && my >= y) && (mx <= x + w && my <= y + h) {
                    return true;
                }
                false
            }
        };
    }

    pub fn check_path_active(&self, part: &OrbParts) -> bool {
        if let Some(storage) = self.state.binded_char.borrow().get(self.bind_char) {
            return storage.contains(part);
        }

        false
    }

    pub fn handle_click_in(&mut self, part: &OrbParts) -> () {
        if self.state.had_click() == true {
            if let Some(storage) = self.state.binded_char.borrow_mut().get_mut(self.bind_char) {
                let part = part.clone();

                if let Some(pos) = storage.iter().position(|d| d == &part) {
                    storage.remove(pos);
                    return;
                }

                storage.push(part.clone());
            } else {
                self.state
                    .binded_char
                    .borrow_mut()
                    .insert(self.bind_char.to_owned(), vec![part.clone()]);
            }
        }
    }
}

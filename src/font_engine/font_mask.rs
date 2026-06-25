use std::{rc::Rc, sync::RwLock};

use femtovg::{Canvas, Color, Paint, Renderer};

use crate::{
    font_engine::font::{ContextPoints, OrbFont, OrbParts},
    interfaces::app::AppState,
};

pub struct FontMask<'a, T: Renderer> {
    bind_char: &'static str,
    app_state: Rc<RwLock<AppState>>,
    canvas: &'a mut Canvas<T>,
    font_instance: OrbFont<'a, T>,
    cp: ContextPoints,
}

// encontrar uma forma de controlar um estado
// encontrar uma forma de obter as coordenadas do mouse

impl<'a, T: Renderer> FontMask<'a, T> {
    pub fn new(
        canvas: &'a mut Canvas<T>,
        state: Rc<RwLock<AppState>>,
        cp: ContextPoints,
        bind_char: &'static str,
    ) -> () {
        let state = state.read().expect("Fail to read app state");
        println!(
            "mouse_position: x: {} - y: {}",
            state.mouse.x, state.mouse.y
        );

        let _ = OrbFont::init(
            canvas,
            100.0,
            Paint::color(Color::rgb(82, 88, 95)).with_line_width(4.0),
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
    }
}

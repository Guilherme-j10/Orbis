use femtovg::{Canvas, Color, Renderer};

use crate::dtos::app::ApplicationStateType;

pub enum ContainerStyle {
    JustifyCenter(Option<(f32, f32)>), // x, w
    AlignCenter(Option<(f32, f32)>),   // y, h
    MarginTop(f32),
    Padding(f32, f32), // horizontal, vertical
    Background(Color),
}

pub struct UIContianer<'a, T: Renderer> {
    pub canvas: &'a mut Canvas<T>,
    pub app_state: ApplicationStateType,
    pub style: Vec<ContainerStyle>,
}

impl<'a, T: Renderer> UIContianer<'a, T> {
    pub fn new(
        canvas: &'a mut Canvas<T>,
        app_state: ApplicationStateType,
        style: Vec<ContainerStyle>,
    ) -> Self {
        Self {
            app_state,
            canvas,
            style,
        }
    }

    pub fn draw(&self) -> () {}
}


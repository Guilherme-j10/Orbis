use femtovg::{Canvas, Renderer};

use crate::{dtos::app::ApplicationStateType, utils::style::UIStyle};

pub struct UIContianer<'a, T: Renderer> {
    pub canvas: &'a mut Canvas<T>,
    pub app_state: ApplicationStateType,
    pub style: Vec<UIStyle>,
}

impl<'a, T: Renderer> UIContianer<'a, T> {
    pub fn new(
        canvas: &'a mut Canvas<T>,
        app_state: ApplicationStateType,
        style: Vec<UIStyle>,
    ) -> Self {
        Self {
            app_state,
            canvas,
            style,
        }
    }

    pub fn draw(&self) -> () {}
}


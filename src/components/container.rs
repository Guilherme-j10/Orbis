use femtovg::{Canvas, Renderer};

use crate::dtos::app::ApplicationStateType;

pub struct UIContianer<'a, T: Renderer> {
    pub canvas: &'a mut Canvas<T>,
    pub app_state: ApplicationStateType,
}

impl<'a, T: Renderer> UIContianer<'a, T> {
    pub fn new(canvas: &'a mut Canvas<T>, app_state: ApplicationStateType) -> Self {
        Self { app_state, canvas }
    }
}
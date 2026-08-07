use femtovg::{Canvas, Renderer};

use crate::dtos::app::{ApplicationScreens, ApplicationStateType, EditorScreenState};

pub struct Editor<'a, T: Renderer> {
    pub canvas: &'a mut Canvas<T>,
    pub app_state: ApplicationStateType,
    pub screen_state: &'a EditorScreenState,
}

impl<'a, T: Renderer> Editor<'a, T> {
    pub fn new(
        canvas: &'a mut Canvas<T>,
        app_state: ApplicationStateType,
        screen_state: &'a EditorScreenState,
    ) -> Self {
        Self {
            canvas,
            app_state,
            screen_state,
        }
    }

    pub fn render(&mut self) -> Option<ApplicationScreens> {
        None
    }
}

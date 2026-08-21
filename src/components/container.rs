use crate::{dtos::app::ApplicationStateType, utils::style::UIStyle};

pub struct UIContianer {
    pub app_state: ApplicationStateType,
    pub style: Vec<UIStyle>,
}

impl UIContianer {
    pub fn new(
        app_state: ApplicationStateType,
        style: Vec<UIStyle>,
    ) -> Self {
        Self {
            app_state,
            style,
        }
    }

    pub fn draw(&self) -> () {}
}


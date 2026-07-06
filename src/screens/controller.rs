use std::{rc::Rc, sync::RwLock};

use femtovg::{Canvas, Renderer};
use winit::dpi::PhysicalSize;

use crate::{
    interfaces::app::{AppScreens, AppState},
    screens::initial::InitialScreen,
};

pub struct Controller;

impl Controller {
    pub fn render<T: Renderer>(
        canvas: &mut Canvas<T>,
        app_state: Rc<RwLock<AppState>>,
        psize: &PhysicalSize<u32>,
    ) -> () {
        let state = app_state.read().expect("Failed to read state");
        match state.current_screen {
            AppScreens::Initial => {
                let mut init = InitialScreen::render(canvas, app_state.clone(), (0.0, 0.0), &psize);
                init.resolve_font_map();
            }
            AppScreens::FontEditor => {}
            AppScreens::OpenedFile => {}
        }
    }
}

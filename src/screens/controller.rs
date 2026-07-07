use femtovg::{Canvas, Renderer};
use winit::dpi::PhysicalSize;

use crate::{
    interfaces::app::{AppScreens, AppStateType},
    screens::initial::InitialScreen,
};

pub struct Controller;

impl Controller {
    pub fn render<T: Renderer>(
        canvas: &mut Canvas<T>,
        app_state: AppStateType,
        psize: &PhysicalSize<u32>,
    ) -> () {
        match app_state.current_screen.get() {
            AppScreens::Initial => {
                let mut init = InitialScreen::render(canvas, app_state.clone(), (0.0, 0.0), &psize);
                init.resolve_font_map();
            }
            AppScreens::FontEditor => {}
            AppScreens::OpenedFile => {}
        }
    }
}

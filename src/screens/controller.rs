use femtovg::{Canvas, Renderer};
use winit::dpi::PhysicalSize;

use crate::{
    interfaces::app::{AppScreens, AppStateType},
    screens::{font_editor::FontEditorScreen, initial::InitialScreen},
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
                let mut init =
                    InitialScreen::initialize(canvas, app_state.clone(), (0.0, 0.0), &psize);
                init.render();
            }
            AppScreens::FontEditor => {
                let mut init =
                    FontEditorScreen::initialize(canvas, app_state.clone(), (0.0, 0.0), &psize);
                init.render();
                
            }
            AppScreens::OpenedFile => {}
        }
    }
}

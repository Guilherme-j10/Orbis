use femtovg::{Canvas, Renderer};
use winit::dpi::PhysicalSize;

use crate::{
    interfaces::app::{ApplicationScreens, ApplicationStateType},
    screens::{font_editor::FontEditorScreen, initial::InitialScreen},
};

pub struct Controller;

impl Controller {
    pub fn render<T: Renderer>(
        canvas: &mut Canvas<T>,
        app_state: ApplicationStateType,
        psize: &PhysicalSize<u32>,
    ) -> () {
        let screen = match &*app_state.current_screen.borrow() {
            ApplicationScreens::Initial => {
                let mut init =
                    InitialScreen::initialize(canvas, app_state.clone(), (0.0, 0.0), &psize);
                init.render()
            }
            ApplicationScreens::FontMapping(state_screen) => {
                let mut init = FontEditorScreen::initialize(
                    canvas,
                    app_state.clone(),
                    state_screen,
                    (0.0, 0.0),
                    &psize,
                );
                init.render()
            }
        };

        if let Some(screen) = screen {
            app_state.change_screen(screen);
        }
    }
}

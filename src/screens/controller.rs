use femtovg::{Canvas, Renderer};

use crate::{
    dtos::app::{ApplicationScreens, ApplicationStateType},
    screens::{editor::Editor, font_editor::FontEditorScreen, initial::InitialScreen},
};

pub struct Controller;

impl Controller {
    pub fn render<T: Renderer>(canvas: &mut Canvas<T>, app_state: ApplicationStateType) -> () {
        let screen = match &*app_state.current_screen.borrow() {
            ApplicationScreens::Initial(state_screen) => {
                let mut init =
                    InitialScreen::initialize(canvas, app_state.clone(), state_screen, (0.0, 0.0));
                init.render()
            }
            ApplicationScreens::FontMapping(state_screen) => {
                let mut init = FontEditorScreen::initialize(
                    canvas,
                    app_state.clone(),
                    state_screen,
                    (0.0, 0.0),
                );
                init.render()
            }
            ApplicationScreens::Editor(editor_state) => {
                let mut editor = Editor::new(canvas, app_state.clone(), editor_state);
                editor.render()
            }
        };

        if let Some(screen) = screen {
            app_state.change_screen(screen);
        }
    }
}

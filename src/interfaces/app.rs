use std::{cell::{Cell, RefCell}, rc::Rc};

use femtovg::FontId;
use winit::event::ElementState;

pub type ContextPoints = (f32, f32);
pub type OrbPartCode = u8;

#[derive(Debug, Default)]
pub struct MousePosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum AppScreens {
    Initial,
    FontEditor,
    OpenedFile,
}

#[derive(Debug)]
pub struct AppState {
    pub mouse: RefCell<MousePosition>,
    pub current_screen: Cell<AppScreens>,
    pub font_ids: RefCell<Vec<FontId>>,
    pub had_click: RefCell<Option<ElementState>>,
}

pub type AppStateType = Rc<AppState>;
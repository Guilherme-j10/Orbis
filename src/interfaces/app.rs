use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use femtovg::{FontId, Paint, Path};
use winit::event::ElementState;

use crate::font_engine::font::{FontFillKind, OrbParts};

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
    pub binded_char: RefCell<HashMap<String, Vec<OrbParts>>>,
}

pub type AppStateType = Rc<AppState>;

impl AppState {
    pub fn had_click(&self) -> bool {
        let mut had_click = self.had_click.borrow_mut();
        if let Some(element_state) = *had_click {
            if element_state == ElementState::Pressed {
                *had_click = None;
                return true;
            }
        }

        return false;
    }
}

pub enum OrbPathBounds {
    Rect(f32, f32, f32, f32),      //x,y - w,h
    Arc(f32, f32, f32, f32, bool), //cx,cy - r - stroke_w - is_half
    Circle(f32, f32, f32),         //cx,cy - r
}

pub struct OrbPath {
    pub path: Path,
    pub paint: Paint,
    pub font_fill_kind: FontFillKind,
    pub bound: OrbPathBounds
}
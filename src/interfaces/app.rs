use std::{cell::RefCell, collections::HashMap, rc::Rc};

use femtovg::{FontId, Paint, Path};
use winit::event::ElementState;

use crate::font_engine::font::{FontFillKind, OrbParts};

pub type ContextPoints = (f32, f32);

#[derive(Debug, Default)]
pub struct MousePosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default)]
pub struct FontMappingState {
    pub binded_char: RefCell<HashMap<String, Vec<OrbParts>>>,
}

#[derive(Debug)]
pub enum ApplicationScreens {
    Initial,
    FontMapping(FontMappingState),
}

#[derive(Debug)]
pub struct HardwareState {
    pub mouse: RefCell<MousePosition>,
    pub hit_click: RefCell<Option<ElementState>>,
}

#[derive(Debug)]
pub struct ApplicationSettingsData {
    pub font_ids: RefCell<Vec<FontId>>,
}

#[derive(Debug)]
pub struct ApplicationState {
    pub hardware: HardwareState,
    pub app_data: ApplicationSettingsData,
    pub current_screen: RefCell<ApplicationScreens>,
}

pub type ApplicationStateType = Rc<ApplicationState>;

impl ApplicationState {
    pub fn change_screen(&self, screen: ApplicationScreens) {
        *self.current_screen.borrow_mut() = screen;
    }

    pub fn had_click(&self) -> bool {
        let mut had_click = self.hardware.hit_click.borrow_mut();
        if let Some(element_state) = *had_click {
            if element_state == ElementState::Pressed {
                *had_click = None;
                return true;
            }
        }

        return false;
    }
}

#[derive(Debug)]
pub enum OrbPathBounds {
    Rect(f32, f32, f32, f32),             //x,y - w,h
    RotatedRect(f32, f32, f32, f32, f32), //x,y - w,h - angle in degrees
    Arc(f32, f32, f32, f32, bool, u8), //cx,cy - r - stroke_w - is_half - side: 1 = left, 2 = right, 0 = none
    Circle(f32, f32, f32),             //cx,cy - r
}

pub struct OrbPath {
    pub path: Path,
    pub paint: Paint,
    pub font_fill_kind: FontFillKind,
    pub bound: OrbPathBounds,
}

pub enum GlihpPatternCheck {
    Available,
    Unavailable,
}

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
    time::SystemTime,
};

use directories::ProjectDirs;
use femtovg::{FontId, Paint, Path};
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::{
    font_engine::font::{FontFillKind, OrbParts},
    utils::notification::{Notification, NotificationKind},
};

pub enum OrbKeyEvent {
    Gliph(Vec<OrbParts>),
    RawKey(KeyCode),
}

pub type EditorLayoutBound = (f32, f32, f32, f32); // x,y - w,h
pub type OrbGliph = OrbKeyEvent;
pub type ContextPoints = (f32, f32);
pub type ApplicationStateType = Rc<ApplicationState>;
pub type MappedFont = HashMap<String, Vec<OrbParts>>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct EditorLayoutData {
    pub bounds: EditorLayoutBound,
    pub path: Path,
    pub label: &'static str,
}

#[derive(Debug, Default)]
pub struct MousePosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default)]
pub struct FontMappingState {
    pub binded_char: RefCell<MappedFont>,
}

#[derive(Debug, Default)]
pub struct InitialScreenState {
    pub font_mapping_verificate: Cell<bool>,
}

#[derive(Debug, Default)]
pub struct EditorScreenState {
    pub last_click_at: Cell<(f32, f32)>, // x,y
}

#[derive(Debug)]
pub enum ApplicationScreens {
    Initial(InitialScreenState),
    FontMapping(FontMappingState),
    Editor(EditorScreenState),
}

#[derive(Debug)]
pub struct HardwareState {
    pub mouse: RefCell<MousePosition>,
    pub hit_click: RefCell<Option<ElementState>>,
}

#[derive(Debug)]
pub struct ApplicationSettingsData {
    pub font_ids: RefCell<Vec<FontId>>,
    pub font_mapping: RefCell<MappedFont>,
    pub project_dirs: ProjectDirs,
    pub receiver_key_event: Receiver<OrbGliph>,
}

#[derive(Debug)]
pub struct ApplicationState {
    pub hardware: HardwareState,
    pub app_data: ApplicationSettingsData,
    pub current_screen: RefCell<ApplicationScreens>,
    pub window: Arc<Window>,
    pub notification_timers: RefCell<Vec<Notification>>,
}

impl ApplicationState {
    pub fn push_notification(&self, kind: NotificationKind) -> () {
        self.notification_timers.borrow_mut().push(Notification {
            kind,
            create_at: SystemTime::now(),
        });
    }

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
    Arc(f32, f32, f32, f32, bool, u8),    //cx,cy - r - stroke_w - is_half - side: 1 = left, 2 = right, 0 = none
    Circle(f32, f32, f32),                //cx,cy - r
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

pub trait SendOrbInfo {
    fn send_font(&self, event: KeyEvent, font_mapping: &MappedFont) -> ();
}

impl SendOrbInfo for Sender<OrbGliph> {
    fn send_font(&self, event: KeyEvent, font_mapping: &MappedFont) -> () {
        let key = || -> Option<OrbKeyEvent> {
            if event.state == ElementState::Pressed && font_mapping.len() > 0 {
                if let Some(character) = event.logical_key.to_text() {
                    if let Some(gliph) = font_mapping.get(character) {
                        return Some(OrbKeyEvent::Gliph(gliph.clone()));
                    }

                    return match event.physical_key {
                        PhysicalKey::Code(code) => Some(OrbKeyEvent::RawKey(code)),
                        _ => None,
                    };
                }
            }
            None
        }();

        if let Some(key_event) = key {
            if let Some(err) = self.send(key_event).err() {
                println!("Failed to transmit keyevent payload: {err}");
            }
        }
    }
}

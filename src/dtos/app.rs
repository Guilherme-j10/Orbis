use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    path::PathBuf,
    rc::Rc,
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
    time::SystemTime,
};

use directories::ProjectDirs;
use femtovg::{Canvas, FontId, Paint, Path, Renderer};
use walkdir::WalkDir;
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorIcon, Window},
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
pub type DirEntryList = Box<dyn Iterator<Item = walkdir::Result<walkdir::DirEntry>>>;

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

pub trait ScreenBoundsCheck {
    // rename to ElementFocusCheck
    fn get_bounds(&self) -> (f32, f32, f32, f32);
    fn get_path(&self) -> Path;
    fn has_focus<T: Renderer>(
        &self,
        coords: (f32, f32),
        highlight: Option<&mut Canvas<T>>,
    ) -> bool {
        let bounds = self.get_bounds();
        let is_on_focus = (coords.0 >= bounds.0 && coords.0 <= bounds.0 + bounds.2)
            && (coords.1 >= bounds.1 && coords.1 <= bounds.1 + bounds.3);

        if is_on_focus && let Some(canvas) = highlight {
            canvas.stroke_path(
                &self.get_path(),
                &Paint::color(femtovg::Color::rgb(255, 0, 0)).with_line_width(1.0),
            );
        }

        is_on_focus
    }
}

#[derive(Debug, Default)]
pub struct EditorScreenState {
    pub last_click_at: Cell<(f32, f32)>, // x,y
    pub aside_files: RefCell<AsideContainerState>,
    pub main_container: RefCell<MainContainerState>,
    pub root_folder: RefCell<RootFolder>,
    pub current_buffer: Cell<u32>,
    pub opened_buffer_list: RefCell<Vec<u32>>,
}

impl EditorScreenState {
    pub fn entries(&self, path: &PathBuf) -> DirEntryList {
        fn is_hidden(entry: &walkdir::DirEntry) -> bool {
            entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        }

        if self.root_folder.borrow().show_hidden_files == false {
            return Box::new(
                WalkDir::new(path)
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .filter_entry(|e| is_hidden(e) == false),
            );
        }

        return Box::new(WalkDir::new(path).into_iter());
    }

    pub fn get_ordened_direntry_list(&self, path: &PathBuf) -> Vec<walkdir::DirEntry> {
        let entries = self.entries(path);

        let mut folder: Vec<walkdir::DirEntry> = Vec::default();
        let mut files: Vec<walkdir::DirEntry> = Vec::default();
        let mut no_meta: Vec<walkdir::DirEntry> = Vec::default();

        for entrie in entries {
            match entrie {
                Ok(dir) => {
                    if let Some(metadata) = dir.metadata().ok() {
                        if metadata.is_dir() {
                            folder.push(dir);
                        } else if metadata.is_file() {
                            files.push(dir);
                        }
                    } else {
                        no_meta.push(dir);
                    }
                }
                Err(_) => {}
            }
        }

        let callback = |f: &walkdir::DirEntry| f.file_name().to_string_lossy().into_owned();

        folder.sort_by_key(callback);
        files.sort_by_key(callback);
        no_meta.sort_by_key(callback);

        let mut final_listage: Vec<walkdir::DirEntry> = Vec::default();

        final_listage.append(&mut folder);
        final_listage.append(&mut files);
        final_listage.append(&mut no_meta);

        return final_listage;
    }
}

#[derive(Debug, Default)]
pub struct RootFolder {
    pub current_folder: Option<PathBuf>,
    pub show_hidden_files: bool,
    pub folder_structure_cache: Vec<walkdir::DirEntry>,
}

#[derive(Debug, Default)]
pub struct MainContainerState {
    pub bounds: EditorLayoutBound,
    pub path: Path,
}

impl ScreenBoundsCheck for MainContainerState {
    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        self.bounds.clone()
    }

    fn get_path(&self) -> Path {
        self.path.clone()
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AsideContainerState {
    pub bounds: EditorLayoutBound,
    pub path: Path,
}

impl ScreenBoundsCheck for AsideContainerState {
    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        self.bounds.clone()
    }

    fn get_path(&self) -> Path {
        self.path.clone()
    }
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
    pub last_cursor_icon: Cell<CursorIcon>,
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
    pub fn change_cursor_icon(&self, request: CursorIcon) -> () {
        if request != self.app_data.last_cursor_icon.get() {
            self.window.set_cursor(request);
            self.app_data.last_cursor_icon.set(request);
        }
    }

    pub fn push_notification(&self, kind: NotificationKind) -> () {
        self.notification_timers.borrow_mut().push(Notification {
            kind,
            create_at: SystemTime::now(),
        });
    }

    pub fn change_screen(&self, screen: ApplicationScreens) {
        *self.current_screen.borrow_mut() = screen;
    }

    pub fn set_had_click(&self, new_state: ElementState) -> () {
        let mut current_state = self.hardware.hit_click.borrow_mut();
        if let Some(state) = *current_state {
            if new_state != state {
                *current_state = Some(new_state);
            }
        } else {
            *current_state = Some(new_state)
        }
    }

    pub fn had_click(&self) -> bool {
        let had_click = self.hardware.hit_click.borrow();
        if let Some(element_state) = *had_click {
            if element_state == ElementState::Pressed {
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

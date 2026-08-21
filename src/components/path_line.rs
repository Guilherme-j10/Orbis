use femtovg::{Canvas, Renderer};
use walkdir::DirEntry;

use crate::dtos::app::{ApplicationStateType, EditorScreenState};

pub struct PathLine<'a> {
    pub self_root_path: DirEntry,
    pub from_depth: usize, // the layer that this came from
    pub app_state: ApplicationStateType,
    pub screen_state: &'a EditorScreenState,
}

impl<'a> PathLine<'a> {
    pub fn new(
        path: DirEntry,
        app_state: ApplicationStateType,
        screen_state: &'a EditorScreenState,
        from_depth: usize
    ) -> Self {
        Self {
            app_state,
            from_depth,
            screen_state,
            self_root_path: path,
        }
    }

    pub fn draw<T: Renderer>(&self, canvas: &mut Canvas<T>) -> () {
        let path_meta = self.self_root_path.metadata().unwrap();
        
        if path_meta.is_file() {

        }
         
        if path_meta.is_dir() {

        }
    }
}

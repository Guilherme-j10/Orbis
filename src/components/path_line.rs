use femtovg::{Canvas, Renderer};
use walkdir::DirEntry;

use crate::{
    components::button::UIButton,
    dtos::app::{ApplicationStateType, EditorScreenState},
    utils::style::UIStyle,
};

pub struct UIPathLine<'a> {
    pub self_root_path: &'a DirEntry,
    pub from_depth: usize, // the layer that this came from
    pub app_state: ApplicationStateType,
    pub screen_state: &'a EditorScreenState,
}

impl<'a> UIPathLine<'a> {
    pub fn new(
        path: &'a DirEntry,
        app_state: ApplicationStateType,
        screen_state: &'a EditorScreenState,
        from_depth: usize,
    ) -> Self {
        // do the cache of this information

        Self {
            app_state,
            from_depth,
            screen_state,
            self_root_path: path,
        }
    }

    pub fn draw<T: Renderer>(&self, canvas: &mut Canvas<T>) -> () {
        let path_meta = self.self_root_path.metadata().unwrap();
        let entity_name = format!("{:?}", self.self_root_path.file_name().display());

        let button = UIButton::new(
            self.app_state.clone(),
            entity_name.as_str(),
            Vec::from([UIStyle::BoundsSize(Some((0.0, 0.0, 0.0, 0.0)))]),
            None,
        );
    }
}

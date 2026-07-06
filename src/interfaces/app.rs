use femtovg::FontId;

#[derive(Debug, Default)]
pub struct MousePosition {
    pub x: f64, 
    pub y: f64
}

#[derive(Debug)]
pub enum AppScreens {
    Initial,
    FontEditor,
    OpenedFile,
}

#[derive(Debug)]
pub struct AppState {
    pub mouse: MousePosition,
    pub current_screen: AppScreens,
    pub font_ids: Vec<FontId>
}
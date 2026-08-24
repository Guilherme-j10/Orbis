use femtovg::{Align, Baseline, Canvas, Paint, Path, Renderer};
use walkdir::DirEntry;
use winit::window::CursorIcon;

use crate::{
    dtos::app::{ApplicationStateType, EditorScreenState},
    utils::style::{ComputedStyle, UIStyle},
};

pub struct UIPathLine<'a> {
    root: &'a DirEntry,
    from_depth: usize, // the origin layer
    app_state: ApplicationStateType,
    screen_state: &'a EditorScreenState,
    path: Path,
    on_click: Box<dyn Fn() -> () + 'a>,
    style: Vec<UIStyle>,
}

impl<'a> UIPathLine<'a> {
    pub fn new(
        root: &'a DirEntry,
        app_state: ApplicationStateType,
        screen_state: &'a EditorScreenState,
        from_depth: usize,
        style: Vec<UIStyle>,
        on_click: Box<dyn Fn() -> () + 'a>,
    ) -> Self {
        Self {
            root,
            style,
            on_click,
            app_state,
            from_depth,
            screen_state,
            path: Path::new(),
        }
    }

    pub fn draw<T: Renderer>(&mut self, canvas: &mut Canvas<T>) -> () {
        //let path_meta = self.root.metadata().unwrap();
        let root_file_name = self.root.file_name().to_string_lossy().into_owned();
        let style = ComputedStyle::from(&self.style);

        let text_paint = Paint::color(style.text_color)
            .with_font(&style.fonts)
            .with_font_size(style.text_size)
            .with_font_italic(false)
            .with_font_weight(500.0)
            .with_text_align(Align::Center)
            .with_text_baseline(Baseline::Middle);

        let bounds = style
            .bounds_size
            .expect("Bounds necessary to this ui component");

        let mut x = bounds.0;
        let mut y = bounds.1 + style.margin_top;

        let width = bounds.0 + bounds.2;
        let height = style.text_size;

        if self.is_mouse_over(x, y, width, height) {
            self.app_state.change_cursor_icon(CursorIcon::Pointer);
        } else {
            self.app_state.change_cursor_icon(CursorIcon::Default);
        }

        self.button_clicked(x, y, width, height);
        self.path.rect(x, y, width, height);

        canvas.fill_path(&self.path, &Paint::color(style.background));
        canvas
            .fill_text(
                x + width / 2.0,
                y + height / 2.0,
                &root_file_name,
                &text_paint,
            )
            .expect("Failed to fill button text");
    }

    fn is_mouse_over(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let mouse = self.app_state.hardware.mouse.borrow();
        let mx = mouse.x as f32;
        let my = mouse.y as f32;

        mx >= x && my >= y && mx <= x + w && my <= y + h
    }

    fn button_clicked(&self, x: f32, y: f32, w: f32, h: f32) -> () {
        if self.app_state.had_click() && self.is_mouse_over(x, y, w, h) {
            (self.on_click)();
            self.app_state.change_cursor_icon(CursorIcon::Default);
        }
    }
}

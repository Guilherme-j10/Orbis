use femtovg::{Align, Baseline, Canvas, Color, Paint, Path, Renderer};
use walkdir::DirEntry;
use winit::window::CursorIcon;

use crate::{
    dtos::app::{ApplicationStateType, EditorScreenState},
    utils::{
        constants::{FILE_ICON, FOLDER_CLOSE_ICON},
        style::{ComputedStyle, UIStyle},
        svg::{CustomSize, Position, draw_svg},
    },
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
        let path_meta = self.root.metadata().unwrap();
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

        let x = bounds.0;
        let y = bounds.1 + style.margin_top;
        let mut container_paint = Paint::color(Color::rgb(26, 26, 36));
        let width = bounds.0 + bounds.2;
        let height = style.text_size + style.padding_detail.1 + style.padding_detail.3;

        if self.is_mouse_over(x, y, width, height) {
            self.app_state.change_cursor_icon(CursorIcon::Pointer);
            container_paint = Paint::color(Color::rgb(46, 46, 61))
        }

        self.button_clicked(x, y, width, height);
        self.path.rect(x, y, width, height);

        let text_width = canvas
            .measure_text(0.0, 0.0, &root_file_name, &text_paint)
            .expect("Failed to measure button text")
            .width();

        let gap = 5.0;
        let svg_icon_size = CustomSize {
            scale_x: 13.0,
            scale_y: 13.0,
        };

        canvas.fill_path(&self.path, &container_paint);
        canvas
            .fill_text(
                x + text_width / 2.0 + style.padding_detail.0 + svg_icon_size.scale_x + gap,
                y + height / 2.0,
                &root_file_name,
                &text_paint,
            )
            .expect("Failed to fill button text");

        let icon = || -> &[u8] {
            if path_meta.is_dir() {
                return FOLDER_CLOSE_ICON;
            }

            return FILE_ICON;
        }();

        draw_svg(
            canvas,
            icon,
            Position {
                x: x + style.padding_detail.0 + svg_icon_size.scale_x / 2.0
                    - (svg_icon_size.scale_x / 2.0),
                y: y + style.text_size - (svg_icon_size.scale_y / 2.0),
            },
            Some(Color::rgb(255, 255, 255)),
            Some(svg_icon_size),
        );
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
        }
    }
}

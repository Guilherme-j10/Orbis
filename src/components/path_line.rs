use std::path::PathBuf;

use femtovg::{Align, Baseline, Canvas, Color, Paint, Path, Renderer};
use walkdir::DirEntry;
use winit::window::CursorIcon;

use crate::{
    dtos::app::{ApplicationStateType, EditorScreenState, OrganizeListKind},
    utils::{
        constants::{FILE_ICON, FOLDER_CLOSE_ICON, FOLDER_OPEN_ICON},
        style::{ComputedStyle, UIStyle},
        svg::{CustomSize, Position, draw_svg},
    },
};

pub struct UIPathLine<'a> {
    root: &'a DirEntry,
    index: usize,
    root_metadata: std::fs::Metadata,
    app_state: ApplicationStateType,
    screen_state: &'a EditorScreenState,
    path: Path,
    style: Vec<UIStyle>,
}

impl<'a> UIPathLine<'a> {
    pub fn new(
        root: &'a DirEntry,
        index: usize,
        app_state: ApplicationStateType,
        screen_state: &'a EditorScreenState,
        style: Vec<UIStyle>,
    ) -> Self {
        Self {
            root,
            style,
            index,
            app_state,
            screen_state,
            path: Path::new(),
            root_metadata: root.metadata().unwrap(),
        }
    }

    pub fn draw<T: Renderer>(&mut self, canvas: &mut Canvas<T>) -> () {
        let mut root_file_name = self.root.file_name().to_string_lossy().into_owned();
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
        let width = bounds.2;
        let height = style.text_size + style.padding_detail.1 + style.padding_detail.3;

        if self.is_mouse_over(x, y, width, height) {
            self.app_state.change_cursor_icon(CursorIcon::Pointer);
            container_paint = Paint::color(Color::rgb(46, 46, 61))
        }

        self.button_clicked(x, y, width, height);
        self.path.rect(x, y, width, height);

        let gap = 5.0;
        let svg_icon_size = CustomSize {
            scale_x: 13.0,
            scale_y: 13.0,
        };

        canvas.fill_path(&self.path, &container_paint);

        let mut truncated = false;
        let content_start = x + style.padding_detail.0 + svg_icon_size.scale_x + gap;
        let available_width = bounds.2
            - style.padding_detail.0
            - svg_icon_size.scale_x
            - gap
            - style.padding_detail.2
            - style.text_size;

        let text_mensure = |text: &str| -> f32 {
            canvas
                .measure_text(0.0, 0.0, text, &text_paint)
                .expect("Failed to measure text")
                .width()
        };

        let mut text_width = text_mensure(&root_file_name);

        while text_width > available_width && !root_file_name.is_empty() {
            truncated = true;
            root_file_name.pop();
            text_width = text_mensure(&root_file_name);
        }

        if truncated {
            root_file_name.push_str("...");
            text_width = text_mensure(&root_file_name);
        }

        canvas
            .fill_text(
                content_start + text_width / 2.0,
                y + height / 2.0,
                &root_file_name,
                &text_paint,
            )
            .expect("Failed to fill button text");

        let icon = || -> &[u8] {
            if self.root_metadata.is_dir() {
                if self
                    .screen_state
                    .root_folder
                    .borrow()
                    .path_open
                    .contains(&self.root.clone().into_path())
                {
                    return FOLDER_OPEN_ICON;
                }

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
        if self.is_mouse_over(x, y, w, h) {
            if self.app_state.had_click() {
                if self.root_metadata.is_dir() {
                    let target_depth = self.root.depth() + 1;

                    let filtered = |cb: Box<dyn Fn(&&DirEntry) -> bool>| -> Vec<DirEntry> {
                        let store_data = self.screen_state.root_folder.borrow();
                        store_data
                            .path_store
                            .iter()
                            .filter(cb)
                            .filter(|p| p.path().starts_with(self.root.path()))
                            .map(|f| f.clone())
                            .collect::<Vec<DirEntry>>()
                    };

                    let list_in_current_target =
                        filtered(Box::new(|dir| dir.depth() == target_depth));
                    let list_ahead_or_in_same_target =
                        filtered(Box::new(|dir| dir.depth() >= target_depth));

                    let cindex = self.index + 1;
                    let pathbuf = self.root.clone().into_path();
                    let mut root_path = self.screen_state.root_folder.borrow_mut();

                    if !root_path.path_open.contains(&self.root.clone().into_path()) {
                        let output = EditorScreenState::organize_list(
                            OrganizeListKind::Raw(list_in_current_target),
                            Some(target_depth),
                            None,
                        );

                        root_path.path_open.push(pathbuf);
                        root_path.path_cache_list.splice(cindex..cindex, output);

                        return;
                    }

                    let subdir_list = root_path
                        .path_open
                        .iter()
                        .filter(|p| p.starts_with(&pathbuf))
                        .map(|p| p.clone())
                        .collect::<Vec<PathBuf>>();

                    root_path.path_open.retain(|f| !subdir_list.contains(&f));

                    let path_list = list_ahead_or_in_same_target
                        .iter()
                        .map(|f| f.path())
                        .collect::<Vec<&std::path::Path>>();
                    root_path
                        .path_cache_list
                        .retain(|dir| !path_list.contains(&dir.path()));
                }

                // is file
            }
        }
    }
}

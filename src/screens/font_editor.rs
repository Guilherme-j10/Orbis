use std::{cell::Cell, rc::Rc};

use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::dpi::PhysicalSize;

use crate::{
    components::button::UIButton,
    core::settings::Settings,
    dtos::app::{
        ApplicationScreens, ApplicationStateType, EditorScreenState, FontMappingState,
        GlihpPatternCheck,
    },
    font_engine::{
        dimensions::FontDimension,
        font::FontPadding,
        font_mask::{FontMask, FontMaskProp},
    },
    utils::{notification::NotificationKind, style::UIStyle},
};

pub struct FontEditorScreen<'a, T: Renderer> {
    canvas: &'a mut Canvas<T>,
    app_state: ApplicationStateType,
    state_screen: &'a FontMappingState,
    _bounds: (f32, f32),
    psize: PhysicalSize<u32>,
}

impl<'a, T: Renderer> FontEditorScreen<'a, T> {
    pub fn initialize(
        canvas: &'a mut Canvas<T>,
        app_state: ApplicationStateType,
        state_screen: &'a FontMappingState,
        bounds: (f32, f32),
    ) -> Self {
        let mut screen = Path::new();
        let psize = app_state.window.inner_size();
        screen.rect(
            bounds.0,
            bounds.1,
            psize.width as f32 - bounds.0,
            psize.height as f32 - bounds.1,
        );

        canvas.fill_path(&screen, &Paint::color(Color::rgb(10, 10, 14)));

        Self {
            canvas,
            app_state,
            state_screen,
            _bounds: bounds,
            psize,
        }
    }

    pub fn render(&mut self) -> Option<ApplicationScreens> {
        let draw_bounds_points = false;
        let horizontal_margin = 270.0;
        let margin_top = 100.0;
        let font_size = 50.0;
        let padding = FontPadding {
            horizontal: 20.0,
            vertical: 5.5,
        };

        let font_dimension = FontDimension::new(&font_size, &padding);

        let chars: Vec<&str> = "abcdefghijklmnopqrstuvwxyz0123456789"
            .trim()
            .split("")
            .filter(|f| !f.is_empty())
            .collect();

        let bounds: [(f32, f32); 2] = [
            (horizontal_margin, margin_top), // x, y
            (self.psize.width as f32 - horizontal_margin, margin_top),
        ];

        if draw_bounds_points {
            let mut bounds_path = Path::new();
            bounds_path.rect(bounds[0].0, bounds[0].1, 1.0, 1.0);
            bounds_path.rect(bounds[1].0, bounds[1].1, 1.0, 1.0);
            self.canvas
                .fill_path(&bounds_path, &Paint::color(Color::rgb(255, 255, 255)));
        }

        let total_line_size = self.psize.width as f32 - horizontal_margin * 2.0;
        let total_in_line = total_line_size / font_dimension.get_complete_width().0;

        for (ci, i) in chars.chunks(total_in_line.floor() as usize).enumerate() {
            for (index, c) in i.iter().enumerate() {
                if !c.is_empty() {
                    let position_x =
                        bounds[0].0 + (font_dimension.get_complete_width().0 * index as f32);
                    let position_y =
                        bounds[0].1 + (font_dimension.get_complete_width().1 * ci as f32);

                    let mut font_mask = FontMask::new(self.app_state.clone(), self.state_screen, c);
                    font_mask.initialize(FontMaskProp {
                        canvas: &mut self.canvas,
                        cp: (position_x, position_y),
                        font_size,
                        padding: Some(padding.clone()),
                        draw_box: None,
                    });
                }
            }
        }

        let font_mapping_file_was_save = Rc::new(Cell::new(false));
        if self.state_screen.binded_char.borrow().len() == chars.len() {
            for bind in chars.into_iter() {
                match FontMask::check_pattern(bind, &self.state_screen) {
                    GlihpPatternCheck::Unavailable => {
                        return None;
                    }
                    _ => {}
                }
            }

            let mut button = UIButton::new(
                self.app_state.clone(),
                "Save font mapping",
                vec![
                    UIStyle::JustifyCenter(None),
                    UIStyle::AlignCenter(None),
                    UIStyle::Padding(20.0, 10.0),
                    UIStyle::Background(Color::rgb(32, 32, 42)),
                    UIStyle::MarginTop(120.0),
                    UIStyle::TextColor(Color::rgb(255, 255, 255)),
                    UIStyle::TextSize(15.0),
                    UIStyle::Font(vec![
                        self.app_state
                            .app_data
                            .font_ids
                            .borrow()
                            .first()
                            .unwrap()
                            .clone(),
                    ]),
                ],
                Some(Box::new(|| {
                    let settings = Settings::new(&self.app_state.app_data.project_dirs);
                    match settings.save_map_file(&self.state_screen) {
                        Ok(save_local) => {
                            self.app_state
                                .push_notification(NotificationKind::Info(format!(
                                    "font mapping save at: {save_local}"
                                )));
                            font_mapping_file_was_save.set(true);
                        }
                        Err(e) => self
                            .app_state
                            .push_notification(NotificationKind::Error(e.to_string())),
                    }
                })),
            );

            button.draw(self.canvas);
        }

        if font_mapping_file_was_save.get() == true {
            return Some(ApplicationScreens::Editor(EditorScreenState::default()));
        }

        None
    }
}

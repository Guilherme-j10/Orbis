use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::keyboard::KeyCode;

use crate::dtos::app::{
    ApplicationScreens, ApplicationStateType, EditorLayoutData, EditorScreenState, OrbKeyEvent,
};

#[allow(dead_code)]
pub struct Editor<'a, T: Renderer> {
    pub canvas: &'a mut Canvas<T>,
    pub app_state: ApplicationStateType,
    pub screen_state: &'a EditorScreenState,
    pub laylou_list: Vec<EditorLayoutData>,
}

impl<'a, T: Renderer> Editor<'a, T> {
    pub fn new(
        canvas: &'a mut Canvas<T>,
        app_state: ApplicationStateType,
        screen_state: &'a EditorScreenState,
    ) -> Self {
        let window_size = app_state.window.inner_size();

        let bounds_side_file = (0.0, 0.0, 250.0, window_size.height as f32);
        let bounds_main_container = (
            bounds_side_file.2,
            0.0,
            window_size.width as f32 - bounds_side_file.2,
            window_size.height as f32,
        );

        let mut side_file_path = Path::new();
        side_file_path.rect(
            bounds_side_file.0,
            bounds_side_file.1,
            bounds_side_file.2,
            bounds_side_file.3,
        );

        let mut main_container_path = Path::new();
        main_container_path.rect(
            bounds_main_container.0,
            bounds_main_container.1,
            bounds_main_container.2,
            bounds_main_container.3,
        );

        canvas.fill_path(&main_container_path, &Paint::color(Color::rgb(18, 18, 26)));
        canvas.fill_path(&side_file_path, &Paint::color(Color::rgb(26, 26, 36)));

        if app_state.had_click() {
            let mouse_data = app_state.hardware.mouse.borrow();
            screen_state
                .last_click_at
                .set((mouse_data.x as f32, mouse_data.y as f32));
        }

        Self {
            canvas,
            app_state,
            screen_state,
            laylou_list: vec![
                EditorLayoutData {
                    label: "file_container",
                    bounds: bounds_side_file,
                    path: side_file_path,
                },
                EditorLayoutData {
                    label: "main_container",
                    bounds: bounds_main_container,
                    path: main_container_path,
                },
            ],
        }
    }

    pub fn _has_focus_on(&self, label: &'static str) -> (bool, Path) {
        let container = self.laylou_list.iter().find(|l| l.label == label).unwrap();
        let coords = self.screen_state.last_click_at.get();
        let bounds = container.bounds;

        (
            (coords.0 >= bounds.0 && coords.0 <= bounds.0 + bounds.2)
                && (coords.1 >= bounds.1 && coords.1 <= bounds.1 + bounds.3),
            container.path.clone(),
        )
    }

    pub fn _highlight_focus(&mut self) -> () {
        let has_focus = self._has_focus_on("main_container");
        if has_focus.0 {
            self.canvas.stroke_path(
                &has_focus.1,
                &Paint::color(Color::rgb(255, 0, 0)).with_line_width(1.0),
            );
        }
    }

    pub fn render(&mut self) -> Option<ApplicationScreens> {
        if let Ok(pressed_gliph) = self.app_state.app_data.receiver_key_event.try_recv() {
            match pressed_gliph {
                OrbKeyEvent::Gliph(gliph) => {
                    if self._has_focus_on("main_container").0 {
                        println!("{:?}", gliph)
                    }
                }
                OrbKeyEvent::RawKey(key) => match key {
                    KeyCode::Escape => self.screen_state.last_click_at.set((0.0, 0.0)),
                    _ => {}
                },
            }
        }

        None
    }
}

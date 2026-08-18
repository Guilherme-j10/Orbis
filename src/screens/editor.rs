use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::keyboard::KeyCode;

use crate::{
    components::circle::{CircleStyle, UICircleContainer},
    dtos::app::{
        ApplicationScreens, ApplicationStateType, EditorScreenState, OrbKeyEvent, ScreenBoundsCheck,
    },
};

#[allow(dead_code)]
pub struct Editor<'a, T: Renderer> {
    pub canvas: &'a mut Canvas<T>,
    pub app_state: ApplicationStateType,
    pub screen_state: &'a EditorScreenState,
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

        let mut aside = screen_state.aside_files.borrow_mut();
        aside.bounds = bounds_side_file;
        aside.path = side_file_path;

        let mut main = screen_state.main_container.borrow_mut();
        main.bounds = bounds_main_container;
        main.path = main_container_path;

        Self {
            canvas,
            app_state,
            screen_state,
        }
    }

    pub fn render(&mut self) -> Option<ApplicationScreens> {
        self.handle_aside_files();
        self.handle_main_container();
        None
    }

    pub fn handle_aside_files(&mut self) -> () {
        if self.screen_state.current_folder.borrow().is_none() {
            let container = self.screen_state.aside_files.borrow().bounds;

            let mut circle = UICircleContainer::new(
                self.app_state.clone(),
                self.canvas,
                vec![
                    CircleStyle::Background(Color::rgb(43, 44, 54)),
                    CircleStyle::AlignCenter(Some((container.1, container.3))),
                    CircleStyle::JustifyCenter(Some((container.0, container.2))),
                    CircleStyle::Radius(25.0),
                ],
            );

            circle.draw();
            return;
        }
    }

    pub fn handle_main_container(&self) -> () {
        if let Ok(pressed_gliph) = self.app_state.app_data.receiver_key_event.try_recv() {
            match pressed_gliph {
                OrbKeyEvent::Gliph(gliph) => {
                    if self
                        .screen_state
                        .main_container
                        .borrow()
                        .has_focus::<T>(self.screen_state.last_click_at.get(), None)
                    {
                        println!("{:?}", gliph)
                    }
                }
                OrbKeyEvent::RawKey(key) => match key {
                    KeyCode::Escape => self.screen_state.last_click_at.set((0.0, 0.0)),
                    KeyCode::Enter => todo!(),
                    KeyCode::Space => todo!(),
                    _ => {}
                },
            }
        }
    }
}

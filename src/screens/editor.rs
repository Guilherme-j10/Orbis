use femtovg::{Align, Canvas, Color, Paint, Path, Renderer};
use winit::keyboard::KeyCode;

use crate::{
    components::{
        button::UIButton, circle::UICircleContainer, path_line::UIPathLine, text::UIText,
    },
    dtos::app::{
        ApplicationScreens, ApplicationStateType, EditorScreenState, OrbKeyEvent, ScreenBoundsCheck,
    },
    utils::{
        constants::FOLDER_CLOSE_ICON,
        style::UIStyle,
        svg::{CustomSize, Position, draw_svg},
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
        if self
            .screen_state
            .root_folder
            .borrow()
            .current_folder
            .is_none()
        {
            let container = self.screen_state.aside_files.borrow().bounds;

            let mut circle = UICircleContainer::new(
                self.app_state.clone(),
                vec![
                    UIStyle::Background(Color::rgb(43, 44, 54)),
                    UIStyle::AlignCenter(Some((container.1, container.3))),
                    UIStyle::JustifyCenter(Some((container.0, container.2))),
                    UIStyle::Radius(25.0),
                ],
            );

            let mut button = UIButton::new(
                self.app_state.clone(),
                "Select folder",
                vec![
                    UIStyle::AlignCenter(Some((container.1, container.3))),
                    UIStyle::JustifyCenter(Some((container.0, container.2))),
                    UIStyle::Padding(20.0, 10.0),
                    UIStyle::Background(Color::rgb(62, 63, 74)),
                    UIStyle::MarginTop(60.0),
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
                    if let Some(path) = rfd::FileDialog::new().set_directory("/").pick_folder() {
                        self.screen_state.root_folder.borrow_mut().current_folder = Some(path);
                    }
                })),
            );

            circle.draw(self.canvas);
            button.draw(self.canvas);

            let svg_icon_size = CustomSize {
                scale_x: 20.0,
                scale_y: 20.0,
            };

            draw_svg(
                self.canvas,
                FOLDER_CLOSE_ICON,
                Position {
                    x: container.0 + container.2 / 2.0 - (svg_icon_size.scale_x / 2.0),
                    y: container.1 + container.3 / 2.0 - (svg_icon_size.scale_y / 2.0),
                },
                Some(Color::rgb(255, 255, 255)),
                Some(svg_icon_size),
            );
            return;
        }

        let (cache, current_folder) = {
            let data = self.screen_state.root_folder.borrow();
            (
                data.folder_structure_cache.clone(),
                data.current_folder.clone(),
            )
        };

        if cache.len() == 0 {
            let machine_path = current_folder.as_ref().unwrap();
            let entries = self.screen_state.get_ordened_direntry_list(machine_path);
            let mut root_folder = self.screen_state.root_folder.borrow_mut();

            root_folder.folder_structure_cache = entries;
        }

        let container = self.screen_state.aside_files.borrow().bounds;
        let font_ids = self.app_state.app_data.font_ids.borrow();

        let text = UIText::new(
            current_folder
                .as_ref()
                .and_then(|f| f.file_name())
                .and_then(|f| Some(f.to_string_lossy()))
                .map(String::from)
                .unwrap()
                .to_uppercase(),
            vec![
                UIStyle::TextAlign(Some(Align::Left)),
                UIStyle::BoundsSize(Some(container)),
                UIStyle::Font(font_ids.to_owned()),
                UIStyle::TextColor(Color::rgb(155, 160, 174)),
                UIStyle::TextSize(15.0),
                UIStyle::Padding(10.0, 0.0),
            ],
        );

        text.draw(self.canvas);

        for (index, cpath) in cache.iter().enumerate() {
            let depth = cpath.depth();
            let padding_vertical = 7.0;

            let mut path_line = UIPathLine::new(
                cpath,
                self.app_state.clone(),
                Vec::from([
                    UIStyle::BoundsSize(Some((
                        container.0,
                        container.1 + 30.0, // title font_size * 2
                        container.2,
                        container.3,
                    ))),
                    UIStyle::Font(font_ids.to_owned()),
                    UIStyle::TextColor(Color::rgb(255, 255, 255)),
                    UIStyle::TextSize(13.0),
                    UIStyle::MarginTop(index as f32 * (13.0 + padding_vertical * 2.0)),
                    UIStyle::PaddingDetail(depth as f32 * 10.0, padding_vertical, 0.0, padding_vertical),
                ]),
                Box::new(|| {
                    println!("Clicked on me");
                }),
            );

            path_line.draw(self.canvas);
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

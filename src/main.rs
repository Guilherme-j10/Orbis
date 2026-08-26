use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    path::PathBuf,
    rc::Rc,
    sync::{
        Arc,
        mpsc::{self, Receiver, Sender},
    }, time::SystemTime,
};

use directories::ProjectDirs;
use femtovg::{Canvas, Color};
use winit::{event::WindowEvent, window::Window};

use crate::{
    dtos::app::{
        ApplicationScreens, ApplicationSettingsData, ApplicationState, HardwareState,
        InitialScreenState, MousePosition, OrbGliph, SendOrbInfo,
    },
    screens::controller::Controller,
    utils::notification::NotificationRender,
    wgpu::{Callbacks, WindowSurface},
};

mod components;
mod core;
mod dtos;
mod font_engine;
mod screens;
mod utils;
mod wgpu;

fn main() {
    let (tx, rx) = mpsc::channel::<OrbGliph>();
    wgpu::start_wgpu(1440, 900, "Orbis", true, (tx, rx));
}

fn run<W: WindowSurface + 'static>(
    mut canvas: Canvas<W::Renderer>,
    mut surface: W,
    window: Arc<Window>,
    key_event_channel: (Sender<OrbGliph>, Option<Receiver<OrbGliph>>),
) -> Callbacks {
    let app_state = Rc::new(ApplicationState {
        hardware: HardwareState {
            mouse: RefCell::new(MousePosition::default()),
            hit_click: RefCell::new(None),
        },
        app_data: ApplicationSettingsData {
            font_ids: RefCell::new(vec![]),
            last_cursor_icon: Cell::default(),
            last_cursor_touch_at: Cell::new(SystemTime::now()),
            font_mapping: RefCell::new(HashMap::default()),
            receiver_key_event: key_event_channel.1.unwrap(),
            project_dirs: ProjectDirs::from("com.orbis", "orbis", "orbis")
                .expect("failed to get project dirs"),
        },
        current_screen: RefCell::new(ApplicationScreens::Initial(InitialScreenState::default())),
        window: window.clone(),
        notification_timers: RefCell::new(Vec::default()),
    });

    let mut fonts_ids = app_state.app_data.font_ids.borrow_mut();
    let font_path = PathBuf::from("font/Saira");
    match font_path.canonicalize() {
        Ok(path) => {
            *fonts_ids = canvas.add_font_dir(path).expect("failed to load font");
        }
        Err(e) => {
            panic!("font path dont found {e}");
        }
    }

    let state = app_state.clone();

    Callbacks {
        window_event: Box::new(move |event, event_loop| match event {
            WindowEvent::Resized(physical_size) => {
                surface.resize(physical_size.width, physical_size.height);
            }
            WindowEvent::RedrawRequested => {
                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                canvas.set_size(size.width, size.height, dpi_factor as f32);
                canvas.clear_rect(0, 0, size.width, size.height, Color::rgb(10, 10, 14));

                Controller::render(&mut canvas, state.clone());

                let mut notification = NotificationRender::new(&mut canvas, state.clone());
                
                state.check_cursor_timer();
                notification.render_loop();

                surface.present(&mut canvas);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state: mouse_state,
                button: _,
            } => {
                state.set_had_click(mouse_state);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                key_event_channel
                    .0
                    .send_font(event, &*state.app_data.font_mapping.borrow());
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                let mut mpos = state.hardware.mouse.borrow_mut();
                mpos.x = position.x;
                mpos.y = position.y
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }),
        device_event: None,
    }
}

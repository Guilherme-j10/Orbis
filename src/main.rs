use std::{cell::{Cell, RefCell}, path::PathBuf, rc::Rc, sync::Arc};

use femtovg::{Canvas, Color};
use winit::{event::WindowEvent, window::Window};

use crate::{
    interfaces::app::{AppScreens, AppState, MousePosition},
    screens::controller::Controller,
    wgpu::{Callbacks, WindowSurface},
};

mod core;
mod font_engine;
mod interfaces;
mod screens;
mod utils;
mod wgpu;

fn main() {
    wgpu::start_wgpu(1440, 900, "Orbis", true);
}

fn run<W: WindowSurface + 'static>(
    mut canvas: Canvas<W::Renderer>,
    mut surface: W,
    window: Arc<Window>,
) -> Callbacks {
    let app_state = Rc::new(AppState {
        mouse: RefCell::new(MousePosition::default()),
        current_screen: Cell::new(AppScreens::Initial),
        font_ids: RefCell::new(vec![]),
        had_click: RefCell::new(None),
    });

    let mut fonts_ids = app_state.font_ids.borrow_mut();
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
                canvas.clear_rect(0, 0, size.width, size.height, Color::rgb(0, 0, 0));

                let state_wrapper = state.clone();
                Controller::render(&mut canvas, state_wrapper, &size);

                surface.present(&mut canvas);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state: mouse_state,
                button: _,
            } => {
                let mut had_click = state.had_click.borrow_mut();
                *had_click = Some(mouse_state);
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                let mut mpos = state.mouse.borrow_mut();
                mpos.x = position.x;
                mpos.y = position.y
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }),
        device_event: None,
    }
}

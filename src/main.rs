use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::{
    font_engine::font_mask::FontMask,
    interfaces::app::{AppState, MousePosition},
    wgpu::{Callbacks, WindowSurface},
};

const ASIDE_MENU_WIDTH: f32 = 300.0;

mod font_engine;
mod interfaces;
mod wgpu;
mod utils;

fn main() {
    wgpu::start_wgpu(1440, 900, "Orbis", false);
}

fn file_system_container<T: Renderer>(canvas: &mut Canvas<T>, size: &PhysicalSize<u32>) -> () {
    let mut main_container = Path::new();
    main_container.rect(0.0, 0.0, ASIDE_MENU_WIDTH, size.height as f32);
    canvas.fill_path(&main_container, &Paint::color(Color::rgb(16, 16, 21)));
}

fn font_editor<T: Renderer>(
    canvas: &mut Canvas<T>,
    size: &PhysicalSize<u32>,
    state: Rc<RwLock<AppState>>,
) -> () {
    let bounds = (ASIDE_MENU_WIDTH, 0.0);

    let mut main_container = Path::new();
    main_container.rect(ASIDE_MENU_WIDTH, 0.0, size.width as f32, size.height as f32);
    canvas.fill_path(&main_container, &Paint::color(Color::rgb(10, 10, 14)));

    draw_mask(50.0, canvas, state.clone(), bounds.0 + 20.0, bounds.1 + 20.0);
    draw_mask(40.0,canvas, state.clone(), bounds.0 + 120.0, bounds.1 + 20.0);
    draw_mask(30.0,canvas, state.clone(), bounds.0 + 240.0, bounds.1 + 20.0);
    draw_mask(20.0,canvas, state.clone(), bounds.0 + 360.0, bounds.1 + 20.0);
    draw_mask(12.0,canvas, state.clone(), bounds.0 + 420.0, bounds.1 + 20.0);
}

fn draw_mask<T: Renderer>(
    font_size: f32,
    canvas: &mut Canvas<T>,
    state: Rc<RwLock<AppState>>,
    cx: f32,
    cy: f32,
) -> () {
    FontMask::initialize(canvas, state, (cx, cy), font_size, "a");
}

fn run<W: WindowSurface + 'static>(
    mut canvas: Canvas<W::Renderer>,
    mut surface: W,
    window: Arc<Window>,
) -> Callbacks {
    let app_state = Rc::new(RwLock::new(AppState {
        mouse: MousePosition::default(),
    }));

    Callbacks {
        window_event: Box::new(move |event, event_loop| match event {
            WindowEvent::Resized(physical_size) => {
                surface.resize(physical_size.width, physical_size.height);
            }
            WindowEvent::RedrawRequested => {
                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                canvas.set_size(size.width, size.height, dpi_factor as f32);
                canvas.clear_rect(0, 0, size.width, size.height, Color::rgb(40, 43, 51));

                let state_wrapper = app_state.clone();

                file_system_container(&mut canvas, &size);
                font_editor(&mut canvas, &size, state_wrapper.clone());

                surface.present(&mut canvas);
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                let state_wrapper = app_state.clone();
                let mut state = state_wrapper
                    .write()
                    .expect("Fail to aquare the writer state");

                state.mouse.x = position.x;
                state.mouse.y = position.y
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }),
        device_event: None,
    }
}

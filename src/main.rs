use std::{f32::consts::PI, sync::Arc};

use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::wgpu::{Callbacks, WindowSurface};

mod wgpu;

fn main() {
    wgpu::start_wgpu(1440, 900, "Orbis", false);
}

fn file_system_container<T: Renderer>(canvas: &mut Canvas<T>, size: &PhysicalSize<u32>) -> () {
    let mut main_container = Path::new();
    main_container.rect(0.0, 0.0, 320.0, size.height as f32);
    canvas.fill_path(&main_container, &Paint::color(Color::rgb(46, 52, 61)));
}

fn font_editor<T: Renderer>(canvas: &mut Canvas<T>, size: &PhysicalSize<u32>) -> () {
    let mut main_container = Path::new();
    main_container.rect(320.0, 0.0, size.width as f32, size.height as f32);
    canvas.fill_path(&main_container, &Paint::color(Color::rgb(40, 43, 51)));

    //draw mask

    let mut circle_path = Path::new();
    circle_path.arc(340.0, 20.0, 20.0, 0.0, PI * 2.0, femtovg::Solidity::Solid);
    canvas.stroke_path(&circle_path, &Paint::color(Color::rgb(255, 0, 0)).with_line_width(3.0));
}

fn run<W: WindowSurface + 'static>(
    mut canvas: Canvas<W::Renderer>,
    mut surface: W,
    window: Arc<Window>,
) -> Callbacks {
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

                file_system_container(&mut canvas, &size);
                font_editor(&mut canvas, &size);

                surface.present(&mut canvas);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }),
        device_event: None,
    }
}

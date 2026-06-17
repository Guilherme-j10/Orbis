use std::sync::Arc;

use femtovg::{Canvas, Color};
use winit::{event::WindowEvent, window::Window};

use crate::wgpu::{Callbacks, WindowSurface};

mod wgpu;

fn main() {
    wgpu::start_wgpu(1440, 900, "Orbis", false);
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
            },
            WindowEvent::RedrawRequested => {
                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                canvas.set_size(size.width, size.height, dpi_factor as f32);
                canvas.clear_rect(0, 0, size.width, size.height, Color::rgb(0, 0, 0));

                surface.present(&mut canvas);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }),
        device_event: None,
    }
}

use std::{f32::consts::PI, sync::Arc};

use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::wgpu::{Callbacks, WindowSurface};

mod wgpu;
mod engine;

fn main() {
    wgpu::start_wgpu(1440, 900, "Orbis", false);
}

fn file_system_container<T: Renderer>(canvas: &mut Canvas<T>, size: &PhysicalSize<u32>) -> () {
    let mut main_container = Path::new();
    main_container.rect(0.0, 0.0, 320.0, size.height as f32);
    canvas.fill_path(&main_container, &Paint::color(Color::rgb(46, 52, 61)));
}

fn font_editor<T: Renderer>(canvas: &mut Canvas<T>, size: &PhysicalSize<u32>) -> () {
    let bounds = (320.0, 0.0);

    let mut main_container = Path::new();
    main_container.rect(320.0, 0.0, size.width as f32, size.height as f32);
    canvas.fill_path(&main_container, &Paint::color(Color::rgb(40, 43, 51)));

    //draw mask
    draw_mask(canvas, bounds.0 + 20.0, bounds.1 + 20.0);
    draw_mask(canvas, bounds.0 + 120.0, bounds.1 + 20.0);
}

fn draw_mask<T: Renderer>(canvas: &mut Canvas<T>, cx: f32, cy: f32) -> () {
    let (bw, bh) = (100.0, 80.0);
    let default_mask_color = Paint::color(Color::rgb(82, 88, 95)).with_line_width(4.0);
    let circle_center_x = cx + bw / 2.0;
    let circle_center_y = cy + bh / 2.0;
    let base_circle_r = 15.0;

    let mut box_path = Path::new();
    box_path.rect(cx as f32, cy as f32, bw, bh);
    canvas.stroke_path(&box_path, &Paint::color(Color::rgb(255, 0, 0)));

    let mut base_circle = Path::new();
    base_circle.arc(
        circle_center_x,
        circle_center_y,
        base_circle_r,
        0.0,
        PI * 2.0,
        femtovg::Solidity::Solid,
    );
    canvas.stroke_path(&base_circle, &default_mask_color);

    let mut center_circle = Path::new();
    center_circle.circle(circle_center_x, circle_center_y, 5.0);
    canvas.fill_path(&center_circle, &default_mask_color);

    let mut lag_paint_default = default_mask_color.clone();
    lag_paint_default.set_line_width(5.0);

    let (linit_x, linit_y) = (circle_center_x - (base_circle_r / 2.0), circle_center_y);
    let mut left_lag = Path::new();
    left_lag.move_to(linit_x - 5.5, linit_y);
    left_lag.line_to(linit_x - 25.0, linit_y);
    canvas.stroke_path(&left_lag, &lag_paint_default);

    let (rinit_x, rinit_y) = (circle_center_x + (base_circle_r / 2.0), circle_center_y);
    let mut right_lag = Path::new();
    right_lag.move_to(rinit_x + 5.5, rinit_y);
    right_lag.line_to(rinit_x + 25.0, rinit_y);
    canvas.stroke_path(&right_lag, &lag_paint_default);

    let (tinit_x, tinit_y) = (circle_center_x, circle_center_y - (base_circle_r / 2.0));
    let mut top_lag = Path::new();
    top_lag.move_to(tinit_x, tinit_y - 5.5);
    top_lag.line_to(tinit_x, tinit_y - 25.0);
    canvas.stroke_path(&top_lag, &lag_paint_default);

    let (binit_x, binit_y) = (circle_center_x, circle_center_y + (base_circle_r / 2.0));
    let mut bottom_lag = Path::new();
    bottom_lag.move_to(binit_x, binit_y + 5.5);
    bottom_lag.line_to(binit_x, binit_y + 25.0);
    canvas.stroke_path(&bottom_lag, &lag_paint_default);

    let (clinit_x, clinit_y) = (circle_center_x + 19.8, circle_center_y);
    let mut half_right_circle = Path::new();
    half_right_circle.arc(
        clinit_x,
        clinit_y,
        15.0,
        PI / 2.0,
        PI * 1.5,
        femtovg::Solidity::Solid,
    );
    canvas.stroke_path(&half_right_circle, &lag_paint_default);

    let (clinit_x, clinit_y) = (circle_center_x - 19.8, circle_center_y);
    let mut half_left_circle = Path::new();
    half_left_circle.arc(
        clinit_x,
        clinit_y,
        15.0,
        PI * 1.5,
        PI / 2.0,
        femtovg::Solidity::Solid,
    );
    canvas.stroke_path(&half_left_circle, &lag_paint_default);

    let (antlrinit_x, antlrinit_y) = (circle_center_x - base_circle_r + 9.0, circle_center_y - base_circle_r);
    let mut angle_top_left_lag = Path::new();
    angle_top_left_lag.move_to(antlrinit_x, antlrinit_y);
    angle_top_left_lag.line_to(antlrinit_x - 8.0, antlrinit_y - 13.0);
    canvas.stroke_path(&angle_top_left_lag, &lag_paint_default);

    let (antrtinit_x, antrtinit_y) = (circle_center_x + base_circle_r - 9.0, circle_center_y - base_circle_r);
    let mut angle_top_right_lag = Path::new();
    angle_top_right_lag.move_to(antrtinit_x, antrtinit_y);
    angle_top_right_lag.line_to(antrtinit_x + 8.0, antrtinit_y - 13.0);
    canvas.stroke_path(&angle_top_right_lag, &lag_paint_default);

    let (antlbinit_x, antlbinit_y) = (circle_center_x - base_circle_r + 9.0, circle_center_y + base_circle_r);
    let mut angle_bottom_left_lag = Path::new();
    angle_bottom_left_lag.move_to(antlbinit_x, antlbinit_y);
    angle_bottom_left_lag.line_to(antlbinit_x - 8.0, antlbinit_y + 13.0);
    canvas.stroke_path(&angle_bottom_left_lag, &lag_paint_default);

    let (antrbinit_x, antrbinit_y) = (circle_center_x + base_circle_r - 9.0, circle_center_y + base_circle_r);
    let mut angle_bottom_right_lag = Path::new();
    angle_bottom_right_lag.move_to(antrbinit_x, antrbinit_y);
    angle_bottom_right_lag.line_to(antrbinit_x + 8.0, antrbinit_y + 13.0);
    canvas.stroke_path(&angle_bottom_right_lag, &lag_paint_default);
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

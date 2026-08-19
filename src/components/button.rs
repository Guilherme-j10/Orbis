use femtovg::{Align, Baseline, Canvas, Paint, Path, Renderer};
use winit::window::CursorIcon;

use crate::{dtos::app::ApplicationStateType, utils::style::{ComputedStyle, UIStyle}};

pub struct UIButton<'a> {
    pub app_state: ApplicationStateType,
    pub text: &'static str,
    pub style: Vec<UIStyle>,
    path: Path,
    on_click: Option<Box<dyn Fn() -> () + 'a>>,
}

impl<'a> UIButton<'a>{
    pub fn new(
        app_state: ApplicationStateType,
        text: &'static str,
        style: Vec<UIStyle>,
        on_click: Option<Box<dyn Fn() -> () + 'a>>,
    ) -> Self {
        Self {
            app_state,
            style,
            text,
            path: Path::new(),
            on_click,
        }
    }

    pub fn draw<T: Renderer>(&mut self, canvas: &mut Canvas<T>) -> () {
        let style = ComputedStyle::from(&self.style);

        let text_paint = Paint::color(style.text_color)
            .with_font(&style.fonts)
            .with_font_size(style.text_size)
            .with_font_italic(false)
            .with_font_weight(500.0)
            .with_text_align(Align::Center)
            .with_text_baseline(Baseline::Middle);

        let text_width = canvas
            .measure_text(0.0, 0.0, self.text, &text_paint)
            .expect("Failed to measure button text")
            .width();

        // the font size is used instead of the measured height so buttons with
        // and without ascenders/descenders still end up with the same height
        let width = text_width + style.padding.0 * 2.0;
        let height = style.text_size + style.padding.1 * 2.0;

        let mut x = style.position.0;
        let mut y = style.position.1;

        if let Some((origin_x, region_width)) = style.justify {
            x = origin_x + (region_width - width) / 2.0;
        }

        if let Some((origin_y, region_height)) = style.align {
            y = origin_y + (region_height - height) / 2.0;
        }

        y += style.margin_top;

        if self.is_mouse_over(x, y, width, height) {
            self.app_state.window.set_cursor(CursorIcon::Pointer);
        } else {
            self.app_state.window.set_cursor(CursorIcon::Default);
        }

        self.button_clicked(x, y, width, height);

        self.path.rect(x, y, width, height);

        canvas
            .fill_path(&self.path, &Paint::color(style.background));
        canvas
            .fill_text(x + width / 2.0, y + height / 2.0, self.text, &text_paint)
            .expect("Failed to fill button text");
    }

    fn is_mouse_over(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let mouse = self.app_state.hardware.mouse.borrow();
        let mx = mouse.x as f32;
        let my = mouse.y as f32;

        mx >= x && my >= y && mx <= x + w && my <= y + h
    }

    fn button_clicked(&self, x: f32, y: f32, w: f32, h: f32) -> () {
        if self.app_state.had_click() && self.is_mouse_over(x, y, w, h) {
            if let Some(callback) = &self.on_click {
                callback();
            }
        }
    }
}
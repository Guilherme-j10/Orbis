use femtovg::{Align, Baseline, Canvas, Color, FontId, Paint, Path, Renderer};
use winit::window::CursorIcon;

use crate::dtos::app::ApplicationStateType;

pub enum StyleProp {
    JustifyCenter(Option<(f32, f32)>), // x, w
    AlignCenter(Option<(f32, f32)>),   // y, h
    //Fixed(f32, f32),                   // x, y
    MarginTop(f32),
    Padding(f32, f32), // horizontal, vertical
    Background(Color),
    TextColor(Color),
    TextSize(f32),
    Font(Vec<FontId>),
}

struct ComputedStyle {
    position: (f32, f32),
    justify: Option<(f32, f32)>,
    align: Option<(f32, f32)>,
    margin_top: f32,
    padding: (f32, f32),
    background: Color,
    text_color: Color,
    text_size: f32,
    fonts: Vec<FontId>,
}

pub struct UIButton<'a, T: Renderer> {
    pub app_state: ApplicationStateType,
    pub canvas: &'a mut Canvas<T>,
    pub text: &'static str,
    pub style: Vec<StyleProp>,
    path: Path,
    on_click: Option<Box<dyn Fn() -> () + 'a>>,
}

impl<'a, T: Renderer> UIButton<'a, T> {
    pub fn new(
        app_state: ApplicationStateType,
        canvas: &'a mut Canvas<T>,
        text: &'static str,
        style: Vec<StyleProp>,
        on_click: Option<Box<dyn Fn() -> () + 'a>>,
    ) -> Self {
        Self {
            app_state,
            canvas,
            style,
            text,
            path: Path::new(),
            on_click,
        }
    }

    pub fn draw(&mut self) -> () {
        let style = self.computed_style();

        let text_paint = Paint::color(style.text_color)
            .with_font(&style.fonts)
            .with_font_size(style.text_size)
            .with_font_italic(false)
            .with_font_weight(500.0)
            .with_text_align(Align::Center)
            .with_text_baseline(Baseline::Middle);

        let text_width = self
            .canvas
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

        self.canvas
            .fill_path(&self.path, &Paint::color(style.background));
        self.canvas
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

    fn computed_style(&self) -> ComputedStyle {
        let mut computed = ComputedStyle {
            position: (0.0, 0.0),
            justify: None,
            align: None,
            margin_top: 0.0,
            padding: (16.0, 8.0),
            background: Color::rgb(35, 35, 48),
            text_color: Color::rgb(220, 220, 235),
            text_size: 16.0,
            fonts: vec![],
        };

        for style in &self.style {
            match style {
                StyleProp::JustifyCenter(region) => {
                    computed.justify = Some(region.unwrap_or((0.0, self.canvas.width() as f32)));
                }
                StyleProp::AlignCenter(region) => {
                    computed.align = Some(region.unwrap_or((0.0, self.canvas.height() as f32)));
                }
                StyleProp::MarginTop(margin) => {
                    computed.margin_top = *margin;
                }
                StyleProp::Padding(horizontal, vertical) => {
                    computed.padding = (*horizontal, *vertical);
                }
                StyleProp::Background(color) => {
                    computed.background = *color;
                }
                StyleProp::TextColor(color) => {
                    computed.text_color = *color;
                }
                StyleProp::TextSize(size) => {
                    computed.text_size = *size;
                }
                StyleProp::Font(font_ids) => {
                    computed.fonts = font_ids.clone();
                }
            }
        }

        computed
    }
}

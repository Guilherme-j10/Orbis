use femtovg::{Canvas, Color, Paint, Path, Renderer};

use crate::dtos::app::ApplicationStateType;

#[allow(dead_code)]
pub enum CircleStyle {
    Radius(f32),
    JustifyCenter(Option<(f32, f32)>), // x, w
    AlignCenter(Option<(f32, f32)>),   // y, h
    MarginTop(f32),
    Padding(f32, f32), // horizontal, vertical
    Background(Color),
}

#[derive(Default)]
pub struct ComputedStyle {
    radius: f32, // width and height
    justify_center: Option<(f32, f32)>,
    align_center: Option<(f32, f32)>,
    margin_top: f32,
    padding: (f32, f32),
    background: Color,
}

pub struct UICircleContainer<'a, T: Renderer> {
    pub app_state: ApplicationStateType,
    pub canvas: &'a mut Canvas<T>,
    pub style: Vec<CircleStyle>,
}

impl<'a, T: Renderer> UICircleContainer<'a, T> {
    pub fn new(
        state: ApplicationStateType,
        canvas: &'a mut Canvas<T>,
        style: Vec<CircleStyle>,
    ) -> Self {
        Self {
            style,
            canvas,
            app_state: state,
        }
    }

    pub fn draw(&mut self) -> () {
        let style = self.compute_style();

        let mut path = Path::new();
        let mut x = 0.0;
        let mut y = 0.0;

        if let Some((ay, ah)) = style.align_center {
            y = ay + ah / 2.0;
        }

        if let Some((ax, aw)) = style.justify_center {
            x = ax + aw / 2.0;
        }

        path.circle(x, y, style.radius);
        self.canvas.fill_path(&path, &Paint::color(style.background));
    }

    fn compute_style(&self) -> ComputedStyle {
        let mut computed = ComputedStyle::default();

        for style in &self.style {
            match style {
                CircleStyle::Radius(val) => {
                    computed.radius = *val;
                }
                CircleStyle::Padding( horizontal, vertical) => {
                    computed.padding = (*horizontal, *vertical);
                }
                CircleStyle::MarginTop(val) => {
                    computed.margin_top = *val;
                }
                CircleStyle::Background(color) => {
                    computed.background = *color;
                }
                CircleStyle::AlignCenter(val) => {
                    computed.align_center = *val;
                }
                CircleStyle::JustifyCenter(val) => {
                    computed.justify_center = *val;
                }
            }
        }

        computed 
    }
}

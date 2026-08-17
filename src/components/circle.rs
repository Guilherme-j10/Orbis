use femtovg::{Canvas, Color, Path, Renderer};

use crate::dtos::app::ApplicationStateType;

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
    justify_center: (f32, f32),
    align_center: (f32, f32),
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

    pub fn draw(&self) -> () {
        let style = self.compute_style();

        let mut path = Path::new();
        let mut x = 0.0;
        let mut y = 0.0;

        if style.align_center.0 > 0.0 || style.align_center.1 > 0.0 {
        }

        path.circle(x, y, style.radius);
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
                    computed.align_center = val.unwrap_or((0.0, 0.0));
                }
                CircleStyle::JustifyCenter(val) => {
                    computed.justify_center = val.unwrap_or((0.0, 0.0));
                }
            }
        }

       computed 
    }
}

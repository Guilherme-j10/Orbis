use femtovg::{Canvas, Color, Path, Renderer};

use crate::dtos::app::ApplicationStateType;

pub enum CircleStyle {
    Dimensions((f32, f32)),
    JustifyCenter(Option<(f32, f32)>), // x, w
    AlignCenter(Option<(f32, f32)>),   // y, h
    MarginTop(f32),
    Padding(f32, f32), // horizontal, vertical
    Background(Color),
}

#[derive(Default)]
pub struct ComputedStyle {
    dimensions: (f32, f32), // width and height
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

        let path = Path::new();
        //path.circle(x, y, style.dimensions.0, style.dimensions.1);
    }

    fn compute_style(&self) -> ComputedStyle {
        let mut computed = ComputedStyle::default();

        for style in &self.style {
            match style {
                CircleStyle::Dimensions(val) => {
                    computed.dimensions = *val;
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

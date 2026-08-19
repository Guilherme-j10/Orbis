use femtovg::{Canvas, Paint, Path, Renderer};

use crate::{dtos::app::ApplicationStateType, utils::style::{ComputedStyle, UIStyle}};

#[allow(dead_code)]
pub struct UICircleContainer {
    pub app_state: ApplicationStateType,
    pub style: Vec<UIStyle>,
}

impl UICircleContainer {
    pub fn new(
        state: ApplicationStateType,
        style: Vec<UIStyle>,
    ) -> Self {
        Self {
            style,
            app_state: state,
        }
    }

    pub fn draw<T: Renderer>(&mut self, canvas: &mut Canvas<T>) -> () {
        let style = ComputedStyle::from(&self.style);

        let mut path = Path::new();
        let mut x = 0.0;
        let mut y = 0.0;

        if let Some((ay, ah)) = style.align {
            y = ay + ah / 2.0;
        }

        if let Some((ax, aw)) = style.justify {
            x = ax + aw / 2.0;
        }

        path.circle(x, y, style.radius);
        canvas.fill_path(&path, &Paint::color(style.background));
    }
}

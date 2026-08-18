use femtovg::{Canvas, Paint, Path, Renderer};

use crate::{dtos::app::ApplicationStateType, utils::style::{ComputedStyle, UIStyle}};

#[allow(dead_code)]
pub struct UICircleContainer<'a, T: Renderer> {
    pub app_state: ApplicationStateType,
    pub canvas: &'a mut Canvas<T>,
    pub style: Vec<UIStyle>,
}

impl<'a, T: Renderer> UICircleContainer<'a, T> {
    pub fn new(
        state: ApplicationStateType,
        canvas: &'a mut Canvas<T>,
        style: Vec<UIStyle>,
    ) -> Self {
        Self {
            style,
            canvas,
            app_state: state,
        }
    }

    pub fn draw(&mut self) -> () {
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
        self.canvas.fill_path(&path, &Paint::color(style.background));
    }
}

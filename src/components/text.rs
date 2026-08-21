use femtovg::{Baseline, Canvas, Paint, Renderer};

use crate::utils::style::{ComputedStyle, UIStyle};

pub struct UIText {
    text: String,
    style: Vec<UIStyle>,
}

impl UIText {
    pub fn new(text: String, style: Vec<UIStyle>) -> Self {
        Self { style, text }
    }

    pub fn draw<T: Renderer>(&self, canvas: &mut Canvas<T>) -> () {
        let style = ComputedStyle::from(&self.style);
        
        if style.text_size == 0.0 {
            panic!("Text size missing");
        }

        if style.fonts.len() == 0 {
            panic!("fonts not provided");
        }

        let mut bounds = style.bounds_size.ok_or("Bounds not provided").unwrap();

        let mut text_paint = Paint::color(style.text_color)
            .with_font(&style.fonts)
            .with_font_size(style.text_size)
            .with_font_italic(false)
            .with_font_weight(500.0)
            .with_text_baseline(Baseline::Middle);

        let text_metrics = canvas
            .measure_text(0.0, 0.0, &self.text, &text_paint)
            .expect("Failed to measure button text");

        if let Some(align) = style.text_align {
            text_paint = text_paint.with_text_align(align);
        }

        // horizontal 
        if style.padding.0 > 0.0 {
            bounds.0 = bounds.0 + style.padding.0;
        }
        
        // vertical
        if style.padding.1 > 0.0 {
            bounds.1 = bounds.1 + style.padding.1;
        }

        canvas
            .fill_text(bounds.0, bounds.1 + text_metrics.height(), &self.text, &text_paint)
            .map_err(|err| format!("Failed to draw text: {:?} - err: {err}", self.text))
            .unwrap();
    }
}

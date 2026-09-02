use std::time::{Duration, SystemTime};

use femtovg::{Align, Baseline, Canvas, Color, Paint, Path, Renderer};

use crate::dtos::app::ApplicationStateType;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NotificationKind {
    Success(String),
    Error(String),
    Info(String),
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub kind: NotificationKind,
    pub create_at: SystemTime,
}

pub struct NotificationRender<'a, T: Renderer> {
    pub canvas: &'a mut Canvas<T>,
    pub state: ApplicationStateType,
    pub duration_in_seconds: u64,
}

impl<'a, T: Renderer> NotificationRender<'a, T> {
    pub fn new(canvas: &'a mut Canvas<T>, state: ApplicationStateType) -> Self {
        Self {
            canvas,
            duration_in_seconds: 3,
            state,
        }
    }

    pub fn render_loop(&mut self) -> () {
        let duration_in_secs = Duration::from_secs(self.duration_in_seconds);
        let notifications = {
            let mut timers = self.state.notification_timers.borrow_mut();
            timers.retain(|t| t.create_at.elapsed().unwrap() <= duration_in_secs);
            timers.clone() 
        };

        for notify in notifications.iter() {
            self.draw_notifications(&notify.kind);
        }
    }

    pub fn draw_notifications(&mut self, kind: &NotificationKind) -> () {
        let (color, message) = match kind {
            NotificationKind::Error(message) => (Color::rgb(255, 88, 88), message),
            NotificationKind::Info(message) => (Color::rgb(255, 255, 255), message),
            NotificationKind::Success(message) => (Color::rgb(73, 229, 112), message),
        };

        let font_size: f32 = 11.0;
        let text_paint = Paint::color(color)
            .with_font(&self.state.app_data.font_ids.borrow())
            .with_font_size(font_size)
            .with_font_weight(500.0)
            .with_text_align(Align::Center)
            .with_text_baseline(Baseline::Middle);

        let text_width = self
            .canvas
            .measure_text(0.0, 0.0, &message, &text_paint)
            .expect("Failed to measure button text")
            .width();

        let window_size = self.state.window.inner_size();
        let (ph, pv): (f32, f32) = (10.0, 5.0);

        let mut path = Path::new();
        let height = font_size + (pv * 2.0);
        let width = text_width + (ph * 2.0);

        path.rect(
            window_size.width as f32 - width,
            window_size.height as f32 - height,
            width,
            height,
        );

        self.canvas
            .fill_path(&path, &Paint::color(Color::rgb(50, 50, 69)));
        self.canvas
            .fill_text(
                window_size.width as f32 - width + width / 2.0,
                window_size.height as f32 - height + height / 2.0,
                message,
                &text_paint,
            )
            .expect("fail to draw notification text");
    }
}

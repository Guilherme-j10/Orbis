use std::f32::consts::PI;

use femtovg::{Canvas, Paint, Path, Renderer};

pub type ContextPoints = (f32, f32);
type OrbPartCode = u8;
pub struct OrbFont<'a, T: Renderer> {
    _fsize: f32, //means width
    default_paint: Paint,
    lag_paint: Paint,
    _context_point: (f32, f32), // x, y
    font_center: (f32, f32),   // x, y
    canvas: &'a mut Canvas<T>,
    _draw_box: bool,
    base_circle_r: f32,
    parts_to_draw: Vec<OrbParts>,
}

impl<'a, T: Renderer> OrbFont<'a, T> {
    pub fn init(canvas: &'a mut Canvas<T>, fsize: f32, color: Paint, cp: (f32, f32)) -> Self {
        let (bw, bh) = (100.0, 80.0);
        let base_circle_r = 15.0;
        let font_center = (cp.0 + bw / 2.0, cp.1 + bh / 2.0);

        let mut lag_paint_default = color.clone();
        lag_paint_default.set_line_width(5.0);

        Self {
            canvas,
            _fsize: fsize,
            default_paint: color,
            lag_paint: lag_paint_default,
            _context_point: cp,
            _draw_box: false,
            parts_to_draw: vec![],
            font_center,
            base_circle_r,
        }
    }

    pub fn with_box(self, draw_box: bool) -> Self {
        Self { _draw_box: draw_box, ..self }
    }

    pub fn with_parts(self, parts: Vec<OrbParts>) -> Self {
        Self {
            parts_to_draw: parts,
            ..self
        }
    }

    pub fn draw(&mut self) -> () {
        if self._draw_box == true {
            // draw box here
        }

        let parts_to_draw = self.parts_to_draw.clone();
        for part in parts_to_draw.iter() {
            self.draw_part_by_match(part);
        }
    }

    pub fn draw_part_by_match(&mut self, part: &OrbParts) -> () {
        match part {
            OrbParts::CircleBase => self.draw_circle_base(),
            OrbParts::CircleSmallCenter => self.draw_circle_small_center(),
            OrbParts::LeftLag => self.draw_left_lag(),
            OrbParts::RightLag => self.draw_right_lag(),
            OrbParts::TopLag => self.draw_top_lag(),
            OrbParts::BottomLag => self.draw_bottom_lag(),
            OrbParts::HalfLeftCircle => self.draw_half_left_circle(),
            OrbParts::HalfRightCircle => self.draw_half_right_circle(),
            OrbParts::TopAngleLeftLag => self.draw_top_angle_left_lag(),
            OrbParts::TopAngleRightLag => self.draw_top_angle_right_lag(),
            OrbParts::BottomAngleLeftLag => self.draw_bottom_angle_left_lag(),
            OrbParts::BottomAngleRightLag => self.draw_bottom_angle_right_lag(),
        }
    }

    pub fn draw_circle_base(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let mut base_circle = Path::new();
        base_circle.arc(
            cx,
            cy,
            self.base_circle_r,
            0.0,
            PI * 2.0,
            femtovg::Solidity::Solid,
        );
        self.canvas.stroke_path(&base_circle, &self.default_paint);
    }

    pub fn draw_circle_small_center(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let mut center_circle = Path::new();
        center_circle.circle(cx, cy, self.base_circle_r / 3.0);
        self.canvas.fill_path(&center_circle, &self.default_paint);
    }

    pub fn draw_left_lag(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (cx - (self.base_circle_r / 2.0), cy);
        let mut path = Path::new();
        path.move_to(initx - 5.5, inity);
        path.line_to(initx - 25.0, inity);
        self.canvas.stroke_path(&path, &self.lag_paint);
    }

    pub fn draw_right_lag(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (cx + (self.base_circle_r / 2.0), cy);
        let mut path = Path::new();
        path.move_to(initx + 5.5, inity);
        path.line_to(initx + 25.0, inity);
        self.canvas.stroke_path(&path, &self.lag_paint);
    }

    pub fn draw_top_lag(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (cx, cy - (self.base_circle_r / 2.0));
        let mut path = Path::new();
        path.move_to(initx, inity - 5.5);
        path.line_to(initx, inity - 25.0);
        self.canvas.stroke_path(&path, &self.lag_paint);
    }

    pub fn draw_bottom_lag(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (cx, cy + (self.base_circle_r / 2.0));
        let mut path = Path::new();
        path.move_to(initx, inity + 5.5);
        path.line_to(initx, inity + 25.0);
        self.canvas.stroke_path(&path, &self.lag_paint);
    }

    pub fn draw_half_left_circle(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (cx - 19.8, cy);
        let mut path = Path::new();
        path.arc(
            initx,
            inity,
            15.0,
            PI * 1.5,
            PI / 2.0,
            femtovg::Solidity::Solid,
        );
        self.canvas.stroke_path(&path, &self.lag_paint);
    }

    pub fn draw_half_right_circle(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (cx + 19.8, cy);
        let mut path = Path::new();
        path.arc(
            initx,
            inity,
            15.0,
            PI / 2.0,
            PI * 1.5,
            femtovg::Solidity::Solid,
        );
        self.canvas.stroke_path(&path, &self.lag_paint);
    }

    pub fn draw_top_angle_left_lag(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (cx - self.base_circle_r + 9.0, cy - self.base_circle_r);
        let mut path = Path::new();
        path.move_to(initx, inity);
        path.line_to(initx - 8.0, inity - 13.0);
        self.canvas.stroke_path(&path, &self.lag_paint);
    }

    pub fn draw_top_angle_right_lag(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (cx + self.base_circle_r - 9.0, cy - self.base_circle_r);
        let mut path = Path::new();
        path.move_to(initx, inity);
        path.line_to(initx + 8.0, inity - 13.0);
        self.canvas.stroke_path(&path, &self.lag_paint);
    }

    pub fn draw_bottom_angle_left_lag(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (cx - self.base_circle_r + 9.0, cy + self.base_circle_r);
        let mut path = Path::new();
        path.move_to(initx, inity);
        path.line_to(initx - 8.0, inity + 13.0);
        self.canvas.stroke_path(&path, &self.lag_paint);
    }

    pub fn draw_bottom_angle_right_lag(&mut self) -> () {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (cx + self.base_circle_r - 9.0, cy + self.base_circle_r);
        let mut path = Path::new();
        path.move_to(initx, inity);
        path.line_to(initx + 8.0, inity + 13.0);
        self.canvas.stroke_path(&path, &self.lag_paint);
    }
}

#[derive(Clone)]
pub enum OrbParts {
    CircleBase = 1,
    CircleSmallCenter = 2,
    LeftLag = 3,
    RightLag = 4,
    TopLag = 5,
    BottomLag = 6,
    HalfLeftCircle = 7,
    HalfRightCircle = 8,
    TopAngleLeftLag = 9,
    TopAngleRightLag = 10,
    BottomAngleLeftLag = 11,
    BottomAngleRightLag = 12,
}

impl From<OrbParts> for u8 {
    fn from(value: OrbParts) -> Self {
        value as u8
    }
}

impl TryFrom<OrbPartCode> for OrbParts {
    type Error = &'static str;

    fn try_from(value: OrbPartCode) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OrbParts::CircleBase),
            2 => Ok(OrbParts::CircleSmallCenter),
            3 => Ok(OrbParts::LeftLag),
            4 => Ok(OrbParts::RightLag),
            5 => Ok(OrbParts::TopLag),
            6 => Ok(OrbParts::BottomLag),
            7 => Ok(OrbParts::HalfLeftCircle),
            8 => Ok(OrbParts::HalfRightCircle),
            9 => Ok(OrbParts::TopAngleLeftLag),
            10 => Ok(OrbParts::TopAngleRightLag),
            11 => Ok(OrbParts::BottomAngleLeftLag),
            12 => Ok(OrbParts::BottomAngleRightLag),
            _ => Err("Invalid part code"),
        }
    }
}
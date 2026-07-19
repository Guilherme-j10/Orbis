use std::f32::consts::PI;

use femtovg::{Canvas, Color, Paint, Path, Renderer};

use crate::{font_engine::font::OrbParts::{CircleBase, HalfRightCircle}, interfaces::app::OrbPartCode, utils::interpolation};

pub struct FillRotate {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    a: f32,
}

impl FillRotate {
    pub fn render<T: Renderer>(
        &self,
        mx: f32,
        my: f32,
        canvas: &mut Canvas<T>,
        path: &mut Path,
        color: &Paint,
    ) -> () {
        canvas.save();

        canvas.translate(self.x + self.w / 2.0, self.y + self.h / 2.0);
        canvas.rotate(self.a * PI / 180.0);

        if canvas.contains_point(path, mx, my, femtovg::FillRule::NonZero) {
            path.rect(-(self.w / 2.0), -(self.h / 2.0), self.w, self.h);
            canvas.fill_path(&path, color);
        }

        canvas.restore();
    }
}

pub enum FontFillKind {
    Stroke,
    Path,
    Rotate(FillRotate),
}

#[derive(Clone)]
pub struct FontPadding {
    pub horizontal: f32,
    pub vertical: f32,
}

impl Default for FontPadding {
    fn default() -> Self {
        Self {
            horizontal: 5.5,
            vertical: 5.5
        }
    }
}

pub struct OrbFont<'a, T: Renderer> {
    default_paint: Paint,
    lag_paint: Paint,
    context_point: (f32, f32), // x, y
    font_center: (f32, f32),   // x, y
    canvas: &'a mut Canvas<T>,
    draw_box: bool,
    base_circle_r: f32,
    parts_to_draw: Vec<OrbParts>,
    h_leg_w: f32,
    h_leg_h: f32,
    v_leg_w: f32,
    v_leg_h: f32,
    lf_angle_d: f32,
    lr_angle_d: f32,
    circle_width: f32,
    box_dimension: (f32, f32),
}

impl<'a, T: Renderer> OrbFont<'a, T> {
    pub fn with_box(self, draw_box: bool) -> Self {
        Self {
            draw_box: draw_box,
            ..self
        }
    }

    pub fn with_parts(self, parts: Vec<OrbParts>) -> Self {
        Self {
            parts_to_draw: parts,
            ..self
        }
    }

    pub fn init(
        canvas: &'a mut Canvas<T>,
        fsize: f32,
        padding: Option<FontPadding>,
        mut color: Paint,
        cp: (f32, f32),
    ) -> Self {
        let font_size_input: [f32; 2] = [12.0, 50.0];

        if fsize > *font_size_input.get(1).unwrap() {
            panic!("font size great than 50")
        } else if fsize < *font_size_input.get(0).unwrap() {
            panic!("font size is lass than 12")
        }

        let h_leg_w = interpolation(fsize, font_size_input.to_vec(), [12.0, 20.0].to_vec());
        let h_leg_h = interpolation(fsize, font_size_input.to_vec(), [3.3, 5.5].to_vec());

        let v_leg_w = interpolation(fsize, font_size_input.to_vec(), [3.3, 5.5].to_vec());
        let v_leg_h = interpolation(fsize, font_size_input.to_vec(), [9.0, 15.0].to_vec());

        let base_circle_r = interpolation(fsize, font_size_input.to_vec(), [9.0, 15.0].to_vec());

        let padding = padding.unwrap_or_default();
        let width_box = ((h_leg_w + base_circle_r) * 2.0) + padding.horizontal;
        let height_box = (v_leg_h + base_circle_r) * 2.0 + padding.vertical;

        let (bw, bh) = (width_box, height_box);

        let font_center = (cp.0 + bw / 2.0, cp.1 + bh / 2.0);

        color.set_line_width(interpolation(
            fsize,
            font_size_input.to_vec(),
            [2.4, 4.0].to_vec(),
        ));

        let mut lag_paint_default = color.clone();
        lag_paint_default.set_line_width(interpolation(
            fsize,
            font_size_input.to_vec(),
            [3.0, 5.0].to_vec(),
        ));

        let lag_left_angle_distance_prop = 0.8;
        let lag_right_angle_distance_prop = 0.45;

        Self {
            box_dimension: (bw, bh),
            circle_width: color.line_width(),
            lf_angle_d: lag_left_angle_distance_prop,
            lr_angle_d: lag_right_angle_distance_prop,
            h_leg_h,
            h_leg_w,
            v_leg_h,
            v_leg_w,
            canvas,
            default_paint: color,
            lag_paint: lag_paint_default,
            context_point: cp,
            draw_box: false,
            parts_to_draw: vec![],
            font_center,
            base_circle_r,
        }
    }

    pub fn draw(&mut self) -> Vec<((Path, Paint, FontFillKind), OrbParts)> {
        if self.draw_box == true {
            let mut path = Path::new();
            path.rect(
                self.context_point.0,
                self.context_point.1,
                self.box_dimension.0,
                self.box_dimension.1,
            );

            let mut box_color = Paint::color(Color::rgb(255, 0, 0));
            box_color.set_line_width(1.0);
            self.canvas.stroke_path(&path, &box_color);
        }

        let mut path_list: Vec<((Path, Paint, FontFillKind), OrbParts)> = vec![];
        let parts_to_draw = self.parts_to_draw.clone();
        for part in parts_to_draw.iter() {
            let path = self.draw_part_by_match(part);
            path_list.push(path);
        }

        return path_list;
    }

    pub fn draw_part_by_match(&mut self, part: &OrbParts) -> ((Path, Paint, FontFillKind), OrbParts) {
        match part {
            OrbParts::CircleBase => (self.draw_circle_base(), OrbParts::CircleBase),
            OrbParts::CircleSmallCenter => (self.draw_circle_small_center(), OrbParts::CircleSmallCenter),
            OrbParts::LeftLag => (self.draw_left_lag(), OrbParts::LeftLag),
            OrbParts::RightLag => (self.draw_right_lag(), OrbParts::RightLag),
            OrbParts::TopLag => (self.draw_top_lag(), OrbParts::TopLag),
            OrbParts::BottomLag => (self.draw_bottom_lag(), OrbParts::BottomLag),
            OrbParts::HalfLeftCircle => (self.draw_half_left_circle(), OrbParts::HalfLeftCircle),
            OrbParts::HalfRightCircle => (self.draw_half_right_circle(), OrbParts::HalfRightCircle),
            OrbParts::TopAngleLeftLag => (self.draw_top_angle_left_lag(), OrbParts::TopAngleLeftLag),
            OrbParts::TopAngleRightLag => (self.draw_top_angle_right_lag(), OrbParts::TopAngleRightLag),
            OrbParts::BottomAngleLeftLag => (self.draw_bottom_angle_left_lag(), OrbParts::BottomAngleLeftLag),
            OrbParts::BottomAngleRightLag => (self.draw_bottom_angle_right_lag(), OrbParts::BottomAngleRightLag),
        }
    }

    pub fn draw_circle_base(&mut self) -> (Path, Paint, FontFillKind) {
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
        return (
            base_circle,
            self.default_paint.clone(),
            FontFillKind::Stroke,
        );
    }

    pub fn draw_circle_small_center(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let mut center_circle = Path::new();
        center_circle.circle(cx, cy, self.base_circle_r / 3.0);
        self.canvas.fill_path(&center_circle, &self.default_paint);
        return (
            center_circle,
            self.default_paint.clone(),
            FontFillKind::Path,
        );
    }

    pub fn draw_left_lag(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let line_height = self.h_leg_h;
        let line_width = self.h_leg_w;

        let (initx, inity) = (
            cx - (self.base_circle_r * 2.0) - self.circle_width,
            cy - (line_height / 2.0),
        );

        let mut path = Path::new();
        path.rect(initx, inity, line_width, line_height);
        self.canvas.fill_path(&path, &self.lag_paint);

        return (path, self.lag_paint.clone(), FontFillKind::Path);
    }

    pub fn draw_right_lag(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let line_height = self.h_leg_h;
        let line_width = self.h_leg_w;

        let (initx, inity) = (
            cx + self.base_circle_r - (self.circle_width / 2.0),
            cy - (line_height / 2.0),
        );

        let mut path = Path::new();
        path.rect(initx, inity, line_width, line_height);
        self.canvas.fill_path(&path, &self.lag_paint);

        return (path, self.lag_paint.clone(), FontFillKind::Path);
    }

    pub fn draw_top_lag(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let line_height = self.v_leg_h;
        let line_width = self.v_leg_w;

        let (initx, inity) = (
            cx - (line_width / 2.0),
            cy - (self.base_circle_r * 2.0) + (self.circle_width / 2.0),
        );

        let mut path = Path::new();
        path.rect(initx, inity, line_width, line_height);
        self.canvas.fill_path(&path, &self.lag_paint);

        return (path, self.lag_paint.clone(), FontFillKind::Path);
    }

    pub fn draw_bottom_lag(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let line_height = self.v_leg_h;
        let line_width = self.v_leg_w;

        let (initx, inity) = (
            cx - (line_width / 2.0),
            cy + (self.base_circle_r) - (self.circle_width / 2.0),
        );

        let mut path = Path::new();
        path.rect(initx, inity, line_width, line_height);
        self.canvas.fill_path(&path, &self.lag_paint);

        return (path, self.lag_paint.clone(), FontFillKind::Path);
    }

    pub fn draw_half_left_circle(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (
            cx - (((self.base_circle_r / 2.0) + self.h_leg_w) - (self.circle_width * 2.0)),
            cy,
        );
        let mut path = Path::new();
        path.arc(
            initx,
            inity,
            self.base_circle_r,
            PI * 1.5,
            PI / 2.0,
            femtovg::Solidity::Solid,
        );
        self.canvas.stroke_path(&path, &self.lag_paint);
        return (path, self.lag_paint.clone(), FontFillKind::Stroke);
    }

    pub fn draw_half_right_circle(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let (initx, inity) = (
            cx + (((self.base_circle_r / 2.0) + self.h_leg_w) - (self.circle_width * 2.0)),
            cy,
        );
        let mut path = Path::new();
        path.arc(
            initx,
            inity,
            self.base_circle_r,
            PI / 2.0,
            PI * 1.5,
            femtovg::Solidity::Solid,
        );
        self.canvas.stroke_path(&path, &self.lag_paint);
        return (path, self.lag_paint.clone(), FontFillKind::Stroke);
    }

    pub fn draw_top_angle_left_lag(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let line_height = self.v_leg_h;
        let line_width = self.v_leg_w;

        let (initx, inity) = (
            cx - self.base_circle_r * self.lf_angle_d,
            cy - (self.base_circle_r * 2.0) + self.circle_width,
        );

        let mut path = Path::new();
        let angle = -35.0;

        self.canvas.save();

        self.canvas
            .translate(initx + line_width / 2.0, inity + line_height / 2.0);
        self.canvas.rotate(angle * PI / 180.0);

        path.rect(
            -(line_width / 2.0),
            -(line_height / 2.0),
            line_width,
            line_height,
        );
        self.canvas.fill_path(&path, &self.lag_paint);

        self.canvas.restore();

        return (
            path,
            self.lag_paint.clone(),
            FontFillKind::Rotate(FillRotate {
                x: initx,
                y: inity,
                w: line_width,
                h: line_height,
                a: angle,
            }),
        );
    }

    pub fn draw_top_angle_right_lag(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let line_height = self.v_leg_h;
        let line_width = self.v_leg_w;

        let (initx, inity) = (
            cx + self.base_circle_r * self.lr_angle_d,
            cy - (self.base_circle_r * 2.0) + self.circle_width,
        );

        let mut path = Path::new();
        let angle = 35.0;

        self.canvas.save();

        self.canvas
            .translate(initx + line_width / 2.0, inity + line_height / 2.0);
        self.canvas.rotate(angle * PI / 180.0);

        path.rect(
            -(line_width / 2.0),
            -(line_height / 2.0),
            line_width,
            line_height,
        );
        self.canvas.fill_path(&path, &self.lag_paint);

        self.canvas.restore();

        return (
            path,
            self.lag_paint.clone(),
            FontFillKind::Rotate(FillRotate {
                x: initx,
                y: inity,
                w: line_width,
                h: line_height,
                a: angle,
            }),
        );
    }

    pub fn draw_bottom_angle_left_lag(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let line_height = self.v_leg_h;
        let line_width = self.v_leg_w;

        let (initx, inity) = (
            cx - self.base_circle_r * self.lf_angle_d,
            cy + (self.base_circle_r) - self.circle_width,
        );

        let mut path = Path::new();
        let angle = 35.0;

        self.canvas.save();

        self.canvas
            .translate(initx + line_width / 2.0, inity + line_height / 2.0);
        self.canvas.rotate(angle * PI / 180.0);

        path.rect(
            -(line_width / 2.0),
            -(line_height / 2.0),
            line_width,
            line_height,
        );
        self.canvas.fill_path(&path, &self.lag_paint);

        self.canvas.restore();

        return (
            path,
            self.lag_paint.clone(),
            FontFillKind::Rotate(FillRotate {
                x: initx,
                y: inity,
                w: line_width,
                h: line_height,
                a: angle,
            }),
        );
    }

    pub fn draw_bottom_angle_right_lag(&mut self) -> (Path, Paint, FontFillKind) {
        let (cx, cy) = self.font_center;
        let line_height = self.v_leg_h;
        let line_width = self.v_leg_w;

        let (initx, inity) = (
            cx + self.base_circle_r * self.lr_angle_d,
            cy + (self.base_circle_r) - self.circle_width,
        );

        let mut path = Path::new();
        let angle = -35.0;

        self.canvas.save();

        self.canvas
            .translate(initx + line_width / 2.0, inity + line_height / 2.0);
        self.canvas.rotate(angle * PI / 180.0);

        path.rect(
            -(line_width / 2.0),
            -(line_height / 2.0),
            line_width,
            line_height,
        );
        self.canvas.fill_path(&path, &self.lag_paint);

        self.canvas.restore();

        return (
            path,
            self.lag_paint.clone(),
            FontFillKind::Rotate(FillRotate {
                x: initx,
                y: inity,
                w: line_width,
                h: line_height,
                a: angle,
            }),
        );
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

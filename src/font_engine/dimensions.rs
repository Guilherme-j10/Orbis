use crate::utils::interpolation;

pub struct FontDimension {
    font_size: f32,
    base_circle_r: f32,
    h_leg_w: f32,
    v_leg_h: f32,
    padding: f32,
}

impl FontDimension {
    pub fn new(fsize: f32, padding: f32) -> Self {
        let font_size_input: [f32; 2] = [12.0, 50.0];

        if fsize > *font_size_input.get(1).unwrap() {
            panic!("font size great than 50")
        } else if fsize < *font_size_input.get(0).unwrap() {
            panic!("font size is lass than 12")
        }

        let h_leg_w = interpolation(fsize, font_size_input.to_vec(), [12.0, 20.0].to_vec());
        let base_circle_r = interpolation(fsize, font_size_input.to_vec(), [9.0, 15.0].to_vec());
        let v_leg_h = interpolation(fsize, font_size_input.to_vec(), [9.0, 15.0].to_vec());

        Self {
            font_size: fsize,
            base_circle_r,
            h_leg_w,
            padding: padding,
            v_leg_h,
        }
    }

    pub fn get_complete_width(&self) -> (f32, f32) { // (total_h, total_v)
        (
            ((self.h_leg_w + self.base_circle_r) * 2.0) + self.padding,
            ((self.v_leg_h + self.base_circle_r) * 2.0) + self.padding,
        )
    }
}

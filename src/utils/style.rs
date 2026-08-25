use femtovg::{Align, Color, FontId};

#[derive(Clone)]
pub enum UIStyle {
    BoundsSize(Option<(f32, f32, f32, f32)>), // x, y, w, h
    JustifyCenter(Option<(f32, f32)>),        // x, w
    AlignCenter(Option<(f32, f32)>),          // y, h
    MarginTop(f32),
    Padding(f32, f32),                        // horizontal, vertical
    PaddingDetail(f32, f32, f32, f32),        // left, top, right, bottom
    Background(Color),
    TextColor(Color),
    TextAlign(Option<Align>),
    TextSize(f32),
    Font(Vec<FontId>),
    Radius(f32),
}

#[derive(Default)]
pub struct ComputedStyle {
    pub bounds_size: Option<(f32, f32, f32, f32)>,
    pub position: (f32, f32),
    pub justify: Option<(f32, f32)>,
    pub align: Option<(f32, f32)>,
    pub margin_top: f32,
    pub padding_detail: (f32, f32, f32, f32),
    pub padding: (f32, f32),
    pub background: Color,
    pub text_color: Color,
    pub text_align: Option<Align>,
    pub radius: f32,
    pub text_size: f32,
    pub fonts: Vec<FontId>,
}

impl From<&Vec<UIStyle>> for ComputedStyle {
    fn from(value: &Vec<UIStyle>) -> Self {
        let mut computed = ComputedStyle::default();
        for style in value {
            match style {
                UIStyle::BoundsSize(bounds) => {
                    computed.bounds_size = *bounds;
                }
                UIStyle::Radius(val) => {
                    computed.radius = *val;
                }
                UIStyle::Padding( horizontal, vertical) => {
                    computed.padding = (*horizontal, *vertical);
                }
                UIStyle::MarginTop(val) => {
                    computed.margin_top = *val;
                }
                UIStyle::Background(color) => {
                    computed.background = *color;
                }
                UIStyle::AlignCenter(val) => {
                    computed.align = *val;
                }
                UIStyle::JustifyCenter(val) => {
                    computed.justify = *val;
                }
                UIStyle::TextColor(color) => {
                    computed.text_color = *color;
                }
                UIStyle::TextSize(size) => {
                    computed.text_size = *size;
                }
                UIStyle::TextAlign(align) => {
                    computed.text_align = *align;
                }
                UIStyle::Font(font_ids) => {
                    computed.fonts = font_ids.clone();
                },
                UIStyle::PaddingDetail(left, top, right, bottom) => {
                    computed.padding_detail = (*left, *top, *right, *bottom);
                }
            }
        }
        computed
    }
}
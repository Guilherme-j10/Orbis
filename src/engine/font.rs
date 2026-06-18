use femtovg::{Canvas, Renderer};

type OrbPartCode = u32;

pub struct OrbFont<'a, T: Renderer> {
    fsize: f32, //means width
    canvas: &'a mut Canvas<T>
}

enum OrbParts {
    CircleBase,
    CircleSmallCenter,
    LeftLag,
    RightLag,
    TopLag,
    BottomLag,
    HalfLeftCircle,
    HalfRightCircle,
    TopAngleLeftLag,
    TopAngleRightLag,
    BottomAngleLeftLag,
    BottomAngleRightLag
}

impl From<OrbPartCode> for OrbParts {
    fn from(value: OrbPartCode) -> Self {
        OrbParts::BottomAngleLeftLag
    }
}
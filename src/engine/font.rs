use femtovg::{Canvas, Renderer};

type OrbPartCode = u32;

pub struct OrbFont<'a, T: Renderer> {
    fsize: f32, //means width
    canvas: &'a mut Canvas<T>
}

enum OrbParts {
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
    BottomAngleRightLag = 12
}
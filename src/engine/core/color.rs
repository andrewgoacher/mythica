pub mod colors;

pub struct Color(f32, f32, f32, f32);

pub fn convert(color: &Color) -> (f32, f32, f32, f32) {
    (color.0, color.1, color.2, color.3)
}

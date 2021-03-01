pub struct Color(f32, f32, f32, f32);

pub fn convert(color: &Color) -> (f32, f32, f32, f32) {
    (color.0, color.1, color.2, color.3)
}

pub mod colors {
    use super::Color;

    #[allow(dead_code)]
    pub const CORNFLOWER_BLUE: Color =
        Color(100f32 / 255f32, 149f32 / 255f32, 237f32 / 255f32, 1f32);
    #[allow(dead_code)]
    pub const BLACK: Color = Color(0f32, 0f32, 0f32, 1f32);
    #[allow(dead_code)]
    pub const WHITE: Color = Color(1f32, 1f32, 1f32, 1f32);
}

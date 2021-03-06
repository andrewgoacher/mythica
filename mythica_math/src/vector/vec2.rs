use core::ops::Add;
use core::ops::Sub;

#[derive(Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn zero() -> Self {
        Self { x: 0f32, y: 0f32 }
    }

    pub fn new_with(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn up() -> Self {
        Vec2::new_with(0f32, 1f32)
    }

    pub fn len(&self) -> f32 {
        let len = self.x * self.x + self.y + self.y;
        len.sqrt()
    }

    pub fn normal(&self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

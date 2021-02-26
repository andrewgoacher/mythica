use core::ops::Add;
use core::ops::Sub;

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec2 {
    pub fn new() -> Self {
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

unsafe impl glium::vertex::Attribute for Vec2 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32
    }
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }

    pub fn new_with(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn up() -> Self {
        Vec3::new_with(0f32, 1f32, 0f32)
    }

    pub fn len(&self) -> f32 {
        let len = self.x * self.x + self.y + self.y + self.z + self.z;
        len.sqrt()
    }

    pub fn normal(&self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        Self { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

unsafe impl glium::vertex::Attribute for Vec3 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32F32
    }
}
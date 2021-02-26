use crate::Matrix;
use core::f32::consts::PI;

pub struct ProjectionOptions {
    width: u32,
    height: u32,
    fov: f32,
    far: f32,
    near: f32,
}

impl ProjectionOptions {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
            fov: PI / 3f32,
            far: 1024f32,
            near: 0.1f32,
        }
    }

    pub fn with_fov(self, fov: f32) -> Self {
        Self { fov, ..self }
    }

    pub fn with_near(self, near: f32) -> Self {
        Self { near, ..self }
    }

    pub fn with_far(self, far: f32) -> Self {
        Self { far, ..self }
    }
}

impl Matrix {
    pub fn perspective_fov(options: &ProjectionOptions) -> Self {
        let aspect_ratio = options.width as f32 / options.height as f32;
        let f = 1f32 / (options.fov / 2f32).tan();

        Self {
            m11: f * aspect_ratio,
            m21: 0f32,
            m31: 0f32,
            m41: 0f32,
            m12: 0f32,
            m22: f,
            m32: 0f32,
            m42: 0f32,
            m13: 0f32,
            m23: 0f32,
            m33: (options.far + options.near) / (options.far - options.near),
            m43: 1f32,
            m14: 0f32,
            m24: 0f32,
            m34: -(2f32 * options.far * options.near) / (options.far - options.near),
            m44: 0f32,
        }
    }
}

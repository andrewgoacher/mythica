mod glium;
pub mod projection;
pub mod view;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Matrix {
    pub m11: f32,
    pub m21: f32,
    pub m31: f32,
    pub m41: f32,

    pub m12: f32,
    pub m22: f32,
    pub m32: f32,
    pub m42: f32,

    pub m13: f32,
    pub m23: f32,
    pub m33: f32,
    pub m43: f32,

    pub m14: f32,
    pub m24: f32,
    pub m34: f32,
    pub m44: f32,
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            m11: 1f32,
            m21: 0f32,
            m31: 0f32,
            m41: 0f32,
            m12: 0f32,
            m22: 1f32,
            m32: 0f32,
            m42: 0f32,
            m13: 0f32,
            m23: 0f32,
            m33: 1f32,
            m43: 0f32,
            m14: 0f32,
            m24: 0f32,
            m34: 0f32,
            m44: 1f32,
        }
    }

    pub fn to_arr(self) -> [[f32; 4]; 4] {
        [
            [self.m11, self.m21, self.m31, self.m41],
            [self.m12, self.m22, self.m32, self.m42],
            [self.m13, self.m23, self.m33, self.m43],
            [self.m14, self.m24, self.m34, self.m44],
        ]
    }
}

impl From<[f32; 16]> for Matrix {
    fn from(arr: [f32; 16]) -> Self {
        Self {
            m11: arr[0],
            m21: arr[1],
            m31: arr[2],
            m41: arr[3],
            m12: arr[4],
            m22: arr[5],
            m32: arr[6],
            m42: arr[7],
            m13: arr[8],
            m23: arr[9],
            m33: arr[10],
            m43: arr[11],
            m14: arr[12],
            m24: arr[13],
            m34: arr[14],
            m44: arr[15],
        }
    }
}

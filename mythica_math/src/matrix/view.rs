use crate::matrix::Matrix;
use crate::vector::Vec3;

impl Matrix {
    pub fn view_matrix(pos: Vec3, direction: Vec3, up: Vec3) -> Matrix {
        let dir_norm = direction.normal();

        let s = up.cross(&dir_norm);
        let s_norm = s.normal();

        let u = dir_norm.cross(&s_norm);

        let p = Vec3::new_with(
            -pos.x * s_norm.x - pos.y * s_norm.y - pos.z * s_norm.z,
            -pos.x * u.x - pos.y * u.y - pos.z * u.z,
            -pos.x * dir_norm.x - pos.y * dir_norm.y - pos.z * dir_norm.z,
        );

        Self {
            m11: s_norm.x,
            m21: u.x,
            m31: dir_norm.x,
            m41: 0f32,
            m12: s_norm.y,
            m22: u.y,
            m32: dir_norm.y,
            m42: 0f32,
            m13: s_norm.z,
            m23: u.z,
            m33: dir_norm.z,
            m43: 0f32,
            m14: p.x,
            m24: p.y,
            m34: p.z,
            m44: 1f32,
        }
    }
}

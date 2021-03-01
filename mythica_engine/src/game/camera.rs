use mythica_math::matrix::projection::ProjectionOptions;
use mythica_math::vector::Vec3;
use mythica_math::matrix::Matrix;
use std::f32::consts::PI;

pub struct Camera {
    pub view: Matrix,
    pub projection: Matrix,
    pub position: Vec3,
    pub direction: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            view: Matrix::identity(),
            projection: Matrix::identity(),
            position: Vec3::zero(),
            direction: Vec3::zero(),
        }
    }

    pub fn with_position(self, pos: Vec3) -> Self {
        Self::update_view(Self {
            position: pos,
            ..self
        })
    }

    pub fn with_direction(self, dir: Vec3) -> Self {
        Self::update_view(Self {
            direction: dir,
            ..self
        })
    }

    fn update_view(self: Self) -> Self {
        let up = Vec3::up();
        let pos = self.position;
        let dir = self.direction;

        let view = Matrix::view_matrix(self.position, self.direction, up);

        Self {
            view,
            position: pos,
            direction: dir,
            ..self
        }
    }

    pub fn with_projection(self, width: u32, height: u32) -> Self {
        let projection = Matrix::perspective_fov(
            &ProjectionOptions::new(width, height)
                .with_fov(PI / 3f32)
                .with_near(0.1f32)
                .with_far(1024f32),
        );

        Self { projection, ..self }
    }
}

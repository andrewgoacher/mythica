use crate::graphics::shader::ShaderBuilder;
use mythica_math::vector::Vec2;
use crate::game::Camera;
use crate::core::Resource;
use mythica_math::matrix::Matrix;
use mythica_math::vector::Vec3;
use glium::texture::SrgbTexture2d;
use glium::Display;
use glium::Frame;
use glium::Program;
use glium::Texture2d;
use glium::VertexBuffer;

pub struct Billboard {
    shape: VertexBuffer<Vertex>,
    diffuse_texture: SrgbTexture2d,
    normal_map: Texture2d,
    program: Program,
    light_source: Vec3,
    model: Matrix,
}

impl Billboard {
    pub fn new(display: &Display, resources: &Resource) -> Self {
        let shape = create_shape(display);

        let image = resources.load_image_data("diffuse.jpg").unwrap();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let diffuse_texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

        let image = resources.load_image_data("normal.png").unwrap();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let normal_map = glium::texture::Texture2d::new(display, image).unwrap();

        let program = ShaderBuilder::from_file(resources, "shaders/shader").build(display);

        let model = Matrix::identity();

        let light = Vec3::new_with(1.4f32, 0.4f32, 0.7f32);

        Self {
            shape,
            diffuse_texture,
            normal_map,
            program,
            model,
            light_source: light,
        }
    }

    pub fn draw(&self, frame: &mut Frame, camera: &Camera) {
        use glium::Surface;

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        frame
            .draw(
                &self.shape,
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                &self.program,
                &uniform! {
                     model: self.model,
                      view: camera.view,
                       perspective: camera.projection,
                u_light: self.light_source,
                 diffuse_tex: &self.diffuse_texture,
                  normal_tex: &self.normal_map
                },
                &params,
            )
            .unwrap();
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: Vec3,
    normal: Vec3,
    tex_coords: Vec2,
}

fn create_shape<T: ?Sized>(display: &T) -> glium::vertex::VertexBuffer<Vertex>
where
    T: glium::backend::Facade,
{
    implement_vertex!(Vertex, position, normal, tex_coords);

    let normal = Vec3::new_with(0f32, 0f32, -1f32);

    glium::vertex::VertexBuffer::new(
        display,
        &[
            Vertex {
                position: Vec3::new_with(-1f32, 1f32, 0f32),
                normal,
                tex_coords: Vec2::new_with(0f32, 1f32),
            },
            Vertex {
                position: Vec3::new_with(1f32, 1f32, 0f32),
                normal,
                tex_coords: Vec2::new_with(1f32, 1f32),
            },
            Vertex {
                position: Vec3::new_with(-1f32, -1f32, 0f32),
                normal,
                tex_coords: Vec2::new_with(0f32, 0f32),
            },
            Vertex {
                position: Vec3::new_with(1f32, -1f32, 0f32),
                normal,
                tex_coords: Vec2::new_with(1f32, 0f32),
            },
        ],
    )
    .unwrap()
}

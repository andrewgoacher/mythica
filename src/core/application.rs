use crate::core::resource::Resource;
use crate::core::application::application_builder::ApplicationOptions;
use crate::Matrix;
use crate::ProjectionOptions;
use crate::ShaderBuilder;
use crate::Vec2;
use crate::Vec3;

use crate::core::color;

pub mod application_builder;

pub struct Application {}

impl Application {
    fn create_display(
        event_loop: &glium::glutin::event_loop::EventLoop<()>,
        options: Option<ApplicationOptions>,
    ) -> glium::Display {
        let options = options.or_else(||Some(ApplicationOptions::default())).unwrap();
        let title = options.title.or(Some("Mythica Engine")).unwrap();
        let icon = options.icon;

        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(
                options.width,
                options.height,
            ))
            .with_resizable(true)
            .with_title(title)
            .with_decorations(true)
            .with_window_icon(icon);

        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_gl(glutin::GlRequest::Latest)
            .with_vsync(true)
            .with_hardware_acceleration(Some(true));

        glium::Display::new(wb, cb, &event_loop).unwrap()
    }

    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, options: Option<ApplicationOptions>) {
        #[allow(unused_imports)]
        use glium::{glutin, Surface};

        use ::core::f32::consts::PI;

        let event_loop = glutin::event_loop::EventLoop::new();
        let display = Self::create_display(&event_loop, options);

        let shape = create_shape(&display);

        let resources = Resource::new("./assets");

        let image = resources.load_image_data("diffuse.jpg").unwrap();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
        let image = resources.load_image_data("normal.png").unwrap();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();

        let vertex_shader_src = r#"
        #version 150
        in vec3 position;
        in vec3 normal;
        in vec2 tex_coords;
        out vec3 v_normal;
        out vec3 v_position;
        out vec2 v_tex_coords;
        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;
        void main() {
            v_tex_coords = tex_coords;
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#;

        let fragment_shader_src = r#"
        #version 140
        in vec3 v_normal;
        in vec3 v_position;
        in vec2 v_tex_coords;
        out vec4 color;
        uniform vec3 u_light;
        uniform sampler2D diffuse_tex;
        uniform sampler2D normal_tex;
        const vec3 specular_color = vec3(1.0, 1.0, 1.0);
        mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
            vec3 dp1 = dFdx(pos);
            vec3 dp2 = dFdy(pos);
            vec2 duv1 = dFdx(uv);
            vec2 duv2 = dFdy(uv);
            vec3 dp2perp = cross(dp2, normal);
            vec3 dp1perp = cross(normal, dp1);
            vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
            vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;
            float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
            return mat3(T * invmax, B * invmax, normal);
        }
        void main() {
            vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
            vec3 ambient_color = diffuse_color * 0.1;
            vec3 normal_map = texture(normal_tex, v_tex_coords).rgb;
            mat3 tbn = cotangent_frame(v_normal, v_position, v_tex_coords);
            vec3 real_normal = normalize(tbn * -(normal_map * 2.0 - 1.0));
            float diffuse = max(dot(real_normal, normalize(u_light)), 0.0);
            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir);
            float specular = pow(max(dot(half_direction, real_normal), 0.0), 16.0);
            color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
        }
    "#;

        let program = ShaderBuilder::new()
            .with_vertex_shader(vertex_shader_src)
            .with_fragment_shader(fragment_shader_src)
            .build(&display);

        event_loop.run(move |event, _, control_flow| {
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }

            let mut target = display.draw();
            target.clear_color_and_depth(color::convert(&color::colors::CORNFLOWER_BLUE), 1.0);

            let model = Matrix::identity();

            let pos = Vec3::new_with(0.5f32, 0.2f32, -3f32);
            let direction = Vec3::new_with(-0.5f32, -0.2f32, 3f32);
            let up = Vec3::up();

            let view = Matrix::view_matrix(&pos, &direction, &up);
            let (width, height) = target.get_dimensions();
            let perspective = Matrix::perspective_fov(
                &ProjectionOptions::new(width, height)
                    .with_fov(PI / 3f32)
                    .with_near(0.1f32)
                    .with_far(1024f32),
            );

            let light = Vec3::new_with(1.4f32, 0.4f32, 0.7f32);

            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            target
                .draw(
                    &shape,
                    glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                    &program,
                    &uniform! { model: model, view: view, perspective: perspective,
                    u_light: light, diffuse_tex: &diffuse_texture, normal_tex: &normal_map },
                    &params,
                )
                .unwrap();
            target.finish().unwrap();
        });
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

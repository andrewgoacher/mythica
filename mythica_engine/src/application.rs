use crate::game::Context;
use crate::Resource;
use crate::game::GameState;
use crate::graphics::color;

pub struct Application {
    options: ApplicationOptions,
}

impl Application {
    fn create_display(
        &self,
        event_loop: &glium::glutin::event_loop::EventLoop<()>,
    ) -> glium::Display {
        let title = self.options.title.or(Some("Mythica Engine")).unwrap();
        let icon = self.options.icon.clone();

        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(
                self.options.width,
                self.options.height,
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

    pub fn new(options: Option<ApplicationOptions>) -> Self {
        let options = options
            .or_else(|| Some(ApplicationOptions::default()))
            .unwrap();
        Self { options }
    }

    pub fn run<'a>(&self, state: impl GameState<'a> + 'static) {
        #[allow(unused_imports)]
        use glium::{glutin, Surface};
        let event_loop = glutin::event_loop::EventLoop::new();
        let display = self.create_display(&event_loop);
        let resources = Resource::new("./assets");

        let w = self.options.width;
        let h = self.options.height;

        let context = Context {
            display,
            resources,
            dimensions: (w, h),
        };

        let mut state = GameState::on_init(state, &context);

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

            let mut target = context.display.draw();
            target.clear_color_and_depth(color::convert(&color::colors::CORNFLOWER_BLUE), 1.0);

            GameState::on_update(&mut state, &context);
            GameState::on_draw(&mut state, &mut target, &context);

            target.finish().unwrap();
        });
    }
}

pub struct ApplicationOptions {
    pub icon: Option<glutin::window::Icon>,
    pub title: Option<&'static str>,
    pub width: u32,
    pub height: u32,
}

impl ApplicationOptions {
    pub fn new(w: u32, h: u32) -> Self {
        ApplicationOptions {
            width: w,
            height: h,
            icon: None,
            title: None,
        }
    }

    pub fn with_icon(self, icon: glutin::window::Icon) -> Self {
        ApplicationOptions {
            icon: Some(icon),
            ..self
        }
    }

    pub fn with_title(self, title: &'static str) -> Self {
        ApplicationOptions {
            title: Some(title),
            ..self
        }
    }
}

impl Default for ApplicationOptions {
    fn default() -> Self {
        ApplicationOptions::new(800, 600)
    }
}

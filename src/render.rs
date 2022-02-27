use crate::canvas::Canvas;
pub use glium::glutin::event::VirtualKeyCode;
use glium::glutin::event::{Event, StartCause};
use glium::glutin::event_loop::ControlFlow;
use glium::Surface;
use std::time::Duration;
use std::time::Instant;
pub use winit_input_helper::WinitInputHelper;

pub type InputHelper = WinitInputHelper;

/// Render2D Trait which contains all the functions to:
/// 1. Draw to the screen
/// 2. Update objects on the screen
/// 3. Handle user input to the screen
///
/// This design abstracts the design of a window away from the user
///
/// 1. Create a Render2D implementor
/// 2. Use its methods to:
///     setup() -> initialize the canvas with a given frame at start up
///     update() -> Update a frame at ny time during execution

#[allow(dead_code, unused_variables)]
pub trait Render2D {
    ///  Get window properties height
    ///  Defaults to 600, override to provide custom height
    fn height(&mut self) -> u32 {
        600
    }
    /// Get wWindow properties width
    /// Defaults to 800, override to provide custom width
    fn width(&mut self) -> u32 {
        800
    }
    /// Get wWindow properties title
    /// Defaults to "Render2D Canvas", override to provide custom title
    fn title(&mut self) -> String {
        "Render2D Canvas".into()
    }
    ///
    /// Setup method called when the world is first created
    /// Must be overriden.
    ///
    fn setup(&mut self, canvas: &mut Canvas) -> bool {
        true
    }

    /// Update method called when the canvas is to be updated
    /// This is called periodically per frame and each frame is drawn individually
    /// Must be overriden/implmented
    fn update(&mut self, canvas: &mut Canvas, events: &InputHelper, delta_t: f32) -> bool {
        true
    }

    fn render(mut self)
    where
        Self: Sized + 'static,
    {
        let width = self.width();
        let height = self.height();
        let title = self.title();
        let mut canvas = Canvas::new(width, height);
        let event_loop = glium::glutin::event_loop::EventLoop::new();
        let inner_size = glium::glutin::dpi::LogicalSize::new(width, height);
        let frames_per_sec = ((1.0 / 120.0) * 1000000000.0) as u64; // 120 frames per second

        let wb = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(inner_size)
            .with_title(&title[..])
            .with_resizable(false);

        let cb = glium::glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        let texture = glium::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::EmptyMipmaps,
            width,
            height,
        )
        .unwrap();

        let mut input = InputHelper::new();
        let mut last_frame_time = Instant::now();
        let mut next_frame_time = Instant::now();
        let mut frame_counter = 0.0;
        let mut last_draw = Instant::now();
        if self.setup(&mut canvas) {
            event_loop.run(move |event, _, control_flow| {
                match event {
                    Event::NewEvents(StartCause::Init)
                    | Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                        let elapsed = Instant::now().duration_since(last_draw);
                        if !self.update(&mut canvas, &input, elapsed.as_secs_f32()) {
                            *control_flow = ControlFlow::Exit;
                        }
                        last_draw = Instant::now();
                        frame_counter += 1.0;

                        let target = display.draw();
                        texture.write(
                            glium::Rect {
                                left: 0,
                                bottom: 0,
                                width,
                                height,
                            },
                            &canvas,
                        );
                        texture
                            .as_surface()
                            .fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
                        target.finish().unwrap();
                        next_frame_time += Duration::from_nanos(frames_per_sec);
                        *control_flow = ControlFlow::WaitUntil(next_frame_time);
                    }
                    Event::WindowEvent { ref event, .. } => match event {
                        glium::glutin::event::WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        }

                        _ => (),
                    },
                    _ => (),
                }
                input.update(&event);

                let passed_time = Instant::now() - last_frame_time;
                if passed_time > Duration::from_secs(1) {
                    display.gl_window().window().set_title(&format!(
                        "{} - {} FPS",
                        &title,
                        (frame_counter / passed_time.as_secs_f32()) as u32
                    ));
                    frame_counter = 0.0;
                    last_frame_time = Instant::now();
                }
            });
        } else {
            println!("Could not start rendering...");
        }
    }
}

use crate::grfx::canvas::Canvas;
use crate::grfx::color::Color;
use pixels::{PixelsBuilder, SurfaceTexture};
use std::time::Duration;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

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
    fn height(&mut self) -> u32;
    /// Get wWindow properties width
    fn width(&mut self) -> u32;
    /// Get wWindow properties title
    fn title(&mut self) -> String;
    ///
    /// Setup method called when the world is first created
    /// Must be overriden.
    ///
    fn setup(&mut self, canvas: &mut Canvas) -> bool;

    /// Update method called when the canvas is to be updated
    /// This is called periodically per frame and each frame is drawn individually
    /// Must be overriden/implmented
    fn update(&mut self, canvas: &mut Canvas, input: &WinitInputHelper, delta_t: f32) -> bool;

    fn render(mut self)
    where
        Self: Sized + 'static,
    {
        let width = self.width();
        let height = self.height();

        let title = self.title();

        let event_loop = EventLoop::new();
        let image_size = LogicalSize::new(width, height);
        let window = WindowBuilder::new()
            .with_inner_size(image_size)
            .with_title(&title)
            .with_resizable(false)
            .with_transparent(true)
            .build(&event_loop)
            .unwrap();
        // Keyboard/mouse input handler
        let mut input = WinitInputHelper::new();
        let mut canvas: Canvas = Canvas::new(width, height);

        // Initialize a surface
        let surface_texture = SurfaceTexture::new(width, height, &window);

        // pixesl which we use to draw on the screen
        // let mut pixelbuffer = Pixels::new(self.width, self.height, surface_texture).unwrap();
        let mut pixelbuffer = PixelsBuilder::new(width, height, surface_texture)
            .enable_vsync(true)
            .build()
            .unwrap();

        // a counter for counting FPS
        let mut frame_counter: f64 = 0.0;
        let mut start = Instant::now();
        let mut last_draw = Instant::now();

        if self.setup(&mut canvas) {
            event_loop.run(move |event, _, control_flow| {
                // Handle events for closing screen and redrawing it
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id,
                    } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                    Event::RedrawRequested(_windowid) => {
                        frame_counter += 1.0;
                        // Update the frame.
                        let elapsed_time = Instant::now().duration_since(last_draw);
                        last_draw = Instant::now();
                        if !self.update(&mut canvas, &input, elapsed_time.as_secs_f32()) {
                            *control_flow = ControlFlow::Exit;
                        }
                        draw_pixels(pixelbuffer.get_frame(), &canvas);
                        // Paint pixels to screen
                        if pixelbuffer.render().is_err() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    _ => (),
                }
                // Handle mouse input and Keyboard input
                if input.update(&event) {}
                // Update FPS on title screen (cuz why not?)
                let end = std::time::Instant::now();
                let time_from_last_update = end.duration_since(start);

                // Update the title after some resoneable time passes (a few seconds)
                if time_from_last_update > Duration::from_secs(1) {
                    let new_title = format!(
                        "{} - {} FPS",
                        title,
                        (frame_counter / time_from_last_update.as_secs() as f64) as u32
                    );
                    frame_counter = 0.0;
                    window.set_title(&new_title);
                    start = std::time::Instant::now();
                }
                window.request_redraw();
                if *control_flow != ControlFlow::Exit {
                    *control_flow = ControlFlow::Wait;
                }
            });
        } else {
            println!("Could no start application. Setup did not complete successfully");
        }
    }

    /// Static function to render pixels arbitrarily of existence of canvas or not
    /// Simply give a height, width and pixel buffer with the right number of pixels.
    ///
    /// This is useful when rendering  things that are not necessarily drawn by us.
    /// Example: png decoded data.__rust_force_expr!
    ///
    /// see grfx::image::png::PNGImage#pixels
    fn render_pixels(width: u32, height: u32, pixels: Vec<Color>) {
        let event_loop = EventLoop::new();
        let image_size = LogicalSize::new(width, height);
        let window = WindowBuilder::new()
            .with_inner_size(image_size)
            .with_title("Pixel Rendering")
            .with_resizable(true)
            .build(&event_loop)
            .unwrap();

        // Initialize a surface
        let surface_texture = SurfaceTexture::new(width, height, &window);

        // pixesl which we use to draw on the screen
        // let mut pixelbuffer = Pixels::new(self.width, self.height, surface_texture).unwrap();
        let mut pixelbuffer = PixelsBuilder::new(width, height, surface_texture)
            .enable_vsync(true)
            .build()
            .unwrap();

        // copy pixels to buffer to display them
        for (i, chunk) in pixelbuffer.get_frame().chunks_exact_mut(4).enumerate() {
            chunk.copy_from_slice(&pixels[i].as_bytes());
        }
        pixelbuffer.render().expect("Not able to render picture");

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                _ => (),
            }
            if *control_flow != ControlFlow::Exit {
                *control_flow = ControlFlow::Wait;
            }
        });
    }
}
// simply put pixels on the frame
fn draw_pixels(frames: &mut [u8], canvas: &Canvas) {
    for (i, pixel) in frames.chunks_exact_mut(4).enumerate() {
        pixel.copy_from_slice(&canvas.pixels[i].as_bytes());
    }
}

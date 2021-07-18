use crate::grfx::canvas::Canvas;
use crate::grfx::color;
pub use crate::grfx::render::Render2D;
use crate::grfx::shape::Polygon;
use crate::math::vector::AngleTrait;
use crate::math::vector::FVec2D;
use crate::math::vector::Point2D;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Draw2D {
    width: u32,
    height: u32,
    title: String,
    polygon: Option<Polygon>,
}

impl Draw2D {
    pub fn new(width: u32, height: u32, title: String) -> Self {
        Draw2D {
            width,
            height,
            title,
            polygon: None,
        }
    }
}

impl Render2D for Draw2D {
    ///  Get window properties height
    fn height(&mut self) -> u32 {
        self.height
    }
    /// Get wWindow properties width
    fn width(&mut self) -> u32 {
        self.width
    }
    /// Get wWindow properties title
    fn title(&mut self) -> String {
        self.title.clone()
    }
    ///
    /// Setup method called when the world is first created
    /// Must be overriden.
    ///
    fn setup(&mut self, _: &mut Canvas) -> bool {
        self.polygon = Some(Polygon::new(Point2D::new(50, 50), 3, 50, 0.0));
        true
    }

    /// Update method called when the canvas is to be updated
    /// This is called periodically per frame and each frame is drawn individually
    /// Must be overriden/implmented
    fn update(&mut self, canvas: &mut Canvas, input: &WinitInputHelper, _delta_t: f32) -> bool {
        canvas.fill(color::BLUE);
        if let Some(polygon) = &mut self.polygon {
            canvas.polygon(
                polygon.origin,
                polygon.sides,
                polygon.side_length,
                color::CYAN,
                Some(polygon.angle),
            );
            canvas.fill_circle(Point2D::new(100, 100), 30, color::Color::rgba(255, 0, 0, 0));
            if input.key_pressed(VirtualKeyCode::W) {
                polygon.origin.y -= 10;
            }
            if input.key_pressed(VirtualKeyCode::S) {
                polygon.origin.y += 10;
            }
            if input.key_pressed(VirtualKeyCode::A) {
                polygon.origin.x -= 10;
            }
            if input.key_pressed(VirtualKeyCode::D) {
                polygon.origin.x += 10;
            }
            if input.key_pressed(VirtualKeyCode::Left) {
                polygon.angle += std::f32::consts::FRAC_PI_4;
            }
            if input.key_pressed(VirtualKeyCode::Right) {
                polygon.angle -= std::f32::consts::FRAC_PI_4;
            }

            if input.mouse_held(1) {
                if let Some((x, y)) = input.mouse() {
                    let direction = (polygon.origin.to_f32() - FVec2D::new(x, y)).angle();
                    polygon.angle = direction;
                }
            }
        }
        true
    }
}

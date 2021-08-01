// use graphics::grfx::Canvas;
use graphics::grfx::canvas::Canvas;
use graphics::grfx::color::Color;
use graphics::grfx::render::InputHelper;
use graphics::grfx::render::Render2D;
use graphics::math::Point2D;
fn main() {
    let graphics = Graphics;
    graphics.render();
}
pub struct Graphics;

impl Render2D for Graphics {
    fn setup(&mut self, canvas: &mut Canvas) -> bool {
        canvas.fill(Color::BLUE);
        canvas.draw_string(
            Point2D::new(200, 200),
            "See Examples!".into(),
            0.4,
            Color::GRAY,
        );
        true
    }
    fn update(&mut self, _canvas: &mut Canvas, _input: &InputHelper, _delta_t: f32) -> bool {
        true
    }
}

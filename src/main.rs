// use graphics::Canvas;
use graphics::canvas::Canvas;
use graphics::color::Color;
use graphics::math::Point2D;
use graphics::render::Render2D;
fn main() {
    let graphics = Graphics;
    graphics.render();
}
pub struct Graphics;

impl Render2D for Graphics {
    fn setup(&mut self, canvas: &mut Canvas) -> bool {
        // canvas.fill(Color::BLACK);
        canvas.fill_triangle(
            Point2D::new(100, 100),
            Point2D::new(300, 100),
            Point2D::new(150, 250),
            Color::WHITE,
        );
        canvas.draw_string(
            Point2D::new(100, 200),
            "See Examples".into(),
            0.7,
            Color::GRAY,
        );
        true
    }
}

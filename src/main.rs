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
        canvas.fill_triangle(
            Point2D::new(500, 500),
            Point2D::new(100, 100),
            Point2D::new(100, 500),
            Color::MAGENTA,
        );
        canvas.draw_string(
            Point2D::new(250, 200),
            "See Examples folder".into(),
            0.4,
            Color::GRAY,
        );
        true
    }
}

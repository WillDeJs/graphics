// use graphics::grfx::Canvas;
use graphics::grfx::canvas::Canvas;
use graphics::grfx::color::Color;
use graphics::grfx::render::InputHelper;
use graphics::grfx::render::Render2D;
use graphics::math::Point2D;
fn main() {
    let graphics = Graphics::new(400, 400, "Test Window - See Example".into());
    graphics.render();
}
pub struct Graphics {
    width: u32,
    height: u32,
    title: String,
}

impl Graphics {
    pub fn new(width: u32, height: u32, title: String) -> Self {
        Graphics {
            width,
            height,
            title,
        }
    }
}
impl Render2D for Graphics {
    fn width(&mut self) -> u32 {
        self.width
    }
    fn height(&mut self) -> u32 {
        self.height
    }
    fn title(&mut self) -> String {
        self.title.clone()
    }
    fn setup(&mut self, canvas: &mut Canvas) -> bool {
        canvas.fill(Color::BLUE);
        canvas.draw_string(
            Point2D::new(80, 200),
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

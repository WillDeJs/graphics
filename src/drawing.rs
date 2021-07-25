use crate::grfx::canvas::Canvas;
use crate::grfx::canvas::Transform;
use crate::grfx::canvas::Transformer;
use crate::grfx::color;
use crate::grfx::image::imageutils::*;
pub use crate::grfx::render::Render2D;

use winit_input_helper::WinitInputHelper;

pub struct Draw2D {
    width: u32,
    height: u32,
    title: String,
    tiles: Vec<Sprite>,
}

impl Draw2D {
    pub fn new(width: u32, height: u32, title: String, tiles: Vec<Sprite>) -> Self {
        Draw2D {
            width,
            height,
            title,
            tiles,
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
    fn setup(&mut self, canvas: &mut Canvas) -> bool {
        // draw first pass transformed to small no rotation
        canvas.fill(color::Color::rgb(80, 0, 40));
        let mut transformer = Transformer::new();
        transformer.add(Transform::Scale(0.3, 0.3));
        transformer.add(Transform::Translate(100.0, 100.0));
        canvas.transform_sprite(&self.tiles[0], &transformer);

        // draw again transformed to slightly bigger with rotation
        transformer.clear();
        transformer.add(Transform::Scale(0.5, 0.5));
        transformer.add(Transform::Translate(200.0, 200.0));
        transformer.add(Transform::Rotate(0.2));
        canvas.transform_sprite(&self.tiles[0], &transformer);
        true
    }

    /// Update method called when the canvas is to be updated
    /// This is called periodically per frame and each frame is drawn individually
    /// Must be overriden/implmented
    fn update(&mut self, _: &mut Canvas, _: &WinitInputHelper, _delta_t: f32) -> bool {
        true
    }
}

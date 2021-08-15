use graphics::grfx::canvas::Canvas;
use graphics::grfx::canvas::Transform;
use graphics::grfx::canvas::Transformer;
use graphics::grfx::color;
use graphics::grfx::image::imageutils::SpriteExtractor;
use graphics::grfx::image::imageutils::SpriteSize;
use graphics::grfx::image::imageutils::*;
use graphics::grfx::image::png::PNGImage;
use graphics::grfx::render::*;
use graphics::math::Point2D;

fn main() {
    let drawing_canvas = Draw2D::new(800, 600, "Tiles".into());
    drawing_canvas.render();
}

pub struct Draw2D {
    width: u32,
    height: u32,
    title: String,
    tile: Sprite,
    angle: f32,
}

impl Draw2D {
    pub fn new(width: u32, height: u32, title: String) -> Self {
        Draw2D {
            width,
            height,
            title,
            tile: Sprite::default(),
            angle: 0.0,
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
    fn setup(&mut self, _canvas: &mut Canvas) -> bool {
        let image = PNGImage::from_file("../assets/sample.png").unwrap();
        let extractor = SpriteExtractor::from_png(&image, SpriteSize::default(), 0, 0).unwrap();
        self.tile = extractor.extract_whole();
        true
    }

    /// Update method called when the canvas is to be updated
    /// This is called periodically per frame and each frame is drawn individually
    /// Must be overriden/implmented
    fn update(&mut self, canvas: &mut Canvas, _input: &WinitInputHelper, _delta_t: f32) -> bool {
        // clear screen
        canvas.fill(color::Color::rgb(255, 217, 217));
        canvas.draw_string(
            Point2D::new(10, 10),
            "Rotating Sample.png".into(),
            0.20,
            color::Color::rgb(231, 150, 0),
        );
        let mut transformer = Transformer::new();
        //rotate center of sprite to origin, makes easier to rotate later on.
        transformer.add(Transform::Translate(
            (-(self.tile.width as i32) / 2) as f32,
            (-(self.tile.height as i32) / 2) as f32,
        ));
        transformer.add(Transform::Rotate(self.angle));
        transformer.add(Transform::Scale(0.2, 0.2)); // scale down at 30% of size
        transformer.add(Transform::Translate(
            (self.width() / 2) as f32,
            (self.height() / 2) as f32,
        ));

        // Keep turning it on the screen
        if self.angle >= 6.28 {
            self.angle = 0.0;
        }
        self.angle += 0.01;

        // draw sprite
        canvas.transform_sprite(&self.tile, &transformer);
        true
    }
}

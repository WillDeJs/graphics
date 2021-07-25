use graphics::drawing::*;
// use graphics::grfx::image::imageutils::Sprite;
use graphics::grfx::image::imageutils::SpriteExtractor;
use graphics::grfx::image::imageutils::SpriteSize;
use graphics::grfx::image::png::PNGImage;
// use graphics::math::vector::Point2D;

fn main() {
    let image = PNGImage::from_file("sample.png").unwrap();

    let extractor = SpriteExtractor::from_png(&image, SpriteSize::default(), 0).unwrap();
    let tree = extractor.extract_whole();
    let drawing_canvas = Draw2D::new(800, 600, "Tiles".into(), vec![tree]);
    drawing_canvas.render();
}

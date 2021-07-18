use graphics::drawing::*;
use graphics::grfx::image::png::PNGImage;

fn main() {
    let image = PNGImage::from_file("sample.png").unwrap();
    let pixels = image.image_pixels().unwrap();
    Draw2D::render_pixels(image.width(), image.height(), pixels);
}

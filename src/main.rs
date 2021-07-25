use graphics::drawing::{Draw2D, Render2D};

fn main() {
    let drawing_canvas = Draw2D::new(800, 600, "Tiles".into());
    drawing_canvas.render();
}

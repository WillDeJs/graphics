use crate::grfx::color::Color;
use crate::grfx::image::imageutils::Sprite;
use crate::grfx::image::imageutils::SpriteExtractor;
use crate::grfx::image::imageutils::SpriteSize;
use crate::grfx::image::png::PNGImage;
use crate::math;
use crate::math::FVec2D;
use crate::math::FVec3D;
use crate::math::Mat3x3;
use crate::math::Point2D;
use std::collections::HashMap;

/// Font letters and symbols.
// const FONT_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 .,;#$&()?[]}{@*!''";
pub const FONT_LETTERS: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[derive(Debug, Copy, Clone)]
pub enum Transform {
    Rotate(f32),
    Scale(f32, f32),
    Translate(f32, f32),
}
#[allow(dead_code, unused_variables)]
pub struct Transformer {
    transforms: std::collections::VecDeque<Transform>,
}

impl Transformer {
    pub fn new() -> Self {
        Self {
            transforms: std::collections::VecDeque::<Transform>::new(),
        }
    }

    pub fn add(&mut self, item: Transform) {
        self.transforms.push_front(item);
    }
    pub fn remove(&mut self, index: usize) -> Option<Transform> {
        self.transforms.remove(index)
    }

    pub fn clear(&mut self) {
        self.transforms.clear();
    }
    pub fn all(&self) -> Vec<Transform> {
        (&self.transforms)
            .into_iter()
            .map(|el| *el)
            .collect::<Vec<Transform>>()
    }
    pub fn count(&self) -> usize {
        self.transforms.len()
    }
}

#[allow(dead_code, unused_variables)]
pub struct Canvas {
    width: u32,
    height: u32,
    font: Option<HashMap<char, Sprite>>,
    pub pixels: std::cell::RefCell<Vec<Color>>,
}
impl Canvas {
    /// Create a new canvas with the given dimensions
    pub fn new(width: u32, height: u32) -> Self {
        let mut pixels = Vec::with_capacity((width * height) as usize);
        for _ in 0..pixels.capacity() {
            pixels.push(Color::BLACK); // initialize to black pixels;
        }
        let font = read_font(); // load font into memory
        Self {
            width,
            height,
            font,
            pixels: std::cell::RefCell::new(pixels),
        }
    }

    /// Clear the canvas by filling it with a given color
    pub fn fill(&self, color: Color) {
        for pixel in &mut *self.pixels.borrow_mut() {
            *pixel = color;
        }
    }
    /// Retrieve canvas width
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Retrieve canvas height
    pub fn height(&self) -> u32 {
        self.height
    }

    ///
    ///  Plots a single pixel at the given coordinates
    ///     x  -> X axis offset
    ///     y  -> y axis offset
    ///     color -> pixel color
    ///
    pub fn plot(&self, x: i32, y: i32, color: Color) {
        // Don't paint transparent pixels
        if color.alpha() == 0 {
            return;
        }
        let pixel_length = self.width() * self.height();
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            // let normalized_position = (y * self.width as i32 + x) as usize;
            // reverse y location  as glium texture starts bottom left as origing
            let normalized_position = ((self.height as i32 - y) * self.width as i32 + x) as usize;
            if normalized_position < pixel_length as usize {
                self.pixels.borrow_mut()[normalized_position] = color;
            }
        }
    }

    pub fn line(&self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let origin = Point2D::new(x0, y0);
        let end = Point2D::new(x1, y1);

        self.line_between(origin, end, color);
    }
    ///
    /// Draws a line using Bresenham Algorthm
    /// origin: start point
    /// dest: final point
    /// color: Pixel color
    ///
    /// <https://www.programmersought.com/article/60715259426/>
    pub fn line_between(&self, origin: Point2D, dest: Point2D, color: Color) {
        let x0 = origin.x;
        let y0 = origin.y;

        let x1 = dest.x;
        let y1 = dest.y;

        let dx = if x1 >= x0 { x1 - x0 } else { x0 - x1 };
        let dy = if x1 >= x0 { y1 - y0 } else { y0 - y1 };

        // ensure we go from smaller to highest
        let mut x = if x1 >= x0 { x0 } else { x1 };
        let mut y = if x1 >= x0 { y0 } else { y1 };

        // veritical line
        if dx == 0 {
            for i in y0.min(y1)..y0.max(y1) {
                self.plot(x, i, color);
            }
            return;
        }
        // horizontal line
        if dy == 0 {
            for i in x0.min(x1)..x0.max(x1) {
                self.plot(i, y, color);
            }
            return;
        }
        // slope is less than 1
        if dy.abs() <= dx {
            let mut decision = 2 * dy.abs() - dx;
            self.plot(x, y, color);
            while x < x0.max(x1) {
                x += 1;
                if decision < 0 {
                    decision = decision + 2 * dy.abs();
                } else {
                    y += if dy >= 0 { 1 } else { -1 };
                    decision = decision + 2 * (dy.abs() - dx);
                }
                self.plot(x, y, color);
            }
        } else {
            // slopw is > 1 and dy positions are swapped
            let mut decision = 2 * dx - dy.abs();
            self.plot(x, y, color);
            while x < x0.max(x1) {
                y += if dy >= 0 { 1 } else { -1 };
                if decision < 0 {
                    decision = decision + 2 * dx;
                } else {
                    x += 1;
                    decision = decision + 2 * (dx - dy.abs());
                }
                self.plot(x, y, color);
            }
        }
    }

    ///
    /// Draws a hollow circle using Bresenham Algortim for circles
    /// origin: Center of circle
    /// Radius: Radius
    /// color: Pixel color
    /// <https://iq.opengenus.org/bresenhams-circle-drawing-algorithm/>
    ///
    pub fn circle(&self, origin: Point2D, radius: i32, color: Color) {
        let mut x = 0;
        let mut y = radius;
        let mut decision = 3 - 2 * y;
        while y >= x {
            self.plot(origin.x + x, origin.y + y, color);
            self.plot(origin.x - x, origin.y + y, color);
            self.plot(origin.x + x, origin.y - y, color);
            self.plot(origin.x - x, origin.y - y, color);
            self.plot(origin.x + y, origin.y + x, color);
            self.plot(origin.x - y, origin.y + x, color);
            self.plot(origin.x + y, origin.y - x, color);
            self.plot(origin.x - y, origin.y - x, color);

            x += 1;
            if decision > 0 {
                y -= 1;
                decision = decision + 4 * (x - y) + 10;
            } else {
                decision = decision + 4 * x + 6;
            }

            // break;
        }
    }

    ///
    /// Draws a hollow rectangle
    /// Takes:
    /// origin: toip left corner
    /// Width
    /// Height
    /// Color for pixels
    ///
    pub fn rectangle(&self, origin: Point2D, width: i32, height: i32, color: Color) {
        let top_right = Point2D::new(origin.x + width, origin.y);
        let bottom_left = Point2D::new(origin.x, origin.y + height);
        let bottom_right = Point2D::new(origin.x + width, origin.y + height);

        self.line_between(origin, top_right, color);
        self.line_between(origin, bottom_left, color);
        self.line_between(bottom_left, bottom_right, color);
        self.line_between(top_right, bottom_right, color);
    }

    ///
    /// Draws a filled rectangle
    /// Takes:
    /// origin: toip left corner
    /// Width
    /// Height
    /// Color for pixels
    ///
    pub fn fill_rectangle(&self, origin: &Point2D, width: i32, height: i32, color: Color) {
        for i in 0..=height {
            let right = Point2D::new(origin.x, origin.y + i);
            let left = Point2D::new(origin.x + width, origin.y + i);
            self.line_between(right, left, color);
        }
    }

    ///
    /// Draws a hollow triangle
    /// Takes:
    /// v1 first point
    /// v2 second point
    /// v3 third point
    /// color: Color for the pixels
    ///
    pub fn triangle(&self, v1: Point2D, v2: Point2D, v3: Point2D, color: Color) {
        self.line_between(v1, v2, color);
        self.line_between(v2, v3, color);
        self.line_between(v3, v1, color);
    }

    ///
    /// Draw a given polygone based on the given vertices/points vector
    /// vertices -> Points to connect
    /// color -> color to paint them
    pub fn connect_points(&self, vertices: &Vec<Point2D>, color: Color) {
        let len = vertices.len();
        if len >= 3 {
            let first = vertices[0];
            let last = vertices[len - 1];
            for i in 1..len {
                self.line_between(vertices[i - 1], vertices[i], color);
            }
            self.line_between(last, first, color);
        }
    }

    ///
    /// Draw a regular polygone based on the given start point, number of sides and length
    /// origin -> First point on the polygon
    /// sides -> Number of sides on the polygon
    /// length -> Length of each side of the polygon
    /// angle -> Optional angle (in degrees) given to start position of polygon
    /// color -> Line color
    ///
    /// Not sure if there is a more efficient way of doing this but its similar to to the process of drawing a traingle.
    /// In this case we simply need to know that the exterior angles of a polygon are always 360 degrees or 2PI.
    ///
    /// We create a vector that runs from one point to the next.
    ///     To determine what the next point is we use the angle for each side (360/sides) and increment accordingly
    /// Then we simply draw a line between the previous point and the next one as they will always allign to close the polygon
    ///  due to the angle calculation.
    ///             
    /// <html>
    /// <pre>
    ///   O ->   ******
    ///       *         *  <-- a
    ///      *            *
    /// F->  *            *   <---b
    ///      *            *
    /// e-->   *        *   <--- c
    ///         ******
    ///             ^
    ///              d
    /// </pre>
    ///    In this example we have a polygon on which each line differs from the previous in exactly 60 degrees (pi/3)
    /// <ul>
    ///     <li>Point O is at 0 degress</li>
    ///     <li>Point a is at 60 degress</li>
    ///     <li>Point b is at 120 degress</li>
    ///     <li>Point c is at 180 degress</li>
    ///     <li>Point d is at 240 degress</li>
    ///     <li>Point e is at 300 degress</li>
    ///     <li>Point f is at 360 degress</li>
    /// </ul>
    /// <html>
    pub fn polygon(
        &self,
        origin: Point2D,
        sides: i32,
        length: i32,
        color: Color,
        angle: Option<f32>,
    ) {
        let delta_angle = 2.0 * std::f32::consts::PI / sides as f32;
        let mut current_angle = match angle {
            Some(value) => value * std::f32::consts::PI / 180.0,
            None => 2.0 * std::f32::consts::PI,
        };
        let mut start = origin;
        let mut next: Point2D;

        for _ in 0..sides {
            next = start + Point2D::from_polar(length, current_angle);
            current_angle -= delta_angle;
            self.line_between(start, next, color);
            start = next;
        }
        self.fill_circle(origin, 5, Color::BLUE);
    }

    ///
    /// Draws a filled circle
    /// Takes:
    /// origin: toip left corner
    /// Width
    /// Height
    /// Color for pixels
    ///  Helpful material to get this working:
    /// <https://iq.opengenus.org/bresenhams-circle-drawing-algorithm/>
    /// <https://stackoverflow.com/questions/1201200/fast-algorithm-for-drawing-filled-circles>
    /// <https://github.com/OneLoneCoder/olcPixelGameEngine/blob/master/olcPixelGameEngine.h>  (Javidx9  github)
    ///
    pub fn fill_circle(&self, origin: Point2D, radius: i32, color: Color) {
        let mut x = 0;
        let mut y = radius;
        let mut decision = 3 - 2 * y;
        while y >= x {
            self.line(
                origin.x + x,
                origin.y - y,
                origin.x + x,
                origin.y + y,
                color,
            );
            self.line(
                origin.x - x,
                origin.y - y,
                origin.x - x,
                origin.y + y,
                color,
            );
            self.line(
                origin.x - y,
                origin.y - x,
                origin.x + y,
                origin.y - x,
                color,
            );
            self.line(
                origin.x - y,
                origin.y + x,
                origin.x + y,
                origin.y + x,
                color,
            );

            x += 1;
            if decision > 0 {
                y -= 1;
                decision = decision + 4 * (x - y) + 10;
            } else {
                decision = decision + 4 * x + 6;
            }
        }
    }

    ///
    /// Draws a filled  triangle
    /// Takes:
    /// v1 first point
    /// v2 second point
    /// v3 third point
    /// color: Color for the pixels
    ///
    /// Uses scan line algorithm: <https://www.avrfreaks.net/sites/default/files/triangles.c>
    ///
    pub fn fill_triangle(&self, v1: Point2D, v2: Point2D, v3: Point2D, color: Color) {
        let mut a: i32;
        let mut b: i32;
        let last: i32;
        let y: i32;
        let mut x0 = v1.x;
        let mut y0 = v1.y;

        let mut x1 = v2.x;
        let mut y1 = v2.y;

        let mut x2 = v3.x;
        let mut y2 = v3.y;

        // Sort coordinates by Y order (y2 >= y1 >= y0)
        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1);
            std::mem::swap(&mut x0, &mut x1);
        }
        if y1 > y2 {
            std::mem::swap(&mut y2, &mut y1);
            std::mem::swap(&mut x2, &mut x1);
        }
        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1);
            std::mem::swap(&mut x0, &mut x1);
        }

        let smallest_x = math::min(x0, math::min(x1, x2));
        let biggest_x = math::max(x0, math::max(x1, x2));
        let h_line_plot = |a: i32, b: i32, y: i32| {
            for i in math::min(a, b)..math::max(a, b) {
                if i >= smallest_x && i <= biggest_x {
                    self.plot(i, y, color);
                }
            }
        };
        if y0 == y2 {
            // All on same line case
            a = x0;
            b = x0;
            if x1 < a {
                a = x1;
            } else if x1 > b {
                b = x1;
            }
            if x2 < a {
                a = x2;
            } else if x2 > b {
                b = x2;
            }
            // self.line(a, y0, b, y0, color);
            h_line_plot(a, b, y0);
            return;
        }
        let dx01 = x1 - x0;
        let dy01 = y1 - y0;
        let dx02 = x2 - x0;
        let dy02 = y2 - y0;
        let dx12 = x2 - x1;
        let dy12 = y2 - y1;
        let mut sa = 0;
        let mut sb = 0;
        // For upper part of triangle, find scanline crossings for segment
        // 0-1 and 0-2.  If y1=y2 (flat-bottomed triangle), the scanline y
        // is included here (and second loop will be skipped, avoiding a /
        // error there), otherwise scanline y1 is skipped here and handle
        // in the second loop...which also avoids a /0 error here if y0=y
        // (flat-topped triangle)
        if y1 == y2 {
            last = y1;
        }
        // Include y1 scanline
        else {
            last = y1 - 1;
        } // Skip it
        for y in y0..=last {
            if dy01 != 0 && dy02 != 0 {
                a = x0 + sa / dy01;
                b = x0 + sb / dy02;
                sa += dx01;
                sb += dx02;
                // longhand a = x0 + (x1 - x0) * (y - y0) / (y1 - y0)
                //          b = x0 + (x2 - x0) * (y - y0) / (y2 - y0)
                // self.line(a, y, b, y, color);
                h_line_plot(a, b, y);
            }
        }

        // pick up where we left off
        y = last;
        // For lower part of triangle, find scanline crossings for segment
        // 0-2 and 1-2.  This loop is skipped if y1=y2
        sa = dx12 * (y - y1);
        sb = dx02 * (y - y0);
        for i in y..=y2 {
            if dy12 != 0 && dy02 != 0 {
                a = x1 + sa / dy12;
                b = x0 + sb / dy02;
                sa += dx12;
                sb += dx02;
                // longhand a = x1 + (x2 - x1) * (y - y1) / (y2 - y1)
                //          b = x0 + (x2 - x0) * (y - y0) / (y2 - y0)
                h_line_plot(a, b, i);
                // self.line(a, y, b, y, color);
            }
        }
    }

    pub fn sprite(&self, origin: Point2D, tile: &Sprite) {
        for (i, pixel) in tile.pixels.iter().enumerate() {
            let x = origin.x() + (i % tile.width) as i32;
            let y = origin.y() + (i / tile.width) as i32;
            self.plot(x, y, *pixel);
        }
    }
    pub fn transform_sprite(&self, tile: &Sprite, transformer: &Transformer) {
        self.transform_sprite_colored(tile, transformer, None);
    }

    pub fn transform_sprite_colored(
        &self,
        tile: &Sprite,
        transformer: &Transformer,
        color: Option<Color>,
    ) {
        let mut transformed = Mat3x3::<f32>::identity();

        // Calculate all transforms, notice they are stored in reversed order since
        // the transform first added to the list will be the last operation
        // transforms are combined by multiplying their transform matrixes
        // the transformed there will contain the result of all matrix multiplications
        for transform in transformer.all() {
            transformed = transformed
                * match transform {
                    Transform::Rotate(angle) => Mat3x3::<f32>::rotate(angle),
                    Transform::Scale(cx, cy) => Mat3x3::<f32>::scale(cx, cy),
                    Transform::Translate(cx, cy) => Mat3x3::<f32>::translate(cx, cy),
                };
        }
        let inversed_transformed = transformed.inverse();

        // get corners of untransformed sprite
        // I could use Vec2D here since the z component is 1.0 but being able to not skipp a dimension
        // and use a 3D Vector with a 3x3 matrix just makes the concept more understandable
        let tl_corner = FVec3D::new(0.0, 0.0, 1.0); // top left corner
        let tr_corner = FVec3D::new(tile.width as f32, 0.0, 1.0); // top right corner
        let br_corner = FVec3D::new(tile.width as f32, tile.height as f32, 1.0); // right bottom corner
        let bl_corner = FVec3D::new(0.0, tile.height as f32, 1.0); // left bottom corner

        // get corners of transformed sprite
        let tl_transformed = transformed.transform_point(tl_corner);
        let tr_transformed = transformed.transform_point(tr_corner);
        let bl_transformed = transformed.transform_point(bl_corner);
        let br_transformed = transformed.transform_point(br_corner);

        // get bounding box coordinates of transformed box

        let mut sx = math::min(tl_transformed.x(), br_transformed.x());
        let mut sy = math::min(tl_transformed.y(), br_transformed.y());

        let mut ex = math::max(tl_transformed.x(), br_transformed.x());
        let mut ey = math::max(tl_transformed.y(), br_transformed.y());

        ex = math::max(ex, bl_transformed.x());
        ey = math::max(ey, bl_transformed.y());
        sx = math::min(sx, bl_transformed.x());
        sy = math::min(sy, bl_transformed.y());

        ex = math::max(ex, tr_transformed.x());
        ey = math::max(ey, tr_transformed.y());
        sx = math::min(sx, tr_transformed.x());
        sy = math::min(sy, tr_transformed.y());

        for x in sx as usize..ex as usize {
            for y in sy as usize..ey as usize {
                let new_point =
                    inversed_transformed.transform_point(FVec3D::new(x as f32, y as f32, 1.0));
                if let Some(pixel) = tile.get_pixel(
                    (new_point.x() + 0.5) as usize,
                    (new_point.y() + 0.5) as usize,
                ) {
                    if let Some(override_color) = color {
                        if pixel.alpha() != 0 {
                            self.plot(x as i32, y as i32, override_color);
                        }
                    } else {
                        self.plot(x as i32, y as i32, pixel);
                    }
                }
            }
        }
    }

    pub fn draw_string(&self, origin: Point2D, msg: String, size: f32, color: Color) {
        if let Some(font) = &self.font {
            let mut width = 0.0;
            let mut translate_point = origin.to_f32();
            for character in msg.chars() {
                if let Some(sprite) = font.get(&character) {
                    width = sprite.width as f32 * size;
                    let mut transformer = Transformer::new();
                    transformer.add(Transform::Scale(size, size));
                    transformer.add(Transform::Translate(
                        translate_point.x(),
                        translate_point.y(),
                    ));
                    self.transform_sprite_colored(&sprite, &transformer, Some(color));
                }
                translate_point = FVec2D::new(translate_point.x() + width, translate_point.y());
            }
        }
    }
}

/// Helper read all fonts into statuc FONT_SYMBOLS for later usage.
fn read_font() -> Option<HashMap<char, Sprite>> {
    let mut font_map = HashMap::<char, Sprite>::new();

    match PNGImage::from_file("./assets/font2.png") {
        Ok(image) => {
            let extractor =
                SpriteExtractor::from_png(&image, SpriteSize::new(50, 85), 0, 15).unwrap();
            let symbols: Vec<Sprite> = extractor.collect();
            for (index, character) in FONT_LETTERS.chars().enumerate() {
                if symbols.len() > index {
                    font_map.insert(character, symbols[index].clone());
                }
            }
        }
        Err(_) => return None,
    }
    Some(font_map)
}

impl<'a> glium::texture::Texture2dDataSource<'a> for &'a Canvas {
    type Data = u8;
    fn into_raw(self) -> glium::texture::RawImage2d<'a, Self::Data> {
        let length = (self.width * self.height) as usize;

        // let mut data: Vec<u8> = Vec::with_capacity(length * 4);
        // for pixel in &*self.pixels.borrow() {
        //     data.extend_from_slice(&pixel.as_bytes());
        // }
        glium::texture::RawImage2d {
            // use unsafe borrow as the alterantive would be to duplicate the buffer and occupy the processor copying it's data
            data: std::borrow::Cow::Borrowed(unsafe {
                std::slice::from_raw_parts(self.pixels.borrow().as_ptr() as *const u8, length * 4)
            }),
            // data: std::borrow::Cow::Owned(data),
            height: self.height,
            width: self.width,
            format: glium::texture::ClientFormat::U8U8U8U8,
        }
    }
}

use crate::grfx::color::Color;
use crate::grfx::image::png::PNGImage;
use crate::math::Point2D;
use std::error::Error;

#[derive(Debug, Default, Clone)]
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Sprite {
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if x >= self.width || y >= self.height {
            None
        } else {
            let normalized_position = y * self.width + x;
            match self.pixels.get(normalized_position) {
                Some(&color) => Some(color),
                None => None,
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SpriteSize {
    pub width: usize,
    pub height: usize,
}
impl SpriteSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SpriteExtractor {
    image_width: usize,
    image_height: usize,
    tile_size: SpriteSize,
    separation: usize,
    pixels: Vec<Color>,
    start: Point2D,
}
impl SpriteExtractor {
    pub fn new(
        image_width: usize,
        image_height: usize,
        tile_size: SpriteSize,
        separation: usize,
        pixels: Vec<Color>,
    ) -> Self {
        SpriteExtractor {
            image_width,
            image_height,
            tile_size,
            separation,
            pixels,
            start: Point2D::default(),
        }
    }

    pub fn from_png(
        image: &PNGImage,
        tile_size: SpriteSize,
        separation: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let pixels = image.pixels()?;
        let image_width = image.width() as usize;
        let image_height = image.height() as usize;

        Ok(SpriteExtractor {
            image_width,
            image_height,
            pixels,
            tile_size,
            separation,
            start: Point2D::default(),
        })
    }

    fn extract_pixels(&mut self, x: usize, y: usize, length: usize) -> Option<&[Color]> {
        let start_x = y * self.image_width as usize + x;
        let image_size = (self.image_height * self.image_width) as usize;
        if start_x + length < image_size {
            Some(&self.pixels[start_x..start_x + length])
        } else {
            None
        }
    }

    pub fn extract_sprite(&mut self, start: Point2D, size: SpriteSize) -> Option<Sprite> {
        let mut pixels = Vec::<Color>::with_capacity(size.width * size.height);
        if start.x() as usize + size.width < self.image_width
            && start.y() as usize + size.height < self.image_height
        {
            for i in 0..size.height {
                if let Some(colors) =
                    self.extract_pixels(start.x() as usize, start.y() as usize + i, size.width)
                {
                    pixels.extend_from_slice(colors);
                }
            }
            Some(Sprite {
                width: size.width,
                height: size.height,
                pixels,
            })
        } else {
            None
        }
    }

    pub fn extract_whole(&self) -> Sprite {
        Sprite {
            width: self.image_width,
            height: self.image_height,
            pixels: self.pixels.clone(),
        }
    }
}
impl Iterator for SpriteExtractor {
    type Item = Sprite;
    fn next(&mut self) -> Option<Self::Item> {
        let sprite = self.extract_sprite(self.start, self.tile_size.clone());
        if self.start.x() as usize + self.separation + 2 * self.tile_size.width < self.image_width {
            self.start = Point2D::new(
                self.start.x() + (self.separation + self.tile_size.width) as i32,
                self.start.y(),
            );
        } else {
            self.start = Point2D::new(
                0,
                self.start.y() + self.tile_size.height as i32 + self.separation as i32,
            );
        }
        sprite
    }
}

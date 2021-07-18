use crate::math::vector::Point2D;
#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Polygon {
    pub origin: Point2D,
    pub sides: i32,
    pub side_length: i32,
    pub angle: f32, // angle in radians
}

#[allow(unused)]
impl Polygon {
    pub fn new(origin: Point2D, sides: i32, side_length: i32, angle: f32) -> Self {
        Self {
            origin,
            sides,
            side_length,
            angle,
        }
    }

    pub fn vertices(&self) -> Vec<Point2D> {
        let mut vertices = Vec::<Point2D>::new();
        let mut start = self.origin;
        let mut current_angle = self.angle;
        let delta_angle = 2.0 * std::f32::consts::PI / self.sides as f32;
        let mut next: Point2D;

        vertices.push(start);
        for _ in 1..self.sides {
            next = start + Point2D::from_polar(self.side_length, current_angle);
            current_angle -= delta_angle;
            vertices.push(next);
            start = next;
        }
        return vertices;
    }
}

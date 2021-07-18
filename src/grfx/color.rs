use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::SubAssign;

/// RGB like color structure
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Color(u8, u8, u8, u8);

/// Handy color definitions
pub const BLACK: Color = Color(0, 0, 0, 255);
pub const BLUE: Color = Color(0, 0, 255, 255);
pub const GREEN: Color = Color(0, 255, 0, 255);
pub const RED: Color = Color(255, 0, 0, 255);
pub const WHITE: Color = Color(255, 255, 255, 255);
pub const YELLOW: Color = Color(255, 255, 0, 255);
pub const MAGENTA: Color = Color(255, 0, 255, 255);
pub const CYAN: Color = Color(0, 255, 255, 255);
pub const GRAY: Color = Color(127, 127, 127, 255);

impl Color {
    /// Create a new color
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b, 255)
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }
    pub fn from_bytes(bytes: &[u8; 4]) -> Self {
        Self(bytes[0], bytes[1], bytes[2], bytes[3])
    }
    pub fn from_slice(bytes: &[u8]) -> Self {
        let mut array: [u8; 4] = [0, 0, 0, 255];

        for (i, byte) in bytes.iter().enumerate() {
            array[i] = *byte;
            if i >= 3 {
                break;
            }
        }
        Self::from_bytes(&array)
    }
    // Retrieve red component
    pub fn r(&self) -> u8 {
        self.0
    }
    // Retrieve green component
    pub fn g(&self) -> u8 {
        self.1
    }
    // Retrieve blue component
    pub fn b(&self) -> u8 {
        self.2
    }
    pub fn alpha(&self) -> u8 {
        self.3
    }
    pub fn to_string(&self) -> String {
        format!("{} {} {}\n", self.0, self.1, self.2)
    }
    /// Liniarly interpolate the color based on the values of a vector given
    pub fn difuse(&self, col: &Color) -> Self {
        Self(
            ((self.0 as f32 / 255.0) * col.0 as f32) as u8,
            ((self.1 as f32 / 255.0) * col.1 as f32) as u8,
            ((self.2 as f32 / 255.0) * col.2 as f32) as u8,
            255, // opacity as max by default
        )
    }
    pub fn set_alpha(&mut self, alpha: u8) {
        self.3 = alpha;
    }
    /// Convert color to array of bytes
    pub fn as_bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

/// Operator +
impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Self::Output {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2, 255)
    }
}
/// Operator -
impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Self::Output {
        Color(self.0 - other.0, self.1 - other.1, self.2 - other.2, 255)
    }
}

/// Operator  +=
impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
        self.3 = 255; // set opasity to max
    }
}

/// Operator  -=
impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
        self.3 = 255; // set opasity to max
    }
}

/// Operator  * (vect * number)
impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, scalar: f32) -> Self::Output {
        Self(
            (self.0 as f32 * scalar) as u8,
            (self.1 as f32 * scalar) as u8,
            (self.2 as f32 * scalar) as u8,
            255,
        ) // alpha/opacity as max
    }
}

/// Operator  * (number * vec3d)
impl Mul<&Color> for f32 {
    type Output = Color;
    fn mul(self, color: &Color) -> Self::Output {
        color.clone() * self
    }
}

/// Operator  * (number * Vec3D) - Consumes/moves the operands
impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, color: Color) -> Self::Output {
        color.clone() * self
    }
}

/// Operator  / (vect / number)
impl Div<f32> for Color {
    type Output = Color;
    fn div(self, scalar: f32) -> Self::Output {
        assert_ne!(scalar, 0.0);
        Self(
            (self.0 as f32 / scalar) as u8,
            (self.1 as f32 / scalar) as u8,
            (self.2 as f32 / scalar) as u8,
            255,
        ) // alpha/opacity as max
    }
}

pub mod matrix;
#[cfg(test)]
mod test;
pub mod vector;

pub use crate::math::matrix::Mat3x3;
pub use crate::math::vector::Vector2D;
pub use crate::math::vector::Vector3D;
pub use std::ops::Add;
pub use std::ops::AddAssign;
pub use std::ops::Div;
pub use std::ops::DivAssign;
pub use std::ops::Mul;
pub use std::ops::MulAssign;
pub use std::ops::Neg;
pub use std::ops::Sub;
pub use std::ops::SubAssign;

/// Integer vector type.
pub type IVec2D = crate::math::vector::Vector2D<i32>;

/// Integer vector type.
pub type Point2D = crate::math::vector::Vector2D<i32>;

/// Floating point vector type
pub type FVec2D = crate::math::vector::Vector2D<f32>;

/// Floating point 3D Vector type
pub type FVec3D = crate::math::vector::Vector3D<f32>;

/// Integer 3D vector type
pub type IVec3D = crate::math::vector::Vector3D<i32>;

pub type FMat3 = crate::math::matrix::Mat3x3<f32>;

/// Determine the minimum among two numbers a and b
pub fn min<T>(a: T, b: T) -> T
where
    T: SimpleMathTrait + PartialEq + PartialOrd,
{
    if a > b {
        b
    } else {
        a
    }
}

/// Determine the maximum among two numbers a and b
pub fn max<T>(a: T, b: T) -> T
where
    T: SimpleMathTrait + PartialEq + PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}
/// Inner trait to implement all operations required for generic vector types.
///  Restricts operations to only implemented primitive types
#[doc(hidden)]
pub trait SimpleMathTrait:
    Add + Div + Mul + Sub + Sized + Copy + MulAssign + DivAssign + SquareRoot + Neg + Zero
{
}

/// By using traits the way we did we now don't have a way to calculate square root
/// We define  our custom SquareRoot trait to use the square root functionality in the float primitives
/// Its not pretty but we avoid using an external crate like num_traits.
pub trait SquareRoot<Rhs = Self> {
    fn sqrt(self) -> Self;
}

// Used to be able to transfor into polar coordinates
pub trait PolarTrait<Rhs = Self>
where
    Self: Sized,
{
    fn from_polar(self, angle: f32) -> (Self, Self);
}

impl SquareRoot for i8 {
    fn sqrt(self) -> Self {
        (self as f32).sqrt() as i8
    }
}
impl SquareRoot for i16 {
    fn sqrt(self) -> Self {
        (self as f32).sqrt() as i16
    }
}
impl SquareRoot for i32 {
    fn sqrt(self) -> Self {
        (self as f32).sqrt() as i32
    }
}
impl SquareRoot for u32 {
    fn sqrt(self) -> Self {
        (self as f32).sqrt() as u32
    }
}
impl SquareRoot for f64 {
    fn sqrt(self) -> Self {
        (self as f64).sqrt()
    }
}
impl SquareRoot for f32 {
    fn sqrt(self) -> Self {
        (self as f32).sqrt()
    }
}

impl PolarTrait for i8 {
    fn from_polar(self, angle: f32) -> (i8, i8) {
        (
            (self as f32 * angle.sin()) as i8,
            (self as f32 * angle.cos()) as i8,
        )
    }
}
impl PolarTrait for i16 {
    fn from_polar(self, angle: f32) -> (i16, i16) {
        (
            (self as f32 * angle.sin()) as i16,
            (self as f32 * angle.cos()) as i16,
        )
    }
}
impl PolarTrait for i32 {
    fn from_polar(self, angle: f32) -> (i32, i32) {
        (
            (self as f32 * angle.sin()) as i32,
            (self as f32 * angle.cos()) as i32,
        )
    }
}
impl PolarTrait for f64 {
    fn from_polar(self, angle: f32) -> (f64, f64) {
        (self * angle.sin() as f64, self * angle.cos() as f64)
    }
}
impl PolarTrait for f32 {
    fn from_polar(self, angle: f32) -> (f32, f32) {
        (self * angle.sin(), self * angle.cos())
    }
}
impl PolarTrait for u32 {
    fn from_polar(self, angle: f32) -> (u32, u32) {
        (
            (self as f32 * angle.sin().abs()) as u32,
            (self as f32 * angle.cos().abs()) as u32,
        )
    }
}

/// Used to be able to calculate the angle/direction
pub trait AngleTrait {
    fn angle(&self) -> f32;
}

/// Trait used to be able to convert by truncation between floatingpoint and integer types
pub trait LossyCast<U> {
    fn cast(self) -> U;
}

impl LossyCast<i32> for f32 {
    fn cast(self) -> i32 {
        self as i32
    }
}
impl LossyCast<i32> for u32 {
    fn cast(self) -> i32 {
        self as i32
    }
}
impl LossyCast<u32> for f32 {
    fn cast(self) -> u32 {
        self.abs() as u32
    }
}
impl LossyCast<i32> for i32 {
    fn cast(self) -> i32 {
        self
    }
}
impl LossyCast<i32> for i64 {
    fn cast(self) -> i32 {
        self as i32
    }
}

impl LossyCast<f32> for f32 {
    fn cast(self) -> f32 {
        self as f32
    }
}
impl LossyCast<f32> for u32 {
    fn cast(self) -> f32 {
        self as f32
    }
}
impl LossyCast<f32> for i32 {
    fn cast(self) -> f32 {
        self as f32
    }
}
impl LossyCast<f32> for i64 {
    fn cast(self) -> f32 {
        self as f32
    }
}

pub trait Zero {
    type Type;
    fn zero() -> Self::Type;
}

impl Zero for u64 {
    type Type = u64;
    fn zero() -> Self::Type {
        0_u64
    }
}

impl Zero for i64 {
    type Type = i64;
    fn zero() -> Self::Type {
        0
    }
}
impl Zero for u32 {
    type Type = u32;
    fn zero() -> Self::Type {
        0
    }
}
impl Zero for i32 {
    type Type = i32;
    fn zero() -> Self::Type {
        0
    }
}
impl Zero for i16 {
    type Type = i16;
    fn zero() -> Self::Type {
        0
    }
}
impl Zero for u16 {
    type Type = u16;
    fn zero() -> Self::Type {
        0
    }
}
impl Zero for u8 {
    type Type = u8;
    fn zero() -> Self::Type {
        0
    }
}

impl Zero for i8 {
    type Type = i8;
    fn zero() -> Self::Type {
        0
    }
}

impl Zero for f32 {
    type Type = f32;
    fn zero() -> Self::Type {
        0.0
    }
}
impl Zero for f64 {
    type Type = f64;
    fn zero() -> Self::Type {
        0.0
    }
}

/// unity trait
///
pub trait Unit {
    type Type;
    fn one() -> Self::Type;
}

impl Unit for u32 {
    type Type = u32;
    fn one() -> Self::Type {
        1
    }
}
impl Unit for i32 {
    type Type = i32;
    fn one() -> Self::Type {
        1
    }
}
impl Unit for u64 {
    type Type = u64;
    fn one() -> Self::Type {
        1
    }
}
impl Unit for i64 {
    type Type = i64;
    fn one() -> Self::Type {
        1
    }
}
impl Unit for f64 {
    type Type = f64;
    fn one() -> Self::Type {
        1.0
    }
}
impl Unit for f32 {
    type Type = f32;
    fn one() -> Self::Type {
        1.0
    }
}

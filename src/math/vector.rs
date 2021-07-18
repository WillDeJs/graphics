use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

/// Inner trait to implement all operations required for generic vector types.
///  Restricts operations to only implemented primitive types
#[doc(hidden)]
pub trait VectorTrait:
    Add + Div + Mul + Sub + Sized + Copy + MulAssign + DivAssign + SquareRoot + Neg
{
}

// types that are allowed to implement vector
impl VectorTrait for i8 {}
impl VectorTrait for i16 {}
impl VectorTrait for i32 {}
impl VectorTrait for f64 {}
impl VectorTrait for f32 {}

/// Integer vector type.
pub type IVec2D = Vector2D<i32>;

/// Integer vector type.
pub type Point2D = Vector2D<i32>;

/// Floating point vector type
pub type FVec2D = Vector2D<f32>;

/// A generic vector type that offers vector operations such as
///     Dot product
///     Cross produt
///     Addition
///     Subtraction
/// Vector of unsigned types are not allowed as we implement Neg Trait
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector2D<T>
where
    T: VectorTrait,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T>
where
    T: VectorTrait
        + Div<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + PolarTrait<T>
        + Neg<Output = T>,
    <T as Mul>::Output: Add<Output = T>,
    <T as Mul>::Output: Sub<Output = T>,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_polar(magnitude: T, radians: f32) -> Self {
        magnitude.from_polar(radians).into()
    }

    /// Retrieve x component
    pub fn x(&self) -> T {
        self.x
    }

    /// Retrieve y component
    pub fn y(&self) -> T {
        self.y
    }

    /// Calculate the squared length/magnitude of the Vec2D
    pub fn squared_length(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    /// Calculate the lenght/magnitude of the Vec2D
    pub fn length(&self) -> T {
        self.squared_length().sqrt()
    }

    /// Turn the Vec2D into a unit Vec2D/
    /// This modifies Vec2D in place
    /// To generate new Vec2D see:backtrace unit()
    pub fn make_unit(&mut self) {
        let den = self.length();
        self.x /= den;
        self.y /= den;
    }

    /// Determine unit Vec2D (return new Vec2D)
    /// To turn this Vec2D into a unit Vec2D see: make_unit()
    pub fn unit_vector(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        } / self.length()
    }

    /// Calculate dot product
    pub fn dot(left: Self, right: Self) -> T {
        left.x * right.x + left.y * right.y
    }
    /// Calculate doct product
    /// ai + bj
    /// wi + yj
    ///
    pub fn cross(left: Self, right: Self) -> T {
        left.x * right.y - right.x * left.y
    }

    pub fn perpendicular(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn to_f32(self) -> Vector2D<f32>
    where
        T: LossyCast<f32>,
    {
        let x: f32 = self.x.cast();
        let y: f32 = self.y.cast();
        Vector2D::<f32>::new(x, y)
    }
    pub fn to_i32(self) -> Vector2D<i32>
    where
        T: LossyCast<i32>,
    {
        let x: i32 = self.x.cast();
        let y: i32 = self.y.cast();
        Vector2D::<i32>::new(x, y)
    }
}

// Operator overloading so that +, -, *, /, -=, +=, *=, /= can be used
impl<T> Add for Vector2D<T>
where
    T: VectorTrait + Add<Output = T>,
{
    type Output = Vector2D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: rhs.y + self.y,
        }
    }
}
impl<T> Sub for Vector2D<T>
where
    T: VectorTrait + Sub<Output = T>,
{
    type Output = Vector2D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T> Mul<T> for Vector2D<T>
where
    T: VectorTrait + Mul<Output = T>,
{
    type Output = Vector2D<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for Vector2D<T>
where
    T: VectorTrait + Div<Output = T>,
{
    type Output = Vector2D<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector2D<T>
where
    T: VectorTrait + Mul<Output = T> + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> DivAssign<T> for Vector2D<T>
where
    T: VectorTrait + Div<Output = T> + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}
impl<T> AddAssign<Vector2D<T>> for Vector2D<T>
where
    T: VectorTrait + Add<Output = T>,
{
    fn add_assign(&mut self, rhs: Vector2D<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}
impl<T> SubAssign<Vector2D<T>> for Vector2D<T>
where
    T: VectorTrait + Sub<Output = T>,
{
    fn sub_assign(&mut self, rhs: Vector2D<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T> Neg for Vector2D<T>
where
    T: VectorTrait + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> From<(T, T)> for Vector2D<T>
where
    T: VectorTrait,
{
    fn from(tupple: (T, T)) -> Self {
        Self {
            x: tupple.0,
            y: tupple.1,
        }
    }
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

impl AngleTrait for Vector2D<i32> {
    fn angle(&self) -> f32 {
        let x = self.x as f32;
        let y = self.y as f32;
        return (x / y).atan();
    }
}
impl AngleTrait for Vector2D<f32> {
    fn angle(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        return (x / y).atan();
    }
}
impl AngleTrait for Vector2D<i8> {
    fn angle(&self) -> f32 {
        let x = self.x as f32;
        let y = self.y as f32;
        return (x / y).atan();
    }
}
impl AngleTrait for Vector2D<i16> {
    fn angle(&self) -> f32 {
        let x = self.x as f32;
        let y = self.y as f32;
        return (x / y).atan();
    }
}
impl AngleTrait for Vector2D<f64> {
    fn angle(&self) -> f32 {
        let x = self.x as f32;
        let y = self.y as f32;
        return (x / y).atan();
    }
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

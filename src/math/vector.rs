use crate::math::*;

// types that are allowed to implement vector
impl SimpleMathTrait for i8 {}
impl SimpleMathTrait for i16 {}
impl SimpleMathTrait for i32 {}
impl SimpleMathTrait for f64 {}
impl SimpleMathTrait for f32 {}

/// A generic vector type that offers vector operations such as
///     Dot product
///     Cross produt
///     Addition
///     Subtraction
/// Vector of unsigned types are not allowed as we implement Neg Trait
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector2D<T>
where
    T: SimpleMathTrait,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T>
where
    T: SimpleMathTrait
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

    /// Calculate the squared length/magnitude of the Vector2D
    pub fn squared_length(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    /// Calculate the lenght/magnitude of the Vector2D
    pub fn length(&self) -> T {
        self.squared_length().sqrt()
    }

    /// Turn the Vector2D into a unit Vector2D/
    /// This modifies Vector2D in place
    /// To generate new Vector2D see:backtrace unit()
    pub fn make_unit(&mut self) {
        let den = self.length();
        self.x /= den;
        self.y /= den;
    }

    /// Determine unit Vector2D (return new Vector2D)
    /// To turn this Vector2D into a unit Vector2D see: make_unit()
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
    pub fn clamp(&mut self, x: T, y: T)
    where
        T: PartialOrd,
    {
        self.y = super::min(self.y, y);
        self.x = super::min(self.x, x);
    }
    pub fn clamp_between(&mut self, x_min: T, y_min: T, x_max: T, y_max: T)
    where
        T: PartialOrd + Zero<Type = T>,
    {
        self.y = super::min(self.y, y_max);
        self.x = super::min(self.x, x_max);

        self.y = super::max(self.y, y_min);
        self.x = super::max(self.x, x_min);
    }
}

// Operator overloading so that +, -, *, /, -=, +=, *=, /= can be used
impl<T> Add for Vector2D<T>
where
    T: SimpleMathTrait + Add<Output = T>,
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
    T: SimpleMathTrait + Sub<Output = T>,
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
    T: SimpleMathTrait + Mul<Output = T>,
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
    T: SimpleMathTrait + Div<Output = T>,
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
    T: SimpleMathTrait + Mul<Output = T> + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> DivAssign<T> for Vector2D<T>
where
    T: SimpleMathTrait + Div<Output = T> + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}
impl<T> AddAssign<Vector2D<T>> for Vector2D<T>
where
    T: SimpleMathTrait + Add<Output = T>,
{
    fn add_assign(&mut self, rhs: Vector2D<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}
impl<T> SubAssign<Vector2D<T>> for Vector2D<T>
where
    T: SimpleMathTrait + Sub<Output = T>,
{
    fn sub_assign(&mut self, rhs: Vector2D<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T> Neg for Vector2D<T>
where
    T: SimpleMathTrait + Neg<Output = T>,
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
    T: SimpleMathTrait,
{
    fn from(tupple: (T, T)) -> Self {
        Self {
            x: tupple.0,
            y: tupple.1,
        }
    }
}
impl<T> From<Vector3D<T>> for Vector2D<T>
where
    T: SimpleMathTrait,
{
    fn from(vec: Vector3D<T>) -> Self {
        Self { x: vec.x, y: vec.y }
    }
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

/// A generic vector type that offers vector operations such as
///     Dot product
///     Cross produt
///     Addition
///     Subtraction
/// Vector of unsigned types are not allowed as we implement Neg Trait
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector3D<T>
where
    T: SimpleMathTrait,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T>
where
    T: SimpleMathTrait
        + Div<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>,
    <T as Mul>::Output: Add<Output = T>,
    <T as Mul>::Output: Sub<Output = T>,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Retrieve x component
    pub fn x(&self) -> T {
        self.x
    }

    /// Retrieve y component
    pub fn y(&self) -> T {
        self.y
    }
    /// Retrieve z component
    pub fn z(&self) -> T {
        self.z
    }

    /// Calculate the squared length/magnitude of the Vector3D
    pub fn squared_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Calculate the lenght/magnitude of the Vector3D
    pub fn length(&self) -> T {
        self.squared_length().sqrt()
    }

    /// Turn the Vector3D into a unit Vector3D/
    /// This modifies Vector3D in place
    /// To generate new Vector3D see:backtrace unit()
    pub fn make_unit(&mut self) {
        let den = self.length();
        self.x /= den;
        self.y /= den;
        self.z /= den;
    }

    /// Determine unit Vector3D (return new Vector3D)
    /// To turn this Vector3D into a unit Vector3D see: make_unit()
    pub fn unit_vector(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        } / self.length()
    }

    /// Calculate dot product
    pub fn dot(left: Self, right: Self) -> T {
        left.x * right.x + left.y * right.y + left.z * right.z
    }
    /// Calculate cross product
    pub fn cross(left: Self, right: Self) -> Self {
        Self {
            x: left.y * right.z - left.z * right.y,
            y: -(left.x * right.z - left.z * right.x),
            z: left.x * right.y - left.y * right.x,
        }
    }

    pub fn angle(a: Self, b: Self) -> f32
    where
        T: LossyCast<f32>,
    {
        let a_m: f32 = a.length().cast();
        let b_m: f32 = b.length().cast();
        let a_b_dot: f32 = Vector3D::dot(a, b).cast();
        return (a_b_dot / (a_m * b_m)).acos();
    }

    pub fn to_f32(self) -> Vector3D<f32>
    where
        T: LossyCast<f32>,
    {
        let x: f32 = self.x.cast();
        let y: f32 = self.y.cast();
        let z: f32 = self.z.cast();
        Vector3D::<f32>::new(x, y, z)
    }
    pub fn to_i32(self) -> Vector3D<i32>
    where
        T: LossyCast<i32>,
    {
        let x: i32 = self.x.cast();
        let y: i32 = self.y.cast();
        let z: i32 = self.z.cast();
        Vector3D::<i32>::new(x, y, z)
    }
}

// Operator overloading so that +, -, *, /, -=, +=, *=, /= can be used
impl<T> Add for Vector3D<T>
where
    T: SimpleMathTrait + Add<Output = T>,
{
    type Output = Vector3D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: rhs.y + self.y,
            z: rhs.z + self.z,
        }
    }
}
impl<T> Sub for Vector3D<T>
where
    T: SimpleMathTrait + Sub<Output = T>,
{
    type Output = Vector3D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T> Mul<T> for Vector3D<T>
where
    T: SimpleMathTrait + Mul<Output = T>,
{
    type Output = Vector3D<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div<T> for Vector3D<T>
where
    T: SimpleMathTrait + Div<Output = T>,
{
    type Output = Vector3D<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector3D<T>
where
    T: SimpleMathTrait + Mul<Output = T> + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T> DivAssign<T> for Vector3D<T>
where
    T: SimpleMathTrait + Div<Output = T> + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}
impl<T> AddAssign<Vector3D<T>> for Vector3D<T>
where
    T: SimpleMathTrait + Add<Output = T>,
{
    fn add_assign(&mut self, rhs: Vector3D<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}
impl<T> SubAssign<Vector3D<T>> for Vector3D<T>
where
    T: SimpleMathTrait + Sub<Output = T>,
{
    fn sub_assign(&mut self, rhs: Vector3D<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl<T> Neg for Vector3D<T>
where
    T: SimpleMathTrait + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> From<(T, T, T)> for Vector3D<T>
where
    T: SimpleMathTrait,
{
    fn from(tupple: (T, T, T)) -> Self {
        Self {
            x: tupple.0,
            y: tupple.1,
            z: tupple.2,
        }
    }
}

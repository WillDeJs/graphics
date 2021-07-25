use crate::math::vector::Vector3D;
use crate::math::*;

/// A simple Matrix 3 by 3 struct
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(unused_variables, dead_code)]
pub struct Mat3x3<T>
where
    T: SimpleMathTrait,
{
    inner: [[T; 3]; 3],
}

impl<T> Mat3x3<T>
where
    T: SimpleMathTrait,
{
    pub fn default() -> Self
    where
        T: Zero<Type = T>,
    {
        Self {
            inner: [
                [T::zero(), T::zero(), T::zero()],
                [T::zero(), T::zero(), T::zero()],
                [T::zero(), T::zero(), T::zero()],
            ],
        }
    }

    /// Create an itentiy matrix 3x3
    pub fn identity() -> Self
    where
        T: Zero<Type = T> + Unit<Type = T>,
    {
        Self {
            inner: [
                [T::one(), T::zero(), T::zero()],
                [T::zero(), T::one(), T::zero()],
                [T::zero(), T::zero(), T::one()],
            ],
        }
    }

    /// Scale transform a scale matrix with the given x and y transform values
    pub fn translate(vx: T, vy: T) -> Self
    where
        T: Zero<Type = T> + Unit<Type = T>,
    {
        let mut result = Self::identity();
        result.inner[0][2] = vx;
        result.inner[1][2] = vy;
        return result;
    }

    /// create a scale matrix with the given x and y transform values
    pub fn scale(cx: T, cy: T) -> Self
    where
        T: Zero<Type = T> + Unit<Type = T>,
    {
        let mut result = Self::identity();
        result.inner[0][0] = cx;
        result.inner[1][1] = cy;
        return result;
    }
    /// create a rotate matrix with the given x and y transform values
    pub fn rotate(alpha: f32) -> Self
    where
        T: Zero<Type = T> + Unit<Type = T> + LossyCast<f32> + Neg<Output = T>,
        f32: LossyCast<T>,
    {
        let mut result = Self::identity();
        result.inner[0][0] = alpha.cos().cast();
        result.inner[0][1] = -alpha.sin().cast();

        result.inner[1][0] = alpha.sin().cast();
        result.inner[1][1] = alpha.cos().cast();
        return result;
    }

    /// Calculate the determinant of this matrix
    pub fn det(&self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T>,
    {
        self.inner[0][0]
            * (self.inner[1][1] * self.inner[2][2] - self.inner[1][2] * self.inner[2][1])
            - self.inner[0][1]
                * (self.inner[1][0] * self.inner[2][2] - self.inner[1][2] * self.inner[2][0])
            + self.inner[0][2]
                * (self.inner[1][0] * self.inner[2][1] - self.inner[1][1] * self.inner[2][0])
    }

    /// Calculate the inverse of this matrix
    /// https://mathworld.wolfram.com/MatrixInverse.html
    pub fn inverse(&self) -> Self
    where
        T: Add<Output = T>
            + Mul<Output = T>
            + Sub<Output = T>
            + Zero<Type = T>
            + Unit<Type = T>
            + Div<Output = T>,
    {
        let mut result = Self::default();

        // first row
        result.inner[0][0] =
            self.inner[1][1] * self.inner[2][2] - self.inner[1][2] * self.inner[2][1];
        result.inner[0][1] =
            self.inner[0][2] * self.inner[2][1] - self.inner[2][2] * self.inner[0][1];
        result.inner[0][2] =
            self.inner[0][1] * self.inner[1][2] - self.inner[1][1] * self.inner[0][2];

        // middle row
        result.inner[1][0] =
            self.inner[1][2] * self.inner[2][0] - self.inner[1][0] * self.inner[2][2];
        result.inner[1][1] =
            self.inner[0][0] * self.inner[2][2] - self.inner[0][2] * self.inner[2][0];
        result.inner[1][2] =
            self.inner[0][2] * self.inner[1][0] - self.inner[1][2] * self.inner[0][0];

        // last row
        result.inner[2][0] =
            self.inner[1][0] * self.inner[2][1] - self.inner[2][0] * self.inner[1][1];
        result.inner[2][1] =
            self.inner[0][1] * self.inner[2][0] - self.inner[2][1] * self.inner[0][0];
        result.inner[2][2] =
            self.inner[0][0] * self.inner[1][1] - self.inner[1][0] * self.inner[0][1];

        return result / self.det();
    }

    /// Transform a single point given this matrix (useful on affine transforms)
    pub fn transform_point(&self, point: Vector3D<T>) -> Vector3D<T>
    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Neg<Output = T>,
    {
        let x = self.inner[0][0] * point.x()
            + self.inner[0][1] * point.y()
            + self.inner[0][2] * point.z();
        let y = self.inner[1][0] * point.x()
            + self.inner[1][1] * point.y()
            + self.inner[1][2] * point.z();
        let z = self.inner[2][0] * point.x()
            + self.inner[2][1] * point.y()
            + self.inner[2][2] * point.z();
        Vector3D::<T>::new(x, y, z)
    }
    /// Useful method to convert matrix to i32 matrix
    pub fn to_i32(&self) -> Mat3x3<i32>
    where
        T: LossyCast<i32> + Zero<Type = T>,
    {
        let mut result = Mat3x3::<i32>::default();
        for r in 0..self.inner.len() {
            for c in 0..self.inner[r].len() {
                result.inner[r][c] = self.inner[r][c].cast();
            }
        }
        return result;
    }

    /// Useful method to convert matrix to f32 matrix
    pub fn to_f32(&self) -> Mat3x3<f32>
    where
        T: LossyCast<f32> + Zero<Type = T>,
    {
        let mut result = Mat3x3::<f32>::default();
        for r in 0..self.inner.len() {
            for c in 0..self.inner[r].len() {
                result.inner[r][c] = self.inner[r][c].cast();
            }
        }
        return result;
    }
}

/// Make it so that we can use into() and from() to convert from 2D array
impl<T> From<[[T; 3]; 3]> for Mat3x3<T>
where
    T: SimpleMathTrait,
{
    fn from(inner: [[T; 3]; 3]) -> Self {
        Self { inner }
    }
}

impl<T> Add for Mat3x3<T>
where
    T: Add<Output = T> + SimpleMathTrait,
{
    type Output = Mat3x3<T>;
    fn add(self, other: Mat3x3<T>) -> Self::Output {
        Mat3x3 {
            inner: [
                [
                    self.inner[0][0] + other.inner[0][0],
                    self.inner[0][1] + other.inner[0][1],
                    self.inner[0][2] + other.inner[0][2],
                ],
                [
                    self.inner[1][0] + other.inner[1][0],
                    self.inner[1][1] + other.inner[1][1],
                    self.inner[1][2] + other.inner[1][2],
                ],
                [
                    self.inner[2][0] + other.inner[2][0],
                    self.inner[2][1] + other.inner[2][1],
                    self.inner[2][2] + other.inner[2][2],
                ],
            ],
        }
    }
}
impl<T> Sub for Mat3x3<T>
where
    T: Sub<Output = T> + SimpleMathTrait,
{
    type Output = Mat3x3<T>;
    fn sub(self, other: Mat3x3<T>) -> Self::Output {
        Mat3x3 {
            inner: [
                [
                    self.inner[0][0] - other.inner[0][0],
                    self.inner[0][1] - other.inner[0][1],
                    self.inner[0][2] - other.inner[0][2],
                ],
                [
                    self.inner[1][0] - other.inner[1][0],
                    self.inner[1][1] - other.inner[1][1],
                    self.inner[1][2] - other.inner[1][2],
                ],
                [
                    self.inner[2][0] - other.inner[2][0],
                    self.inner[2][1] - other.inner[2][1],
                    self.inner[2][2] - other.inner[2][2],
                ],
            ],
        }
    }
}

impl<T> AddAssign<Mat3x3<T>> for Mat3x3<T>
where
    T: Add<Output = T> + SimpleMathTrait,
{
    fn add_assign(&mut self, other: Mat3x3<T>) {
        self.inner[0][0] = self.inner[0][0] + other.inner[0][0];
        self.inner[0][1] = self.inner[0][1] + other.inner[0][1];
        self.inner[0][2] = self.inner[0][2] + other.inner[0][2];

        self.inner[1][0] = self.inner[1][0] + other.inner[1][0];
        self.inner[1][1] = self.inner[1][1] + other.inner[1][1];
        self.inner[1][2] = self.inner[1][2] + other.inner[1][2];

        self.inner[2][0] = self.inner[2][0] + other.inner[2][0];
        self.inner[2][1] = self.inner[2][1] + other.inner[2][1];
        self.inner[2][2] = self.inner[2][2] + other.inner[2][2];
    }
}
impl<T> SubAssign<Mat3x3<T>> for Mat3x3<T>
where
    T: Sub<Output = T> + SimpleMathTrait,
{
    fn sub_assign(&mut self, other: Mat3x3<T>) {
        self.inner[0][0] = self.inner[0][0] - other.inner[0][0];
        self.inner[0][1] = self.inner[0][1] - other.inner[0][1];
        self.inner[0][2] = self.inner[0][2] - other.inner[0][2];

        self.inner[1][0] = self.inner[1][0] - other.inner[1][0];
        self.inner[1][1] = self.inner[1][1] - other.inner[1][1];
        self.inner[1][2] = self.inner[1][2] - other.inner[1][2];

        self.inner[2][0] = self.inner[2][0] - other.inner[2][0];
        self.inner[2][1] = self.inner[2][1] - other.inner[2][1];
        self.inner[2][2] = self.inner[2][2] - other.inner[2][2];
    }
}
impl<T> Mul<T> for Mat3x3<T>
where
    T: Mul<Output = T> + SimpleMathTrait,
{
    type Output = Mat3x3<T>;
    fn mul(self, other: T) -> Self::Output {
        Mat3x3 {
            inner: [
                [
                    self.inner[0][0] * other,
                    self.inner[0][1] * other,
                    self.inner[0][2] * other,
                ],
                [
                    self.inner[1][0] * other,
                    self.inner[1][1] * other,
                    self.inner[1][2] * other,
                ],
                [
                    self.inner[2][0] * other,
                    self.inner[2][1] * other,
                    self.inner[2][2] * other,
                ],
            ],
        }
    }
}

impl<T> Div<T> for Mat3x3<T>
where
    T: Div<Output = T> + SimpleMathTrait,
{
    type Output = Mat3x3<T>;
    fn div(self, other: T) -> Self::Output {
        Mat3x3 {
            inner: [
                [
                    self.inner[0][0] / other,
                    self.inner[0][1] / other,
                    self.inner[0][2] / other,
                ],
                [
                    self.inner[1][0] / other,
                    self.inner[1][1] / other,
                    self.inner[1][2] / other,
                ],
                [
                    self.inner[2][0] / other,
                    self.inner[2][1] / other,
                    self.inner[2][2] / other,
                ],
            ],
        }
    }
}

impl<T> Mul for Mat3x3<T>
where
    T: Add<Output = T> + SimpleMathTrait + Mul<Output = T>,
{
    type Output = Mat3x3<T>;
    fn mul(self, other: Mat3x3<T>) -> Self::Output {
        Mat3x3 {
            inner: [
                [
                    self.inner[0][0] * other.inner[0][0]
                        + self.inner[0][1] * other.inner[1][0]
                        + self.inner[0][2] * other.inner[2][0],
                    self.inner[0][0] * other.inner[0][1]
                        + self.inner[0][1] * other.inner[1][1]
                        + self.inner[0][2] * other.inner[2][1],
                    self.inner[0][0] * other.inner[0][2]
                        + self.inner[0][1] * other.inner[1][2]
                        + self.inner[0][2] * other.inner[2][2],
                ],
                [
                    self.inner[1][0] * other.inner[0][0]
                        + self.inner[1][1] * other.inner[1][0]
                        + self.inner[1][2] * other.inner[2][0],
                    self.inner[1][0] * other.inner[0][1]
                        + self.inner[1][1] * other.inner[1][1]
                        + self.inner[1][2] * other.inner[2][1],
                    self.inner[1][0] * other.inner[0][2]
                        + self.inner[1][1] * other.inner[1][2]
                        + self.inner[1][2] * other.inner[2][2],
                ],
                [
                    self.inner[2][0] * other.inner[0][0]
                        + self.inner[2][1] * other.inner[1][0]
                        + self.inner[2][2] * other.inner[2][0],
                    self.inner[2][0] * other.inner[0][1]
                        + self.inner[2][1] * other.inner[1][1]
                        + self.inner[2][2] * other.inner[2][1],
                    self.inner[2][0] * other.inner[0][2]
                        + self.inner[2][1] * other.inner[1][2]
                        + self.inner[2][2] * other.inner[2][2],
                ],
            ],
        }
    }
}

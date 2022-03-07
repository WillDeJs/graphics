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
        result
    }

    /// create a scale matrix with the given x and y transform values
    pub fn scale(cx: T, cy: T) -> Self
    where
        T: Zero<Type = T> + Unit<Type = T>,
    {
        let mut result = Self::identity();
        result.inner[0][0] = cx;
        result.inner[1][1] = cy;
        result
    }
    /// create a rotate matrix with the given x and y transform values
    pub fn rotate(alpha: f32) -> Mat3x3<f32>
    where
        T: Zero<Type = T> + Unit<Type = T> + LossyCast<f32> + Neg<Output = T>,
        f32: LossyCast<T>,
    {
        let mut result = Mat3x3::<f32>::identity();
        result.inner[0][0] = alpha.cos();
        result.inner[0][1] = -alpha.sin();

        result.inner[1][0] = alpha.sin();
        result.inner[1][1] = alpha.cos();
        result
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
    /// <https://mathworld.wolfram.com/MatrixInverse.html>
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

        result / self.det()
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
        result
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
        result
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

/// A simple Matrix 4 by 4 struct
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(unused_variables, dead_code)]
pub struct Mat4x4<T>
where
    T: SimpleMathTrait,
{
    inner: [[T; 4]; 4],
}

impl<T> Mat4x4<T>
where
    T: SimpleMathTrait,
{
    pub fn default() -> Self
    where
        T: Zero<Type = T>,
    {
        Self {
            inner: [
                [T::zero(), T::zero(), T::zero(), T::zero()],
                [T::zero(), T::zero(), T::zero(), T::zero()],
                [T::zero(), T::zero(), T::zero(), T::zero()],
                [T::zero(), T::zero(), T::zero(), T::zero()],
            ],
        }
    }

    /// Create an itentiy matrix 4x4
    pub fn identity() -> Self
    where
        T: Zero<Type = T> + Unit<Type = T>,
    {
        Self {
            inner: [
                [T::one(), T::zero(), T::zero(), T::zero()],
                [T::zero(), T::one(), T::zero(), T::zero()],
                [T::zero(), T::zero(), T::one(), T::zero()],
                [T::zero(), T::zero(), T::zero(), T::one()],
            ],
        }
    }

    /// Calculate the determinant of this matrix
    pub fn det(&self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T>,
    {
        let a = self.inner[0][0];
        let b = self.inner[0][1];
        let c = self.inner[0][2];
        let d = self.inner[0][3];

        // get sub matrixces using laplace method
        let a_mat: Mat3x3<T> = [
            [self.inner[1][1], self.inner[1][2], self.inner[1][3]],
            [self.inner[2][1], self.inner[2][2], self.inner[2][3]],
            [self.inner[3][1], self.inner[3][2], self.inner[3][3]],
        ]
        .into();
        let b_mat: Mat3x3<T> = [
            [self.inner[1][0], self.inner[1][2], self.inner[1][3]],
            [self.inner[2][0], self.inner[2][2], self.inner[2][3]],
            [self.inner[3][0], self.inner[3][2], self.inner[3][3]],
        ]
        .into();
        let c_mat: Mat3x3<T> = [
            [self.inner[1][0], self.inner[1][1], self.inner[1][3]],
            [self.inner[2][0], self.inner[2][1], self.inner[2][3]],
            [self.inner[3][0], self.inner[3][1], self.inner[3][3]],
        ]
        .into();
        let d_mat: Mat3x3<T> = [
            [self.inner[1][0], self.inner[1][1], self.inner[1][2]],
            [self.inner[2][0], self.inner[2][1], self.inner[2][2]],
            [self.inner[3][0], self.inner[3][1], self.inner[3][2]],
        ]
        .into();

        a * a_mat.det() - b * b_mat.det() + c * c_mat.det() - d * d_mat.det()
    }
    /// Multiply by a vector  3D (useful for projection)
    pub fn vector_multiply(&self, point: Vector3D<T>) -> Vector3D<T>
    where
        T: Add<Output = T>
            + Mul<Output = T>
            + Sub<Output = T>
            + Div<Output = T>
            + Neg<Output = T>
            + Zero<Type = T>
            + std::cmp::PartialOrd,
    {
        let x = self.inner[0][0] * point.x()
            + self.inner[1][0] * point.y()
            + self.inner[2][0] * point.z()
            + self.inner[3][0];
        let y = self.inner[0][1] * point.x()
            + self.inner[1][1] * point.y()
            + self.inner[2][1] * point.z()
            + self.inner[3][1];
        let z = self.inner[0][2] * point.x()
            + self.inner[1][2] * point.y()
            + self.inner[2][2] * point.z()
            + self.inner[3][2];
        let w = self.inner[0][3] * point.x()
            + self.inner[1][3] * point.y()
            + self.inner[2][3] * point.z()
            + self.inner[3][3];

        if w != T::zero() {
            Vector3D::<T>::new(x, y, z) / w
        } else {
            Vector3D::<T>::new(x, y, z)
        }
    }

    pub fn projected(aspect: T, fov: T, far: T, near: T) -> Self
    where
        T: Add<Output = T>
            + Mul<Output = T>
            + Sub<Output = T>
            + Div<Output = T>
            + Neg<Output = T>
            + Zero<Type = T>
            + Unit<Type = T>,
    {
        let mut mat = Mat4x4::identity();
        mat.inner[0][0] = aspect * fov;
        mat.inner[1][1] = fov;
        mat.inner[2][2] = far / (far - near);
        mat.inner[3][2] = (-far * near) / (far - near);
        mat.inner[2][3] = T::one();
        mat.inner[3][3] = T::zero();
        mat
    }

    /// Scale transform a scale matrix with the given x and y transform values
    pub fn translate(vx: T, vy: T, vz: T) -> Self
    where
        T: Zero<Type = T> + Unit<Type = T>,
    {
        let mut result = Self::identity();
        result.inner[3][0] = vx;
        result.inner[3][1] = vy;
        result.inner[3][2] = vz;
        result
    }

    /// create a scale matrix with the given x and y transform values
    pub fn scale(cx: T, cy: T, cz: T) -> Self
    where
        T: Zero<Type = T> + Unit<Type = T>,
    {
        let mut result = Self::identity();
        result.inner[0][0] = cx;
        result.inner[1][1] = cy;
        result.inner[2][2] = cz;
        result
    }
    /// create a rotate matrix with the given x and y transform values
    pub fn rotate_x(alpha: f32) -> Mat4x4<f32>
    where
        T: Zero<Type = T> + Unit<Type = T> + LossyCast<f32> + Neg<Output = T>,
        f32: LossyCast<T>,
    {
        let mut result = Mat4x4::<f32>::identity();

        result.inner[1][1] = alpha.cos();

        result.inner[1][2] = alpha.sin();
        result.inner[2][1] = -alpha.sin();
        result.inner[2][2] = alpha.cos();
        result
    }
    /// create a rotate matrix with the given x and y transform values
    pub fn rotate_y(alpha: f32) -> Mat4x4<f32>
    where
        T: Zero<Type = T> + Unit<Type = T> + LossyCast<f32> + Neg<Output = T>,
        f32: LossyCast<T>,
    {
        let mut result = Mat4x4::<f32>::identity();
        result.inner[0][0] = alpha.cos();
        result.inner[0][2] = alpha.sin();

        result.inner[2][0] = -alpha.sin();
        result.inner[2][2] = alpha.cos();
        result
    }
    /// create a rotate matrix with the given x and y transform values
    pub fn rotate_z(alpha: f32) -> Mat4x4<f32>
    where
        T: Zero<Type = T> + Unit<Type = T> + LossyCast<f32> + Neg<Output = T>,
        f32: LossyCast<T>,
    {
        let mut result = Mat4x4::<f32>::identity();
        result.inner[0][0] = alpha.cos();
        result.inner[0][1] = alpha.sin();

        result.inner[1][0] = -alpha.sin();
        result.inner[1][1] = alpha.cos();
        result
    }

    // From OLC Javidx
    // https://github.com/OneLoneCoder/videos/blob/master/OneLoneCoder_olcEngine3D_Part3.cpp
    pub fn point_at(pos: Vector3D<T>, target: Vector3D<T>, up: Vector3D<T>) -> Self
    where
        T: Add<Output = T>
            + Mul<Output = T>
            + Sub<Output = T>
            + Div<Output = T>
            + Neg<Output = T>
            + Zero<Type = T>
            + Unit<Type = T>
            + std::cmp::PartialOrd,
    {
        let forward = (target - pos).unit_vector();
        let a = forward * Vector3D::dot(up, forward);
        let new_up = (up - a).unit_vector();

        let right_dir = Vector3D::cross(new_up, forward);

        Self {
            inner: [
                [right_dir.x, right_dir.y, right_dir.z, T::zero()],
                [new_up.x, new_up.y, new_up.z, T::zero()],
                [forward.x, forward.y, forward.z, T::zero()],
                [pos.x, pos.y, pos.z, T::one()],
            ],
        }
    }

    // Determine the inverse of this matrix
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
        let det = self.det();
        let m = self.inner;
        result.inner[0][0] = m[1][2] * m[2][3] * m[3][1] - m[1][3] * m[2][2] * m[3][1]
            + m[1][3] * m[2][1] * m[3][2]
            - m[1][1] * m[2][3] * m[3][2]
            - m[1][2] * m[2][1] * m[3][3]
            + m[1][1] * m[2][2] * m[3][3];
        result.inner[0][1] =
            m[0][3] * m[2][2] * m[3][1] - m[0][2] * m[2][3] * m[3][1] - m[0][3] * m[2][1] * m[3][2]
                + m[0][1] * m[2][3] * m[3][2]
                + m[0][2] * m[2][1] * m[3][3]
                - m[0][1] * m[2][2] * m[3][3];
        result.inner[0][2] = m[0][2] * m[1][3] * m[3][1] - m[0][3] * m[1][2] * m[3][1]
            + m[0][3] * m[1][1] * m[3][2]
            - m[0][1] * m[1][3] * m[3][2]
            - m[0][2] * m[1][1] * m[3][3]
            + m[0][1] * m[1][2] * m[3][3];
        result.inner[0][3] =
            m[0][3] * m[1][2] * m[2][1] - m[0][2] * m[1][3] * m[2][1] - m[0][3] * m[1][1] * m[2][2]
                + m[0][1] * m[1][3] * m[2][2]
                + m[0][2] * m[1][1] * m[2][3]
                - m[0][1] * m[1][2] * m[2][3];
        result.inner[1][0] =
            m[1][3] * m[2][2] * m[3][0] - m[1][2] * m[2][3] * m[3][0] - m[1][3] * m[2][0] * m[3][2]
                + m[1][0] * m[2][3] * m[3][2]
                + m[1][2] * m[2][0] * m[3][3]
                - m[1][0] * m[2][2] * m[3][3];
        result.inner[1][1] = m[0][2] * m[2][3] * m[3][0] - m[0][3] * m[2][2] * m[3][0]
            + m[0][3] * m[2][0] * m[3][2]
            - m[0][0] * m[2][3] * m[3][2]
            - m[0][2] * m[2][0] * m[3][3]
            + m[0][0] * m[2][2] * m[3][3];
        result.inner[1][2] =
            m[0][3] * m[1][2] * m[3][0] - m[0][2] * m[1][3] * m[3][0] - m[0][3] * m[1][0] * m[3][2]
                + m[0][0] * m[1][3] * m[3][2]
                + m[0][2] * m[1][0] * m[3][3]
                - m[0][0] * m[1][2] * m[3][3];
        result.inner[1][3] = m[0][2] * m[1][3] * m[2][0] - m[0][3] * m[1][2] * m[2][0]
            + m[0][3] * m[1][0] * m[2][2]
            - m[0][0] * m[1][3] * m[2][2]
            - m[0][2] * m[1][0] * m[2][3]
            + m[0][0] * m[1][2] * m[2][3];
        result.inner[2][0] = m[1][1] * m[2][3] * m[3][0] - m[1][3] * m[2][1] * m[3][0]
            + m[1][3] * m[2][0] * m[3][1]
            - m[1][0] * m[2][3] * m[3][1]
            - m[1][1] * m[2][0] * m[3][3]
            + m[1][0] * m[2][1] * m[3][3];
        result.inner[2][1] =
            m[0][3] * m[2][1] * m[3][0] - m[0][1] * m[2][3] * m[3][0] - m[0][3] * m[2][0] * m[3][1]
                + m[0][0] * m[2][3] * m[3][1]
                + m[0][1] * m[2][0] * m[3][3]
                - m[0][0] * m[2][1] * m[3][3];
        result.inner[2][2] = m[0][1] * m[1][3] * m[3][0] - m[0][3] * m[1][1] * m[3][0]
            + m[0][3] * m[1][0] * m[3][1]
            - m[0][0] * m[1][3] * m[3][1]
            - m[0][1] * m[1][0] * m[3][3]
            + m[0][0] * m[1][1] * m[3][3];
        result.inner[2][3] =
            m[0][3] * m[1][1] * m[2][0] - m[0][1] * m[1][3] * m[2][0] - m[0][3] * m[1][0] * m[2][1]
                + m[0][0] * m[1][3] * m[2][1]
                + m[0][1] * m[1][0] * m[2][3]
                - m[0][0] * m[1][1] * m[2][3];
        result.inner[3][0] =
            m[1][2] * m[2][1] * m[3][0] - m[1][1] * m[2][2] * m[3][0] - m[1][2] * m[2][0] * m[3][1]
                + m[1][0] * m[2][2] * m[3][1]
                + m[1][1] * m[2][0] * m[3][2]
                - m[1][0] * m[2][1] * m[3][2];
        result.inner[3][1] = m[0][1] * m[2][2] * m[3][0] - m[0][2] * m[2][1] * m[3][0]
            + m[0][2] * m[2][0] * m[3][1]
            - m[0][0] * m[2][2] * m[3][1]
            - m[0][1] * m[2][0] * m[3][2]
            + m[0][0] * m[2][1] * m[3][2];
        result.inner[3][2] =
            m[0][2] * m[1][1] * m[3][0] - m[0][1] * m[1][2] * m[3][0] - m[0][2] * m[1][0] * m[3][1]
                + m[0][0] * m[1][2] * m[3][1]
                + m[0][1] * m[1][0] * m[3][2]
                - m[0][0] * m[1][1] * m[3][2];
        result.inner[3][3] = m[0][1] * m[1][2] * m[2][0] - m[0][2] * m[1][1] * m[2][0]
            + m[0][2] * m[1][0] * m[2][1]
            - m[0][0] * m[1][2] * m[2][1]
            - m[0][1] * m[1][0] * m[2][2]
            + m[0][0] * m[1][1] * m[2][2];

        // return after deviding by determinant
        result / det
    }

    /// Useful method to convert matrix to i32 matrix
    pub fn to_i32(&self) -> Mat4x4<i32>
    where
        T: LossyCast<i32> + Zero<Type = T>,
    {
        let mut result = Mat4x4::<i32>::default();
        for r in 0..self.inner.len() {
            for c in 0..self.inner[r].len() {
                result.inner[r][c] = self.inner[r][c].cast();
            }
        }
        result
    }

    /// Useful method to convert matrix to f32 matrix
    pub fn to_f32(&self) -> Mat4x4<f32>
    where
        T: LossyCast<f32> + Zero<Type = T>,
    {
        let mut result = Mat4x4::<f32>::default();
        for r in 0..self.inner.len() {
            for c in 0..self.inner[r].len() {
                result.inner[r][c] = self.inner[r][c].cast();
            }
        }
        result
    }
}

/// Make it so that we can use into() and from() to convert from 2D array
impl<T> From<[[T; 4]; 4]> for Mat4x4<T>
where
    T: SimpleMathTrait,
{
    fn from(inner: [[T; 4]; 4]) -> Self {
        Self { inner }
    }
}

impl<T> Add for Mat4x4<T>
where
    T: Add<Output = T> + SimpleMathTrait,
{
    type Output = Mat4x4<T>;
    fn add(self, other: Mat4x4<T>) -> Self::Output {
        Mat4x4 {
            inner: [
                [
                    self.inner[0][0] + other.inner[0][0],
                    self.inner[0][1] + other.inner[0][1],
                    self.inner[0][2] + other.inner[0][2],
                    self.inner[0][3] + other.inner[0][3],
                ],
                [
                    self.inner[1][0] + other.inner[1][0],
                    self.inner[1][1] + other.inner[1][1],
                    self.inner[1][2] + other.inner[1][2],
                    self.inner[1][3] + other.inner[1][3],
                ],
                [
                    self.inner[2][0] + other.inner[2][0],
                    self.inner[2][1] + other.inner[2][1],
                    self.inner[2][2] + other.inner[2][2],
                    self.inner[2][3] + other.inner[2][3],
                ],
                [
                    self.inner[3][0] + other.inner[3][0],
                    self.inner[3][1] + other.inner[3][1],
                    self.inner[3][2] + other.inner[3][2],
                    self.inner[3][3] + other.inner[3][3],
                ],
            ],
        }
    }
}
impl<T> Sub for Mat4x4<T>
where
    T: Sub<Output = T> + SimpleMathTrait,
{
    type Output = Mat4x4<T>;
    fn sub(self, other: Mat4x4<T>) -> Self::Output {
        Mat4x4 {
            inner: [
                [
                    self.inner[0][0] - other.inner[0][0],
                    self.inner[0][1] - other.inner[0][1],
                    self.inner[0][2] - other.inner[0][2],
                    self.inner[0][3] - other.inner[0][3],
                ],
                [
                    self.inner[1][0] - other.inner[1][0],
                    self.inner[1][1] - other.inner[1][1],
                    self.inner[1][2] - other.inner[1][2],
                    self.inner[1][3] - other.inner[1][3],
                ],
                [
                    self.inner[2][0] - other.inner[2][0],
                    self.inner[2][1] - other.inner[2][1],
                    self.inner[2][2] - other.inner[2][2],
                    self.inner[2][3] - other.inner[2][3],
                ],
                [
                    self.inner[3][0] - other.inner[3][0],
                    self.inner[3][1] - other.inner[3][1],
                    self.inner[3][2] - other.inner[3][2],
                    self.inner[3][3] - other.inner[3][3],
                ],
            ],
        }
    }
}

impl<T> AddAssign<Mat4x4<T>> for Mat4x4<T>
where
    T: Add<Output = T> + SimpleMathTrait,
{
    fn add_assign(&mut self, other: Mat4x4<T>) {
        self.inner[0][0] = self.inner[0][0] + other.inner[0][0];
        self.inner[0][1] = self.inner[0][1] + other.inner[0][1];
        self.inner[0][2] = self.inner[0][2] + other.inner[0][2];
        self.inner[0][3] = self.inner[0][3] + other.inner[0][3];

        self.inner[1][0] = self.inner[1][0] + other.inner[1][0];
        self.inner[1][1] = self.inner[1][1] + other.inner[1][1];
        self.inner[1][2] = self.inner[1][2] + other.inner[1][2];
        self.inner[1][3] = self.inner[1][3] + other.inner[1][3];

        self.inner[2][0] = self.inner[2][0] + other.inner[2][0];
        self.inner[2][1] = self.inner[2][1] + other.inner[2][1];
        self.inner[2][2] = self.inner[2][2] + other.inner[2][2];
        self.inner[2][3] = self.inner[2][3] + other.inner[2][3];
        self.inner[3][0] = self.inner[3][0] + other.inner[3][0];
        self.inner[3][1] = self.inner[3][1] + other.inner[3][1];
        self.inner[3][2] = self.inner[3][2] + other.inner[3][2];
        self.inner[3][3] = self.inner[3][3] + other.inner[3][3];
    }
}
impl<T> SubAssign<Mat4x4<T>> for Mat4x4<T>
where
    T: Sub<Output = T> + SimpleMathTrait,
{
    fn sub_assign(&mut self, other: Mat4x4<T>) {
        self.inner[0][0] = self.inner[0][0] - other.inner[0][0];
        self.inner[0][1] = self.inner[0][1] - other.inner[0][1];
        self.inner[0][2] = self.inner[0][2] - other.inner[0][2];
        self.inner[0][3] = self.inner[0][3] - other.inner[0][3];

        self.inner[1][0] = self.inner[1][0] - other.inner[1][0];
        self.inner[1][1] = self.inner[1][1] - other.inner[1][1];
        self.inner[1][2] = self.inner[1][2] - other.inner[1][2];
        self.inner[1][3] = self.inner[1][3] - other.inner[1][3];

        self.inner[2][0] = self.inner[2][0] - other.inner[2][0];
        self.inner[2][1] = self.inner[2][1] - other.inner[2][1];
        self.inner[2][2] = self.inner[2][2] - other.inner[2][2];
        self.inner[2][3] = self.inner[2][3] - other.inner[2][3];

        self.inner[3][0] = self.inner[3][0] - other.inner[3][0];
        self.inner[3][1] = self.inner[3][1] - other.inner[3][1];
        self.inner[3][2] = self.inner[3][2] - other.inner[3][2];
        self.inner[3][3] = self.inner[3][3] - other.inner[3][3];
    }
}
impl<T> Mul<T> for Mat4x4<T>
where
    T: Mul<Output = T> + SimpleMathTrait,
{
    type Output = Mat4x4<T>;
    fn mul(self, other: T) -> Self::Output {
        Mat4x4 {
            inner: [
                [
                    self.inner[0][0] * other,
                    self.inner[0][1] * other,
                    self.inner[0][2] * other,
                    self.inner[0][3] * other,
                ],
                [
                    self.inner[1][0] * other,
                    self.inner[1][1] * other,
                    self.inner[1][2] * other,
                    self.inner[1][3] * other,
                ],
                [
                    self.inner[2][0] * other,
                    self.inner[2][1] * other,
                    self.inner[2][2] * other,
                    self.inner[2][3] * other,
                ],
                [
                    self.inner[3][0] * other,
                    self.inner[3][1] * other,
                    self.inner[3][2] * other,
                    self.inner[3][3] * other,
                ],
            ],
        }
    }
}

impl<T> Div<T> for Mat4x4<T>
where
    T: Div<Output = T> + SimpleMathTrait,
{
    type Output = Mat4x4<T>;
    fn div(self, other: T) -> Self::Output {
        Mat4x4 {
            inner: [
                [
                    self.inner[0][0] / other,
                    self.inner[0][1] / other,
                    self.inner[0][2] / other,
                    self.inner[0][3] / other,
                ],
                [
                    self.inner[1][0] / other,
                    self.inner[1][1] / other,
                    self.inner[1][2] / other,
                    self.inner[1][3] / other,
                ],
                [
                    self.inner[2][0] / other,
                    self.inner[2][1] / other,
                    self.inner[2][2] / other,
                    self.inner[2][3] / other,
                ],
                [
                    self.inner[3][0] / other,
                    self.inner[3][1] / other,
                    self.inner[3][2] / other,
                    self.inner[3][3] / other,
                ],
            ],
        }
    }
}

impl<T> Mul for Mat4x4<T>
where
    T: Add<Output = T> + SimpleMathTrait + Mul<Output = T> + Zero<Type = T> + AddAssign,
{
    type Output = Mat4x4<T>;
    fn mul(self, other: Mat4x4<T>) -> Self::Output {
        let mut inner: [[T; 4]; 4] = [[T::zero(); 4]; 4];
        for (i, item) in inner.iter_mut().enumerate() {
            for (row, _) in self.inner.iter().enumerate() {
                for (col, _) in self.inner[0].iter().enumerate() {
                    item[row] += self.inner[i][col] * other.inner[col][row];
                }
            }
        }
        Mat4x4 { inner }
    }
}

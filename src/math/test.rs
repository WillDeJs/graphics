use crate::math::matrix::Mat3x3;
use crate::math::matrix::Mat4x4;
use crate::math::FVec2D;
use crate::math::IVec2D;

#[test]
fn vec2d_crossed() {
    let lhs = FVec2D::new(1.0, 2.0);
    let rhs = FVec2D::new(3.0, 4.0);

    assert_eq!(FVec2D::cross(lhs, rhs), -2.0);
    let lhs = IVec2D::new(1, 2);
    let rhs = IVec2D::new(3, 4);
    assert_eq!(IVec2D::cross(lhs, rhs), -2);
}

#[test]
fn vec2d_dotted() {
    let lhs = FVec2D::new(1.0, 2.0);
    let rhs = FVec2D::new(3.0, 4.0);

    assert_eq!(FVec2D::dot(lhs, rhs), 11.0);
    let lhs = IVec2D::new(1, 2);
    let rhs = IVec2D::new(3, 4);
    assert_eq!(IVec2D::dot(lhs, rhs), 11);
}
#[test]
fn vec2d_unit() {
    let lhs = FVec2D::new(3.0, 4.0);
    assert_eq!(lhs.unit_vector(), FVec2D::new(3.0 / 5.0, 4.0 / 5.0));

    let lhs = IVec2D::new(3, 4);
    assert_eq!(lhs.unit_vector(), IVec2D::new(3 / 5, 4 / 5));
}

#[test]
fn vec2d_neg() {
    let lhs = IVec2D::new(1, 4);
    assert_eq!(-lhs, lhs * -1);
    assert_eq!(-lhs, IVec2D::new(-1, -4));
}

#[test]
fn polar_test() {
    let lhs = IVec2D::from_polar(1, std::f32::consts::FRAC_PI_2);
    assert_eq!(lhs, IVec2D::new(1, 0));
    let lhs = IVec2D::from_polar(1, std::f32::consts::PI);
    assert_eq!(lhs, IVec2D::new(0, -1));
    let lhs = IVec2D::from_polar(8, std::f32::consts::FRAC_PI_6);
    assert_eq!(lhs, IVec2D::new(4, 6));
}

#[test]
fn matrix_3x3_addition() {
    let a: Mat3x3<i32> = [[10, 20, 10], [4, 5, 6], [2, 3, 5]].into();
    let b: Mat3x3<i32> = [[3, 2, 4], [3, 3, 9], [4, 4, 2]].into();
    let c: Mat3x3<i32> = [[13, 22, 14], [7, 8, 15], [6, 7, 7]].into();

    assert_eq!(a + b, c);
}

#[test]
fn matrix_3x3_subtraction() {
    let a: Mat3x3<i32> = [[10, 20, 10], [4, 5, 6], [2, 3, 5]].into();
    let b: Mat3x3<i32> = [[3, 2, 4], [3, 3, 9], [4, 4, 2]].into();
    let c: Mat3x3<i32> = [[13, 22, 14], [7, 8, 15], [6, 7, 7]].into();

    assert_eq!(c - a, b);
}
#[test]
fn matrix_3x3_multiplication() {
    let a: Mat3x3<i32> = [[10, 20, 10], [4, 5, 6], [2, 3, 5]].into();
    let b: Mat3x3<i32> = [[3, 2, 4], [3, 3, 9], [4, 4, 2]].into();
    let c: Mat3x3<i32> = [[130, 120, 240], [51, 47, 73], [35, 33, 45]].into();
    let d: Mat3x3<i32> = [[6, 4, 8], [6, 6, 18], [8, 8, 4]].into();

    assert_eq!(a * b, c);
    assert_eq!(b * 2, d);
}
#[test]
fn matrix_3x3_division() {
    let a: Mat3x3<i32> = [[3, 2, 4], [3, 3, 9], [4, 4, 2]].into();
    let b: Mat3x3<i32> = [[6, 4, 8], [6, 6, 18], [8, 8, 4]].into();

    assert_eq!(b / 2, a);
}

#[test]
fn matrix_3x3_inverse() {
    let a: Mat3x3<f32> = [[10.0, 20.0, 10.0], [4.0, 5.0, 6.0], [2.0, 3.0, 5.0]].into();
    let b: Mat3x3<f32> = [
        [-7.0 / 70.0, 1.0, -1.0],
        [8.0 / 70.0, -30.0 / 70.0, 20.0 / 70.0],
        [-2.0 / 70.0, -10.0 / 70.0, 30.0 / 70.0],
    ]
    .into();

    assert_eq!(a.inverse(), b);
}

#[test]
fn matrix_4x4_determinant() {
    let a: Mat4x4<i32> = [[1, 3, 5, 9], [1, 3, 1, 7], [4, 3, 9, 7], [5, 2, 0, 9]].into();
    assert_eq!(a.det(), -376);
}
#[test]
fn matrix_4x4_multiplication() {
    let a: Mat4x4<i32> = [[5, 7, 9, 10], [2, 3, 3, 8], [8, 10, 2, 3], [3, 3, 4, 8]].into();
    let b: Mat4x4<i32> = [
        [3, 10, 12, 18],
        [12, 1, 4, 9],
        [9, 10, 12, 2],
        [3, 12, 4, 10],
    ]
    .into();
    let c: Mat4x4<i32> = [
        [210, 267, 236, 271],
        [93, 149, 104, 149],
        [171, 146, 172, 268],
        [105, 169, 128, 169],
    ]
    .into();

    let d = [
        [10, 14, 18, 20],
        [4, 6, 6, 16],
        [16, 20, 4, 6],
        [6, 6, 8, 16],
    ]
    .into();
    assert_eq!(a * b, c);
    assert_eq!(a * 2, d);
}

#[test]
fn matrix_4x4_division() {
    let a: Mat4x4<i32> = [[5, 7, 9, 10], [2, 3, 3, 8], [8, 10, 2, 3], [3, 3, 4, 8]].into();

    let b: Mat4x4<i32> = [
        [10, 14, 18, 20],
        [4, 6, 6, 16],
        [16, 20, 4, 6],
        [6, 6, 8, 16],
    ]
    .into();
    assert_eq!(b / 2, a);
}

#[test]
fn matrix_4x4_addition() {
    let a: Mat4x4<i32> = [[5, 7, 9, 10], [2, 3, 3, 8], [8, 10, 2, 3], [3, 3, 4, 8]].into();

    let b: Mat4x4<i32> = [
        [10, 14, 18, 20],
        [4, 6, 6, 16],
        [16, 20, 4, 6],
        [6, 6, 8, 16],
    ]
    .into();
    assert_eq!(a + a, b);
}
#[test]
fn matrix_4x4_subtraction() {
    let a: Mat4x4<i32> = [[5, 7, 9, 10], [2, 3, 3, 8], [8, 10, 2, 3], [3, 3, 4, 8]].into();

    let b: Mat4x4<i32> = [
        [10, 14, 18, 20],
        [4, 6, 6, 16],
        [16, 20, 4, 6],
        [6, 6, 8, 16],
    ]
    .into();
    assert_eq!(b - a, a);
}

#[test]
fn matrix_4x4_inverse(){
    let a: Mat4x4<f32> = [[5.0, 7.0, 9.0, 10.0], [2.0, 3.0, 3.0, 8.0], [8.0, 10.0, 2.0, 3.0], [3.0, 3.0, 4.0, 8.0]].into();
    let ia : Mat4x4<f32> = Mat4x4::<f32>::from([[-71.0, -271.0, 26.0, 350.0], [51.0,215.0,22.0,-287.0],[71.0,-90.0,-26.0,11.0],[-28.0,66.0,-5.0, 16.0]]) / 361.0;

    assert_eq!(a.inverse(), ia);
}

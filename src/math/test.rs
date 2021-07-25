use crate::math::matrix::Mat3x3;
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

use crate::math::vector::FVec2D;
use crate::math::vector::IVec2D;
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

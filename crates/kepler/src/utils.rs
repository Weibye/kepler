use bevy_math::{Mat3, Quat, Vec3};
use bevy_transform::components::Transform;

use crate::{Ellipse, OrbitalPlane};


pub fn transform_from_axis(right: Vec3, up: Vec3, forward: Vec3) -> Transform {
    Transform {
        translation: Vec3::ZERO,
        rotation: quat_from_axes(right, up, forward),
        scale: Vec3::ONE,
    }
}

pub fn quat_from_axes(right: Vec3, up: Vec3, forward: Vec3) -> Quat {
    Quat::from_rotation_mat3(&Mat3::from_cols(right.normalize(), up.normalize(), forward.normalize()))
}

pub fn get_orbital_position_relative(plane: &OrbitalPlane, ellipse: &Ellipse, angle: f32) -> Vec3 {
    let rotation_offset = Quat::from_axis_angle(Vec3::Y, plane.periapsis_arg());
    let eccentricity_offset = Vec3::new(0.0, 0.0, 1.0) * ellipse.linear_eccentricity();

    let point = ellipse.perimeter_point(angle);
    let vec = Vec3::new(point.1, 0.0, point.0) - eccentricity_offset;
    return rotation_offset * vec;
}
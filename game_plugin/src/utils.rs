use bevy::{math::{Mat3, Quat, Vec3}, prelude::Transform};


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
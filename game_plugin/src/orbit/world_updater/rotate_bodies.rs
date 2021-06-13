use bevy::{core::Time, math::Quat, prelude::{Query, Res, Transform}};

use crate::orbit::OrbitalBody;

/// Rotates the orbital bodies along their own axis of rotation
pub(crate) fn rotate_orbital_bodies(
    mut query: Query<(&mut Transform, &OrbitalBody)>, 
    time: Res<Time>,
) {
    for (mut transform, body) in query.iter_mut() {
        let up = transform.local_y();
        transform.rotation *= Quat::from_axis_angle(up, body.angular_velocity * time.delta_seconds());
    }
}
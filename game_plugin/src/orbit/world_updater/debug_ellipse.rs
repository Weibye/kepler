use std::{f32::consts::PI, ops::Rem};

use bevy::{core::Time, math::{Quat, Vec3}, prelude::{Changed, Children, GlobalTransform, Parent, Query, Res, Transform, With, Without}};
use kepler::OrbitalPlane;

use crate::orbit::components::ReferenceFrame;


pub fn increase_periapsis_arg(mut q: Query<&mut OrbitalPlane>, time: Res<Time>) {
    for mut plane in q.iter_mut() {
        let rate = 0.5;
        let new_angle = rate * time.delta_seconds() as f32 + plane.periapsis_arg();
        let clamped_angle = new_angle.rem(2.0*PI);

        // println!("new_angle: {:?} : {:?}", new_angle, clamped_angle);
        plane.set_periapsis_arg(clamped_angle);
    }
}

pub fn increase_ascending_arg(mut q: Query<&mut OrbitalPlane>, time: Res<Time>) {
    for mut plane in q.iter_mut() {
        let rate = 0.5;
        let new_angle = rate * time.delta_seconds() as f32 + plane.ascending_arg();
        let clamped_angle = new_angle.rem(2.0*PI);

        // println!("new_angle: {:?} : {:?}", new_angle, clamped_angle);
        plane.set_ascending_arg(clamped_angle);
    }
}

pub fn increase_inclination_arg(mut q: Query<&mut OrbitalPlane>, time: Res<Time>) {
    for mut plane in q.iter_mut() {
        let rate = 0.5;
        let new_angle = rate * time.delta_seconds() as f32 + plane.inclination_arg();
        let clamped_angle = new_angle.rem(2.0*PI);

        // println!("new_angle: {:?} : {:?}", new_angle, clamped_angle);
        plane.set_inclination_arg(clamped_angle);
    }
}

pub fn update_orbital_plane_transform(
    mut q: Query<(&mut Transform, &OrbitalPlane), Changed<OrbitalPlane>>,
) {
    for (mut transform, plane) in q.iter_mut() {
        transform.rotation = plane.get_rot();
    }
}

pub fn rotate_refs(mut q: Query<&mut Transform, With<ReferenceFrame>>, time: Res<Time>) {
    for mut transform in q.iter_mut() {
        transform.rotation = Quat::from_axis_angle(Vec3::X, (0.2 * time.seconds_since_startup() as f32).rem(2.0 * PI));
    }
}
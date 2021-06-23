use std::{f32::consts::PI, ops::Rem};

use bevy::{core::Time, math::{Quat, Vec3}, prelude::{Changed, Children, GlobalTransform, Parent, Query, Res, Transform, With, Without}};
use kepler::{Ellipse, OrbitalBody, OrbitalPlane, get_orbital_position_relative};

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

pub fn incrase_eccentricity_arg(mut q: Query<&mut Ellipse>, time: Res<Time>) {
    for mut ellipse in q.iter_mut() {
        println!("Ellipse");
        let new_eccentricity = (1.0 + (time.seconds_since_startup() as f32).sin()) / 2.0; // / 2.0 + 1.0;
        println!("New Eccentricity {}", new_eccentricity);
        ellipse.set_eccentricity(new_eccentricity);

        println!("On object: {:?}", ellipse.eccentricity());
    }
}

/// Updates the orbital plane trasform based on the values from the OrbitalPlane
pub fn update_orbital_plane_transform(
    mut q: Query<(&mut Transform, &OrbitalPlane), Changed<OrbitalPlane>>,
) {
    for (mut transform, plane) in q.iter_mut() {
        transform.rotation = plane.get_rot();
    }
}

/// 
pub fn update_orbital_body_transform(
    mut q: Query<(&mut Transform, &Parent), With<OrbitalBody>>,
    p_q: Query<(&OrbitalPlane, &Ellipse), With<Children>>,
    time: Res<Time>,
) {
    for (mut transform, parent) in q.iter_mut() {
        if let Ok((p_plane, p_ellipse)) = p_q.get(parent.0) {
            let angle = get_cyclic_time(0.2, &time);
            transform.translation = get_orbital_position_relative(p_plane, p_ellipse, angle);
        }
    }
}



pub fn rotate_refs(mut q: Query<&mut Transform, With<ReferenceFrame>>, time: Res<Time>) {
    for mut transform in q.iter_mut() {
        transform.rotation = Quat::from_axis_angle(Vec3::X, (0.2 * time.seconds_since_startup() as f32).rem(2.0 * PI));
    }
}

pub fn get_cyclic_time(speed: f32, time: &Res<Time>) -> f32 {
    (speed * time.seconds_since_startup() as f32).rem(2.0 * PI)
}


use std::{f32::consts::PI, ops::Rem};

use bevy::{core::Time, prelude::{GlobalTransform, Parent, Query, Res, Transform, With}};

use crate::orbit::{OrbitalBody, orbit_parameters::{OrbitParameters, orbital_position_at_true_anomaly}};

pub(crate) fn move_orbital_bodies(
    mut q_child: Query<(&mut Transform, &Parent), With<OrbitalBody>>,
    q_parent: Query<(&OrbitParameters, &GlobalTransform)>,
    time: Res<Time>,
) {
    let speed = PI/30.0;
    let mut total_offset = speed * time.seconds_since_startup() as f32;
    if total_offset >= 2.*PI {
        total_offset = total_offset.rem(2.*PI);
    }

    for (mut transform, parent) in q_child.iter_mut() {
        let parent_result = q_parent.get(parent.0);
        match parent_result {
            Ok((orbit, global_transform)) => {
                let pos = orbital_position_at_true_anomaly(*orbit, orbit.true_anomaly + total_offset);

                transform.translation = pos;
            },
            Err(_) => continue
        }
    }
}
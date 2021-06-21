use std::{f32::consts::PI, ops::Rem, };

use bevy::{
    core::Time, 
    prelude::*
};
use kepler::{EllipticalOrbit, OrbitalBody};

use crate::orbit::{components::{ReferenceFrame}, orbit_parameters::{OrbitParameters, orbital_position_at_true_anomaly}};

pub(crate) fn move_orbital_bodies(
    mut bodies: Query<(&Parent, &mut Transform, &OrbitalBody), Without<ReferenceFrame>>,
    orbits: Query<(&EllipticalOrbit, &Parent), With<Children>>,
    refs: Query<(&ReferenceFrame, &Transform)>,
) {
    for (parent, mut transform, body) in bodies.iter_mut() {
        if let Ok((orbit, grand_parent)) = orbits.get(parent.0) {
            if let Ok((reference, ref_transform)) = refs.get(grand_parent.0) {
                // let new_position = orbit.get_position_vector(ref_transform);
                // transform.translation = new_position;
            }
        }
    }
}
use std::f32::consts::PI;

use bevy::{math::{Quat, Vec3}, prelude::{Children, Color, Entity, GlobalTransform, Parent, Query, Res, ResMut, Transform, With}};
use bevy_prototype_debug_lines::DebugLines;
use kepler::{Ellipse, OrbitalDirectionsGlobal, OrbitalDirectionsLocal, OrbitalPlane, get_orbital_position_relative};

use crate::player::orbit_picker::OrbitTarget;

pub(crate) fn draw_orbit_lines(
    query: Query<(&Ellipse, &OrbitalPlane, &GlobalTransform)>,
    mut lines: ResMut<DebugLines>,
    selected_orbit: Res<OrbitTarget>,
) {
    let color = Color::rgb(0.8, 0.8, 0.8);
    let steps = 24;
    let step_angle = 2. * PI / steps as f32;
    // let ring_color = match selected_orbit {
    
    for (ellipse, orbital_plane, self_global_transform) in query.iter() {
        let mut positions: Vec<Vec3> = Vec::new();

        for n in 0..steps {
            let pos = get_orbital_position_relative(orbital_plane, ellipse, step_angle * n as f32);
            let rotated = self_global_transform.rotation * pos;
            let offset = self_global_transform.translation + rotated;

            positions.push(offset);
        }

        for n in 0..positions.len() {
            let current = positions[n];
            let next;
            if n == positions.len() - 1 {
                next = positions[0];
            } else {
                next = positions[n+1];
            }
            
            lines.line_colored(current, next, 0.0, color);
        }

            // Ascending / Desceding
            // lines.line_colored(transform.translation, transform.translation + orbital_plane.ascending_global(glob_ref), 0.0, Color::rgb(0.6, 0.0, 1.0));
            // lines.line_colored(transform.translation, transform.translation + orbital_plane.descending_global(glob_ref), 0.0, Color::rgb(0.3, 0.0, 7.0));

            // // Apoapsis / Periapsis
            // lines.line_colored(transform.translation, transform.translation + orbital_plane.periapsis_global(glob_ref) * 2.0, 0.0, Color::rgb(0.0, 1.0, 0.3));
            // lines.line_colored(transform.translation, transform.translation + orbital_plane.apoapsis_global(glob_ref) * 2.0, 0.0, Color::rgb(0.0, 0.8, 0.3));

            // // Zenith / Nadir
            // lines.line_colored(transform.translation, transform.translation + orbital_plane.zenith_global(glob_ref) * 2.0, 0.0, Color::rgb(1.0, 0.0, 0.2));
            // lines.line_colored(transform.translation, transform.translation + orbital_plane.nadir_global(glob_ref) * 2.0, 0.0, Color::rgb(0.8, 0.0, 0.2));

            // Forward
            // lines.line_colored(Vec3::ZERO, transform.local_z() * 3.0, 0.0, Color::rgb(1., 1.0, 0.0));
        // }
    }


    // for (e, orbit) in query.iter() {
    //     let color = match selected_orbit.selection {
    //         Some(selected_entity) if e == selected_entity => Color::rgb(1.0, 1.0, 1.0),
    //         Some(_) => Color::rgb(0.2, 0.2, 0.2),
    //         None => Color::rgb(0.2, 0.2, 0.2),
    //     };

        // lines.line_colored(Vec3::ZERO, orbital_position_at_true_anomaly(*orbit, orbit.true_anomaly), 0.0, Color::rgb(1., 1., 1.));

    // }
}
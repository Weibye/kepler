use std::f32::consts::PI;

use bevy::{math::{Quat, Vec3}, prelude::{Children, Color, Entity, GlobalTransform, Parent, Query, Res, ResMut, Transform, With}};
use bevy_prototype_debug_lines::DebugLines;
use kepler::{Ellipse, OrbitalDirectionsGlobal, OrbitalDirectionsLocal, OrbitalPlane};

use crate::orbit::orbit_parameters::{OrbitParameters, orbital_position_at_true_anomaly};
use crate::player::orbit_picker::OrbitTarget;

pub(crate) fn draw_orbit_lines(
    query: Query<(&Ellipse, &OrbitalPlane, &GlobalTransform)>,
    // q_reference_frames: Query<(&Transform, &GlobalTransform), With<Children>>,
    mut lines: ResMut<DebugLines>,
    selected_orbit: Res<OrbitTarget>,
) {
    let color = Color::rgb(0.8, 0.8, 0.8);
    let steps = 24;
    let step_angle = 2. * PI / steps as f32;
    // let ring_color = match selected_orbit {
    
    for (ellipse, orbital_plane, self_global_transform) in query.iter() {
        let mut positions: Vec<Vec3> = Vec::new();

        let periapsis_offset_rot = Quat::from_axis_angle(Vec3::Y, orbital_plane.periapsis_arg());

        let perimeter_offset_vec = Vec3::new(0.0, 0.0, 1.0) * ellipse.linear_eccentricity();

        for n in 0..steps {
            let point = ellipse.perimeter_point(step_angle * n as f32);
            let vec = Vec3::new(point.1, 0.0, point.0) - perimeter_offset_vec;
            let rotated = self_global_transform.rotation * periapsis_offset_rot * vec;
            let position_offset = rotated + self_global_transform.translation;

            // let transformed = transform.translation + ellipse_local_rot_offset * transform.rotation * ;
            positions.push(position_offset);
        }

        for n in 0..positions.len() {
            let current = positions[n];
            let next;
            if n == positions.len() - 1 {
                next = positions[0];
            } else {
                next = positions[n+1];
            }
            
            // let start = Vec3::new(current.0, 0.0, current.1);
            // let end = Vec3::new(next.0, 0.0, next.1);
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
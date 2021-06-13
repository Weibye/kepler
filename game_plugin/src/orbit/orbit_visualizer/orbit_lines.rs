use std::f32::consts::PI;

use bevy::{math::Vec3, prelude::{Color, Entity, Query, Res, ResMut}};
use bevy_prototype_debug_lines::DebugLines;

use crate::orbit::orbit::{Orbit, OrbitalPositions, orbital_position_at_true_anomaly};
use crate::player::orbit_picker::OrbitTarget;

pub(crate) fn draw_orbit_lines(
    query: Query<(Entity, &Orbit)>,
    mut lines: ResMut<DebugLines>,
    selected_orbit: Res<OrbitTarget>,
) {
    let steps = 24;
    let step_angle = 2. * PI / steps as f32;
    // let ring_color = match selected_orbit {
        
    // };

    for (e, orbit) in query.iter() {

        // Ascending / Desceding
        lines.line_colored(Vec3::ZERO, orbit.ascending_node(), 0.0, Color::rgb(0.5, 0.0, 1.0));
        lines.line_colored(Vec3::ZERO, orbit.descending_node(), 0.0, Color::rgb(0.5, 0.0, 1.0));

        // Apoapsis / Periapsis
        lines.line_colored(Vec3::ZERO, orbit.periapsis_node(), 0.0, Color::rgb(0.3, 1.0, 0.0));
        lines.line_colored(Vec3::ZERO, orbit.apoapsis_node(), 0.0, Color::rgb(0.3, 1.0, 0.0));

        lines.line_colored(Vec3::ZERO, orbital_position_at_true_anomaly(*orbit, orbit.true_anomaly), 0.0, Color::rgb(1., 1., 1.));

        let line_positions = orbit.orbit_ring();

        for n in 0..line_positions.len() {
            let current = line_positions[n];
            let next;
            if n == line_positions.len() - 1 {
                next = line_positions[0];
            } else {
                next = line_positions[n+1];
            }
            
            lines.line(current, next, 0.0);
        }
    }
}
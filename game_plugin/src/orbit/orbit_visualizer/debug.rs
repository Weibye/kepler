use bevy::{math::{Vec2, Vec3}, prelude::{Children, Color, GlobalTransform, Parent, Query, ResMut, Transform, With, Without}};
use bevy_prototype_debug_lines::DebugLines;
use kepler::OrbitalPlane;

use crate::orbit::components::ReferenceFrame;

/// Draws a grid around a transform
pub(crate) fn draw_reference_frame(
    query: Query<&GlobalTransform, With<ReferenceFrame>>,
    mut lines: ResMut<DebugLines>,
) {
    let count = 20;
    let size = Vec2::ONE * 5.0;
    let brightness = 0.2;
    let color = Color::rgb(brightness, brightness, brightness);

    for transform in query.iter() {
        let grid_lines = get_grid_lines(size, count, transform);

        for line in grid_lines {
            lines.line_colored(line.0, line.1, 0.0, color);
        }
        
        // AXIS
        // lines.line_colored(transform.translation, transform.translation + transform.local_x() * size.x, 0.0, Color::RED);
        // lines.line_colored(transform.translation, transform.translation + transform.local_y() * size.x, 0.0, Color::GREEN);
        // lines.line_colored(transform.translation, transform.translation + transform.local_z() * size.y, 0.0, Color::BLUE);
    }
}

pub fn draw_orbital_plane(
    query: Query<&GlobalTransform, With<OrbitalPlane>>,
    mut lines: ResMut<DebugLines>,
) {
    for transform in query.iter() {
        // if let Ok(reference)
        let size = Vec2::new(2.5, 2.5);
        let count = 10;
        let grid_lines = get_grid_lines(size, count, transform);
        for line in grid_lines {
            lines.line_colored(line.0, line.1, 0.0, Color::ORANGE);
        }
    }
}


pub fn draw_axis(
    q: Query<&GlobalTransform>, //, With<ReferenceFrame>>,
    mut lines: ResMut<DebugLines>
) {
    let size = 7.0;

    for transform in q.iter() {
        // AXIS
        lines.line_colored(transform.translation, transform.translation + transform.local_x() * size, 0.0, Color::RED);
        lines.line_colored(transform.translation, transform.translation + transform.local_y() * size, 0.0, Color::GREEN);
        lines.line_colored(transform.translation, transform.translation + transform.local_z() * size, 0.0, Color::BLUE);
    }
}

fn get_grid_lines(size: Vec2, grid_count: i32, transform: &GlobalTransform) -> Vec<(Vec3, Vec3)> {
    let mut end_points: Vec<(Vec3, Vec3)> = Vec::new();

    let x_step = 2. * size.x / grid_count as f32;
    let y_step = 2. * size.y / grid_count as f32;

    // Offset to bring the grid to the center of the transform
    let offset = transform.local_x() * -size.x + transform.local_z() * -size.y;

    // X - lines
    for n in 0..=grid_count {
        let start = 
            transform.translation
            + transform.local_x() * x_step * n as f32
            + offset
        ;
        let end = 
            transform.translation
            + transform.local_z() * size.y * 2.
            + transform.local_x() * x_step * n as f32
            + offset
        ;

        end_points.push((start, end));
    }

    // Z-lines
    for n in 0..=grid_count {
        let start = 
            transform.translation
            + transform.local_z() * y_step * n as f32
            + offset
        ;

        let end = 
            transform.translation
            + transform.local_x() * size.x * 2.
            + transform.local_z() * y_step * n as f32
            + offset
        ;

        end_points.push((start, end));
    }

    return end_points;
}
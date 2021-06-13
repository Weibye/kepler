use bevy::{math::{Vec2, Vec3}, prelude::{Color, Query, ResMut, Transform, With}};
use bevy_prototype_debug_lines::DebugLines;

use crate::orbit::components::ReferenceFrame;

/// Draws a grid around a transform
pub(crate) fn draw_reference_frame(
    query: Query<&Transform, With<ReferenceFrame>>,
    mut lines: ResMut<DebugLines>,
) {
    let grid_count = 9;
    let size = Vec2::ONE * 1.0;
    let brightness = 0.2;
    let color = Color::rgb(brightness, brightness, brightness);

    for transform in query.iter() {
        let x_step = 2. * size.x / grid_count as f32;
        let y_step = 2. * size.y / grid_count as f32;

        let mut end_points: Vec<(Vec3, Vec3)> = Vec::new();
        
        // X - lines
        for n in 0..=grid_count {
            let start = 
                Vec3::ZERO
                + transform.local_x() * x_step * n as f32;
            let end = 
                transform.local_z() * size.y * 2.
                + transform.local_x() * x_step * n as f32;

            end_points.push((start, end));
        }

        for n in 0..=grid_count {
            let start = 
                Vec3::ZERO
                + transform.local_z() * y_step * n as f32;
            let end = 
                transform.local_x() * size.x * 2.
                + transform.local_z() * y_step * n as f32;

            end_points.push((start, end));
        }

        let offset = transform.local_x() * -size.x + transform.local_z() * -size.y;
        for (start, end) in end_points.iter() {
            lines.line_colored(
                *start  + transform.translation + offset, 
                *end  + transform.translation + offset, 
                0.0, 
                color
            );
        }

        // AXIS
        lines.line_colored(transform.translation, transform.translation + transform.local_x() * size.x, 0.0, Color::RED);
        lines.line_colored(transform.translation, transform.translation + transform.local_y() * size.x, 0.0, Color::GREEN);
        lines.line_colored(transform.translation, transform.translation + transform.local_z() * size.y, 0.0, Color::BLUE);
    }
}
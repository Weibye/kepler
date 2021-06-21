mod orbit_lines;
mod debug;

use orbit_lines::draw_orbit_lines;
// use debug::{draw_reference_frame, draw_orbital_plane};

use bevy::prelude::{AppBuilder, CoreStage, IntoSystem, Plugin};

use self::debug::*;

pub struct OrbitVisualizerPlugin;

impl Plugin for OrbitVisualizerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_to_stage(
                CoreStage::PostUpdate,
                draw_orbit_lines.system(),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                draw_reference_frame.system(),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                draw_orbital_plane.system(),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                draw_axis.system(),
            )
        ;
    }
}
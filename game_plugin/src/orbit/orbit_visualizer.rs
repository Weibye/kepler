mod orbit_lines;

use orbit_lines::draw_orbit_lines;

use bevy::prelude::{AppBuilder, CoreStage, IntoSystem, Plugin};

pub struct OrbitVisualizerPlugin;

impl Plugin for OrbitVisualizerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_to_stage(
                CoreStage::PostUpdate,
                draw_orbit_lines.system(),
            )
        ;
    }
}
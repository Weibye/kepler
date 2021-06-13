mod orbit_parameters;
mod world_generator;
mod world_updater;
mod orbit_visualizer;
mod bundles;
pub(crate) mod components;

use world_generator::WorldGeneratorPlugin;
use world_updater::WorldUpdaterPlugin;
use orbit_visualizer::OrbitVisualizerPlugin;

use bevy::prelude::{AppBuilder, Plugin};
use bevy_prototype_debug_lines::DebugLinesPlugin;

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(DebugLinesPlugin)
            .add_plugin(WorldGeneratorPlugin)
            .add_plugin(WorldUpdaterPlugin)
            .add_plugin(OrbitVisualizerPlugin)
        ;
    }
}

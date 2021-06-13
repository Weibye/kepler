mod orbit_parameters;
mod world_generator;
mod world_updater;
mod orbit_visualizer;

use world_generator::WorldGeneratorPlugin;
use world_updater::WorldUpdaterPlugin;
use orbit_visualizer::OrbitVisualizerPlugin;
use orbit_parameters::OrbitParameters;

use bevy::{math::Vec3, prelude::{AppBuilder, Plugin}};
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

pub struct ReferenceFrame;
// pub struct ReferenceFrame {
//     forward: Vec3,
//     up: Vec3,
//     position: Vec3,
// }

#[derive(Copy, Clone)]
pub struct OrbitalBody {
    mass: f32,
    radius: f32,
    angular_velocity: f32,
}

pub struct Sun;

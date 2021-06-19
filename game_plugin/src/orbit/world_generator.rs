mod spawn_world;
mod generate_world;
mod ellipse_create;

use kepler::OrbitalBody;
use spawn_world::*;
use ellipse_create::*;

use bevy::prelude::{AppBuilder, IntoSystem, Plugin, Transform};

use super::{
    orbit_parameters::OrbitParameters
};


pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(WorldGenerationSettings {})
            .add_startup_system(spawn_world.system())
            .add_startup_system(create_ellipse.system())
        ;
    }
}

struct WorldGenerationSettings {

}

#[derive(Debug)]
struct HierarchyNode {
    node: OrbitNode,
    children: Option<Vec<HierarchyNode>>
}
#[derive(Debug)]
struct RootNode {
    reference_frame: Transform,
    body: OrbitalBody,
    children: Option<Vec<HierarchyNode>>
}

#[derive(Debug)]
struct OrbitNode {
    orbit: OrbitParameters,
    reference_frame: Transform,
    body: OrbitalBody,
}
// enum SystemOrbitNode {
//     Orbit(OrbitNode),
//     Body(OrbitalBody)
// }
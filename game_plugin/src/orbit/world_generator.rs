mod spawn_world;

use spawn_world::spawn_orbits;

use bevy::prelude::{AppBuilder, IntoSystem, Plugin};


pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_orbits.system())
        ;
    }
}
mod update_orbits;
mod rotate_bodies;

use update_orbits::move_orbital_bodies;
use rotate_bodies::rotate_orbital_bodies;

use bevy::prelude::{AppBuilder, IntoSystem, Plugin};

pub struct WorldUpdaterPlugin;

impl Plugin for WorldUpdaterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // .add_system(move_orbital_bodies.system())
            .add_system(rotate_orbital_bodies.system())
        ;
    }
}
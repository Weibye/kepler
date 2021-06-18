mod move_bodies;
mod rotate_bodies;
mod orbit_driver;

use move_bodies::move_orbital_bodies;
use rotate_bodies::rotate_orbital_bodies;
use orbit_driver::drive_orbits;

use bevy::prelude::{AppBuilder, IntoSystem, ParallelSystemDescriptorCoercion, Plugin};

pub struct WorldUpdaterPlugin;

impl Plugin for WorldUpdaterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(drive_orbits.system().label("driver"))
            .add_system(move_orbital_bodies.system().after("driver"))
            .add_system(rotate_orbital_bodies.system().after("driver"))
        ;
    }
}
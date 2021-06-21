mod move_bodies;
mod rotate_bodies;
mod orbit_driver;
mod debug_ellipse;

use move_bodies::move_orbital_bodies;
use rotate_bodies::rotate_orbital_bodies;
use orbit_driver::drive_orbits;
use debug_ellipse::{increase_periapsis_arg, increase_ascending_arg, increase_inclination_arg, update_orbital_plane_transform};

use bevy::prelude::{AppBuilder, IntoSystem, ParallelSystemDescriptorCoercion, Plugin};

use self::debug_ellipse::rotate_refs;

pub struct WorldUpdaterPlugin;

impl Plugin for WorldUpdaterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // .add_system(drive_orbits.system().label("driver"))
            // .add_system(move_orbital_bodies.system().after("driver"))
            // .add_system(rotate_orbital_bodies.system().after("driver"))

            // .add_system(increase_periapsis_arg.system().before("last"))
            // .add_system(increase_ascending_arg.system().before("last"))
            // .add_system(increase_inclination_arg.system().before("last"))

            .add_system(rotate_refs.system())
            .add_system(update_orbital_plane_transform.system().label("last"))
        ;
    }
}
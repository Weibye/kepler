mod orbit_plugin;
mod orbit;

use crate::orbit_plugin::OrbitPlugin;

use bevy::app::AppBuilder;
use bevy::prelude::Plugin;

use bevy_flycam::PlayerPlugin;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum GameState {
    Loading,
    Playing,
    Menu
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(GameState::Loading)
            .add_plugin(PlayerPlugin)
            .add_plugin(OrbitPlugin);
    }
}
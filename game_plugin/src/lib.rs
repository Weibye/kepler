mod orbit_plugin;
mod orbit;
mod player;
mod utils;

use bevy::{
    app::AppBuilder, 
    prelude::*,
};
// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use orbit::OrbitPlugin;
use player::PlayerPlugin;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    /// Loading of all the things in the world
    Loading,
    /// Runtime play-state of the game
    Playing,
    /// Menu state of the game
    Menu
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Plugins
            .add_plugin(OrbitPlugin)
            .add_plugin(PlayerPlugin)
            // .add_plugin(CameraPlugin)
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            // App state & flow
            // .add_state(GameState::Loading)
            // .add_system_set(
            //     SystemSet::on_enter(GameState::Loading)
            //         .with_system(entering_state.system().label("start").before("end"))
            //         .with_system(transition_post_load.system().label("end").after("start"))
            // )
            // .add_system_set(
            //     SystemSet::on_exit(GameState::Loading)
            //         .with_system(exiting_state.system().label("start").before("end"))
            // )
            // .add_system_set(
            //     SystemSet::on_enter(GameState::PostLoad)
            //         .with_system(entering_state.system().label("start").before("end"))
            //         .with_system(transition_playing.system().label("end").after("start"))
            // )
            // .add_system_set(
            //     SystemSet::on_exit(GameState::PostLoad)
            //         .with_system(exiting_state.system().label("start").before("end"))
            // )
            // .add_system_set(
            //     SystemSet::on_enter(GameState::Playing)
            //         .with_system(entering_state.system().label("start").before("end"))
            // )
            // .add_system_set(
            //     SystemSet::on_exit(GameState::Playing)
            //         .with_system(exiting_state.system().label("start").before("end"))
            // )
            // .add_system_set(
            //     SystemSet::on_enter(GameState::Menu)
            //         .with_system(entering_state.system().label("start").before("end"))
            // )
            // .add_system_set(
            //     SystemSet::on_exit(GameState::Menu)
            //         .with_system(exiting_state.system().label("start").before("end"))
            // )
        ;
    }
}


// fn entering_state(state: Res<State<GameState>>) {
//     // info!("Entering {:?}", state);
// }
// fn exiting_state(state: Res<State<GameState>>) {
//     // info!("Exiting {:?}", *state)
// }

// fn transition_post_load(mut game_state: ResMut<State<GameState>>) {
//     info!("Transition to{:?}", GameState::PostLoad);
//     game_state.set(GameState::PostLoad).unwrap();
// }

// fn transition_playing(mut game_state: ResMut<State<GameState>>) {
//     info!("Transition to{:?}", GameState::Playing);
//     game_state.set(GameState::Playing).unwrap();
// }

// fn transition_menu(mut game_state: ResMut<State<GameState>>) {
//     info!("Transition to{:?}", GameState::Menu);
//     game_state.set(GameState::Menu).unwrap();
// }
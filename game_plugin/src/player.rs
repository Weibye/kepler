/**
* Hoviering over a planet should highlight the planet and its orbit
* Selecting a planet should make the highlight stay on until deletected
* Touble clicking should center the camera on to the 
*/

mod camera_controller;
mod orbit_picker;
mod input;

use bevy::prelude::{
    AppBuilder, 
    Plugin
};

use self::{camera_controller::CameraControllerPlugin, input::InputPlugin, orbit_picker::OrbitPickerPlugin};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(InputPlugin)
            .add_plugin(OrbitPickerPlugin)
            .add_plugin(CameraControllerPlugin)
        ;
    }
}
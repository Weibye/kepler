mod setup_camera;
mod update_camera;

use self::{
    setup_camera::setup_camera,
    update_camera::update_camera,
};

use bevy::prelude::*;
use smooth_bevy_cameras::LookTransformPlugin;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(CameraSettings {
                min_radius: 1.5,
                max_radius: 20.0,
                scroll_speed: 0.25,
            })
            .add_plugin(LookTransformPlugin)
            .add_startup_system(setup_camera.system())
            .add_system(update_camera.system())
        ;
    }
}

pub(crate) struct CameraRadius(pub f32);
pub(crate) struct CameraSettings {
    pub min_radius: f32,
    pub max_radius: f32,
    pub scroll_speed: f32,
}
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
                init_radius: 3.5,
                init_direction: Vec3::new(1.0, 1.5, 0.0).normalize(),
                min_radius: 0.7,
                max_radius: 25.0,
                scroll_speed: 0.35,
                smooth_factor: 0.90,
            })
            .add_plugin(LookTransformPlugin)
            .add_startup_system(setup_camera.system())
            .add_system(update_camera.system())
        ;
    }
}

pub(crate) struct CameraRadius(pub f32);
pub(crate) struct CameraSettings {
    pub init_radius: f32,
    pub init_direction: Vec3,
    pub min_radius: f32,
    pub max_radius: f32,
    pub scroll_speed: f32,
    pub smooth_factor: f32,
}
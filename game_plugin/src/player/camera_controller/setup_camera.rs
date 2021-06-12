use bevy::{math::Vec3, prelude::{Commands, PerspectiveCameraBundle, Res}};
use bevy_mod_picking::PickingCameraBundle;
use smooth_bevy_cameras::{
    LookTransform, 
    LookTransformBundle, 
    Smoother
};

use super::{CameraRadius, CameraSettings};

pub(crate) fn setup_camera(
    mut commands: Commands,
    settings: Res<CameraSettings>,
) {
    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle::default())
        .insert_bundle(LookTransformBundle {
            transform: LookTransform {
                eye: Vec3::ZERO + settings.init_direction * settings.init_radius,
                target: Vec3::ZERO
            },
            smoother: Smoother::new(settings.smooth_factor)
        })
        .insert_bundle(PickingCameraBundle::default())
        .insert(CameraRadius(settings.init_radius))
    ;
}
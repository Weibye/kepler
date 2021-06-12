use bevy::{
    math::Vec3, 
    prelude::{
        Commands, 
        PerspectiveCameraBundle
    }
};
use bevy_mod_picking::PickingCameraBundle;
use smooth_bevy_cameras::{
    LookTransform, 
    LookTransformBundle, 
    Smoother
};

use super::CameraRadius;

pub(crate) fn setup_camera(
    mut commands: Commands,
) {
    let radius = 3.0;
    let direction = Vec3::new(1.0, 1.5, 0.0).normalize();

    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle::default())
        .insert_bundle(LookTransformBundle {
            transform: LookTransform {
                eye: Vec3::ZERO + direction * radius,
                target: Vec3::ZERO
            },
            smoother: Smoother::new(0.9)
        })
        .insert_bundle(PickingCameraBundle::default())
        .insert(CameraRadius(radius))
    ;
}
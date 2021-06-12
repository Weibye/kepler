use bevy::{
    core::Time,
    ecs::system::Query,
    math::{
        Vec3,
        Mat4,
    }, 
    prelude::{
        Entity, 
        Res, 
        Transform, 
        With
    }
};

use smooth_bevy_cameras::LookTransform;

use crate::{orbit_plugin::OrbitalBody, player::{input::Actions, orbit_picker::OrbitTarget}};

use super::{CameraRadius, CameraSettings};

pub(crate) fn update_camera(
    mut q_camera: Query<(&mut LookTransform, &mut CameraRadius)>,
    q_targets: Query<(Entity, &Transform), With<OrbitalBody>>,
    time: Res<Time>,
    actions: Res<Actions>,
    settings: Res<CameraSettings>,
    orbit_target: Res<OrbitTarget>,
    
) {
    for (mut camera, mut radius) in q_camera.iter_mut() {

        let mut direction = (camera.eye - camera.target).normalize();

        // Target
        match orbit_target.body {
            Some(target_entity) => {
                if let Ok((_, target_transform)) = q_targets.get(target_entity) {
                    camera.target = target_transform.translation;
                }
            },
            None => camera.target = Vec3::ZERO,
        }

        // Scroll axis input
        if let Some(scroll) = actions.scroll {
            radius.0 -= scroll * settings.scroll_speed;

            if radius.0 < settings.min_radius { radius.0 = settings.min_radius; }
            else if radius.0 > settings.max_radius { radius.0 = settings.max_radius; }
        }

        match actions.mouse_movement {
            Some(delta) => {
                direction = Mat4::from_axis_angle(
                    Vec3::Y, 
                    -delta.x * 0.004
                ).transform_vector3(direction).normalize();
                direction = Mat4::from_axis_angle(
                    Vec3::Y.cross(direction),
                    -delta.y * 0.01
                ).transform_vector3(direction).normalize();
            },
            None => (),
        }
        camera.eye = camera.target + direction * radius.0;
    }
}
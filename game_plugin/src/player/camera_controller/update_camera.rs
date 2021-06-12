
use bevy::{core::Time, ecs::system::Query, math::{
        Vec3,
        Mat4,
    }, prelude::{Entity, GlobalTransform, Res, Transform, With}};
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
        let speed = 0.15;

        let direction = (camera.eye - camera.target).normalize();

        // Target
        match orbit_target.body {
            Some(target_entity) => {
                if let Ok((_, target_transform)) = q_targets.get(target_entity) {
                    camera.target = target_transform.translation;
                }
            },
            None => camera.target = Vec3::ZERO,
        }
        // if let Some(movement) = actions.player_movement {
        //     // println!("Movement: {:?}", movement);
        //     camera.target += Vec3::new(movement.x, 0.0, movement.y) * speed * 0.3;
        // }

        // Scroll axis input
        if let Some(scroll) = actions.scroll {
            radius.0 -= scroll * settings.scroll_speed;

            if radius.0 < settings.min_radius { radius.0 = settings.min_radius; }
            else if radius.0 > settings.max_radius { radius.0 = settings.max_radius; }
        }

        // let rot = Mat4::from_axis_angle(Vec3::Y, time.delta_seconds() * speed);
        // camera.eye = rot.transform_vector3(direction * radius.0) // * radius.0); //  * Vec3::X * 2.0;
        camera.eye = camera.target + direction * radius.0;
    }
}
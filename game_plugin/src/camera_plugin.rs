use bevy::{input::mouse::{MouseMotion, MouseWheel}, math::{Mat3, Mat4, Quat, Vec3}, prelude::*, render::camera::PerspectiveProjection};
use bevy_mod_picking::*;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother, controllers::{orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin}, unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin}}};

use crate::{GameState, orbit_plugin::{OrbitalBody, Sun}};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(LookTransformPlugin)
            .add_plugin(OrbitCameraPlugin)
            // Resources
            .insert_resource(PlayerCameraSettings {
                offset: Vec3::ONE * 2.0 
            })

            .add_system_set(
                SystemSet::on_enter(GameState::Loading)
                    
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PostLoad)
                    // .with_system(make_oribtal_bodies_pickable.system().before("end").after("start"))
                    .with_system(camera_setup.system().before("end").after("start"))
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    // .with_system(set_camera_target.system().after("end"))
                    // .with_system(move_camera.system().after("end"))
                    // .with_system(move_camera_to_target.system().after("end"))
                    // .with_system(pan_orbit_camera.system().after("end"))
            )
        ;
    }
}

#[derive(Debug, Copy, Clone)]
struct PlayerCamera {
    target: Entity, // Should be an option in this case. 
    radius: f32,
}

#[derive(Debug, Copy, Clone, Default)]
struct PlayerCameraSettings {
    pub offset: Vec3
}

// Spawn camera
fn camera_setup(
    mut commands: Commands,
    q: Query<(Entity, &GlobalTransform), With<Sun>>,
    settings: Res<PlayerCameraSettings>,
) {
    if let Ok((target, target_transform)) = q.single() {
        commands
            // .spawn_bundle(PerspectiveCameraBundle {
            //     transform: Transform {
            //         translation: settings.offset,
            //         rotation: quat_look_at(settings.offset - target_transform.translation, Vec3::Y),
            //         ..Default::default()
            //     },
            //     ..Default::default()
            // })
            // Mark for picking with this camera
            
            .spawn_bundle(OrbitCameraBundle::new(
                OrbitCameraController::default(),
                PerspectiveCameraBundle::default(),
                Vec3::new(-2.0, 5.0, 5.0),
                Vec3::ZERO
            ))
            .insert_bundle(PickingCameraBundle::default())
            //  {
            //     transform: LookTransform { 
            //         eye: Vec3::ONE * 5.0, 
            //         target: Vec3::ZERO 
            //     },
            //     smoother: Smoother::new(0.0)
            // })


            // .insert(BoundVol::default())
            // .insert(PlayerCamera {
            //     target,
            //     radius: (settings.offset - target_transform.translation).length(),
            // })
        ;
    } else {
        panic!("Found no suitable targets");
    }
}


/// _Not working correctly
fn quat_look_at_4(eye_pos: Vec3, focal_point: Vec3, up: Vec3) -> Quat {
    return Quat::from_rotation_mat4(&Mat4::look_at_rh(eye_pos, focal_point, up));
}
/// Create a new quaterion using a forward and up vector
fn quat_look_at(forward: Vec3, up: Vec3) -> Quat {
    
    let axis_forward = forward.normalize();
    let axis_right = up.cross(axis_forward).normalize();
    let axis_up = axis_forward.cross(axis_right).normalize();
    
    return Quat::from_rotation_mat3(
        &Mat3::from_cols(
            axis_right, 
            axis_up, 
            axis_forward)
        )
    ;
}



fn set_camera_target(
    mut events: EventReader<PickingEvent>,
    q_targets: Query<Entity, With<Selection>>,
    mut q_camera: Query<&mut PlayerCamera>,
) {
    if let Ok(mut player_camera) = q_camera.single_mut() {
        for event in events.iter() {
            match event {
                PickingEvent::Selection(selection) => {
                    match selection {
                        SelectionEvent::JustSelected(just_selected) => {

                            if *just_selected == player_camera.target {
                                continue;
                            } else {
                                if let Ok(target_entity) = q_targets.get(*just_selected) {
                                    player_camera.target = target_entity;
                                } else {
                                    error!("Didn't find selected target within query");
                                }
                            }
                        }
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    } else {
        error!("Found no player cameras in scene");
    }
    
}

fn move_camera_action(

) {

}

// fn move_camera_to_target(
//     mut q_camera: Query<(&mut Transform, &PlayerCamera)>,
//     q_targets: Query<(Entity, &GlobalTransform), With<OrbitalBody>>,
//     settings: Res<PlayerCameraSettings>,
// ) {
//     if let Ok((mut transform, player_cam)) = q_camera.single_mut() {
//         if let Ok((_, target_transform)) = q_targets.get(player_cam.target) {
//             transform.translation = target_transform.translation + settings.offset;
//         } else {
//             error!("Found no suitable targets");
//         }
//     } else {
//         panic!("Expected exactly one player camera in scene!");
//     }
// }

// fn pan_orbit_camera(
//     windows: Res<Windows>,
//     input_mouse: Res<Input<MouseButton>>,
//     mut q: Query<(&mut PlayerCamera, &mut Transform, &PerspectiveProjection)>,
//     mut event_mouse_motion: EventReader<MouseMotion>,
//     mut event_mouse_scroll: EventReader<MouseWheel>,
// ) {
//     let mut scroll = 0.0;

//     for event in event_mouse_scroll.iter() {
//         scroll += event.y;
//     }

//     if (scroll.abs() > 0.0) {

//     }
//     println!("Mouse: {:?}", scroll);
// }

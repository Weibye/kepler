use bevy::{math::{Mat3, Mat4, Quat, Vec3}, prelude::*};
use bevy_mod_picking::*;

use crate::{GameState, orbit_plugin::{OrbitalBody, Sun}};

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(HighlightablePickingPlugin)

            // Resources
            .insert_resource(PlayerCameraSettings {
                offset: Vec3::ONE * 2.0 
            })

            .add_system_set(
                SystemSet::on_enter(GameState::Loading)
                    
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PostLoad)
                    .with_system(make_oribtal_bodies_pickable.system().before("end").after("start"))
                    .with_system(camera_setup.system().before("end").after("start"))
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(print_events.system().after("end"))
                    .with_system(set_camera_target.system().after("end"))
                    .with_system(move_camera_to_target.system().after("end"))
            )
        ;
    }
}

#[derive(Debug, Copy, Clone)]
struct PlayerCamera {
    target: Entity
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
            .spawn_bundle(PerspectiveCameraBundle {
                transform: Transform {
                    translation: settings.offset,
                    rotation: quat_look_at(settings.offset - target_transform.translation, Vec3::Y),
                    ..Default::default()
                },
                ..Default::default()
            })
            // Mark for picking with this camera
            .insert_bundle(PickingCameraBundle::default())
            .insert(BoundVol::default())
            .insert(PlayerCamera {
                target
            })
        ;
    } else {
        panic!("Found no suitable targets");
    }
}

fn make_oribtal_bodies_pickable(
    mut commands: Commands,
    q: Query<Entity, With<OrbitalBody>>,
) {
    info!("Query world");
    for entity in q.iter() {
        commands.entity(entity).insert_bundle(PickableBundle::default());
        warn!("Adding pickable");
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

pub fn print_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        // match event {
        //     PickingEvent::Selection(selection) => {
        //         match selection {
        //             SelectionEvent::JustSelected(entity) => todo!(),
        //             SelectionEvent::JustDeselected(_) => todo!(),
        //         }
        //     },
        //     PickingEvent::Hover(_) => continue,
        // }
        info!("This event happened! {:?}", event);
    }
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
                        _ => continue,
                    }
                },
                _ => continue,
            }
        }
    } else {
        error!("Found no player cameras in scene");
    }
    
}

// Replace the target with a reference to the entity in question, so that we can continously update the camera position
fn move_camera_to_target(
    mut q_camera: Query<(&mut Transform, &PlayerCamera)>,
    q_targets: Query<(Entity, &GlobalTransform), With<OrbitalBody>>,
    settings: Res<PlayerCameraSettings>,
) {
    if let Ok((mut transform, player_cam)) = q_camera.single_mut() {
        if let Ok((_, target_transform)) = q_targets.get(player_cam.target) {
            transform.translation = target_transform.translation + settings.offset;
        } else {
            panic!("Found no suitable targets");
        }
    } else {
        error!("None or more than one player camera in scene!");
    }
}

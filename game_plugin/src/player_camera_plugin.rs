use bevy::{math::{Mat3, Mat4, Quat, Vec3}, prelude::*, render::camera::Camera};
use bevy_mod_picking::*;

use crate::{GameState, orbit_plugin::OrbitalBody};

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(HighlightablePickingPlugin)

            .insert_resource(CameraTarget(Vec3::ZERO))

            .add_system_set(
                SystemSet::on_enter(GameState::Loading)
                    .with_system(camera_setup.system().before("end").after("start"))
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PostLoad)
                    .with_system(make_oribtal_bodies_pickable.system().before("end").after("start"))
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

#[derive(Debug, Copy, Clone, Default)]
struct CameraTarget(Vec3);

// Spawn camera
fn camera_setup(mut commands: Commands,) {
    let camera_position = Vec3::ONE * 2.0;
    let camera_target = Vec3::ZERO;

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform {
                translation: camera_position,
                rotation: quat_look_at(camera_position - camera_target, Vec3::Y),
                ..Default::default()
            },
            ..Default::default()
        })
        // Mark for picking with this camera
        .insert_bundle(PickingCameraBundle::default())
        .insert(BoundVol::default())
    ;

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
    q_targets: Query<(Entity, &GlobalTransform), With<OrbitalBody>>,
    mut camera_target_res: ResMut<CameraTarget>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(selection) => {
                match selection {
                    SelectionEvent::JustSelected(entity) => {
                        match q_targets.get(*entity) {
                            Ok((_, selected_transformation)) => {
                                camera_target_res.0 = selected_transformation.translation;
                            }
                            Err(_) => todo!(),
                        }
                    }
                    _ => continue,
                }
            },
            _ => continue,
        }
    }
}

// Replace the target with a reference to the entity in question, so that we can continously update the camera position
fn move_camera_to_target(mut q: Query<&mut Transform, With<Camera>>, target: Res<CameraTarget>) {
    // if target.is_changed() {
    for mut transform in q.iter_mut() {
        let new_pos =  target.0 + Vec3::ONE * 2.0;
        transform.translation = new_pos;
        transform.rotation = quat_look_at(new_pos - target.0, Vec3::Y);
    }
        // q.get_single()
        // info!("Target was changed");
    // }
}

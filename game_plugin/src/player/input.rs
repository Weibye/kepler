// Source: https://github.com/NiklasEi/bevy_game_template/blob/main/game_plugin/src/actions.rs
// Source: https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

use crate::GameState;
use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<Actions>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                set_movement_actions.system()
            )
            // .add_system_set(
            //     SystemSet::on_update(GameState::Playing)
            //         .with_system(set_movement_actions.system())
            // )
        ;
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub scroll: Option<f32>,
}

fn set_movement_actions(
    mut actions: ResMut<Actions>, 
    keyboard_input: Res<Input<KeyCode>>,
    // mut event_mouse_motion: EventReader<MouseMotion>,
    mouse_wheel_reader: EventReader<MouseWheel>,
) {
    if keyboard_input.just_released(KeyCode::W)
        || keyboard_input.pressed(KeyCode::W)
        || keyboard_input.just_released(KeyCode::A)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.just_released(KeyCode::S)
        || keyboard_input.pressed(KeyCode::S)
        || keyboard_input.just_released(KeyCode::D)
        || keyboard_input.pressed(KeyCode::D)
    {
        let mut player_movement = Vec2::ZERO;

        if keyboard_input.just_released(KeyCode::W) || keyboard_input.just_released(KeyCode::S) {
            if keyboard_input.pressed(KeyCode::W) {
                player_movement.y = 1.;
            } else if keyboard_input.pressed(KeyCode::S) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if keyboard_input.just_pressed(KeyCode::W) {
            player_movement.y = 1.;
        } else if keyboard_input.just_pressed(KeyCode::S) {
            player_movement.y = -1.;
        } else {
            player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }

        if keyboard_input.just_released(KeyCode::D) || keyboard_input.just_released(KeyCode::A) {
            if keyboard_input.pressed(KeyCode::D) {
                player_movement.x = 1.;
            } else if keyboard_input.pressed(KeyCode::A) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if keyboard_input.just_pressed(KeyCode::D) {
            player_movement.x = 1.;
        } else if keyboard_input.just_pressed(KeyCode::A) {
            player_movement.x = -1.;
        } else {
            player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        }

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            actions.player_movement = Some(player_movement);
        }
    } else {
        actions.player_movement = None;
    }
    
    // Handle mouse scroll input
    actions.scroll = mouse_scroll(mouse_wheel_reader);    
}


/// Collects all mouse-wheel events from this frame and translates into a 1D axis
fn mouse_scroll(mut mouse_wheel_reader: EventReader<MouseWheel>) -> Option<f32> {
    let mut scroll = 0.0;
    for event in mouse_wheel_reader.iter() {
        scroll += event.y;
    }

    if scroll.abs() > 0.0 {
        return Some(scroll);
    } else {
        return None;
    }
}
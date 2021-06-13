
// use std::{f32::consts::PI, ops::Rem};

// // use crate::GameState;
// use bevy::{app::AppBuilder, pbr::PbrBundle, prelude::*};
// use bevy_mod_picking::{BoundVol, PickableBundle};
// use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
// use rand::Rng;

// use crate::GameState;



// //             .add_system_set(
// //                 SystemSet::on_update(GameState::Playing)
// //                     .with_system(move_planets.system())
// //                     .with_system(rotate_orbital_bodies.system())
// //                     .with_system(orbit_visuals.system())
// //             )
// //         ;
// //     }
// // }


// #[derive(Debug, Copy, Clone)]
// pub struct OrbitalBody {
//     mass: f32,
//     radius: f32,
//     angular_velocity: f32,
// }
// pub struct ReferenceFrame;
// pub struct Sun;



//     //     commands
//     //         .spawn_bundle(PbrBundle {
//     //             mesh: meshes.add(Mesh::from(shape::Icosphere { radius: dot_radius / 2., subdivisions: 1 })),
//     //             material: materials.add(Color::rgb(1.0, 0.2, 0.2).into()),
//     //             transform: Transform::from_translation(reference_position),
//     //             ..Default::default()
//     //         })
//     //         .insert(orbit)
//     //         .with_children(|parent| {
//     //             // Ascending node
//     //             parent.spawn_bundle(PbrBundle {
//     //                 mesh: meshes.add(Mesh::from(shape::Icosphere { radius: dot_radius, subdivisions: 1 })),
//     //                 material: materials.add(Color::rgb(1.0, 0.0,1.0).into()),
//     //                 transform: (Transform {
//     //                     translation: orbit.ascending_node(),
//     //                     rotation: Quat::IDENTITY,
//     //                     scale: Vec3::ONE
//     //                 }),
//     //                 ..Default::default()
//     //             });

//     //             // Descending node
//     //             parent.spawn_bundle(PbrBundle {
//     //                 mesh: meshes.add(Mesh::from(shape::Icosphere { radius: dot_radius, subdivisions: 1 })),
//     //                 material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
//     //                 transform: (Transform {
//     //                     translation: orbit.descending_node(),
//     //                     rotation: Quat::IDENTITY,
//     //                     scale: Vec3::ONE
//     //                 }),
//     //                 ..Default::default()
//     //             });
                
//     //             parent.spawn_bundle(PbrBundle {
//     //                 mesh: meshes.add(Mesh::from(shape::Icosphere { radius: dot_radius, subdivisions: 0 })),
//     //                 material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
//     //                 transform: (Transform {
//     //                     translation: orbit.periapsis_node(),
//     //                     rotation: Quat::IDENTITY,
//     //                     scale: Vec3::ONE
//     //                 }),
//     //                 ..Default::default()
//     //             });

//     //             parent.spawn_bundle(PbrBundle {
//     //                 mesh: meshes.add(Mesh::from(shape::Icosphere { radius: dot_radius, subdivisions: 0 })),
//     //                 material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
//     //                 transform: (Transform {
//     //                     translation: orbit.apoapsis_node(),
//     //                     rotation: Quat::IDENTITY,
//     //                     scale: Vec3::ONE
//     //                 }),
//     //                 ..Default::default()
//     //             });

//     //             // parent.spawn_bundle(PbrBundle {
//     //             //     mesh: meshes.add(Mesh::from(shape::Icosphere { radius: dot_radius, subdivisions: 0 })),
//     //             //     material: materials.add(Color::rgb(0.0, 1.0, 0.5).into()),
//     //             //     transform: (Transform {
//     //             //         translation: orbit_position.body_pos,
//     //             //         rotation: Quat::IDENTITY,
//     //             //         scale: Vec3::ONE
//     //             //     }),
//     //             //     ..Default::default()
//     //             // })
//     //             // .insert(OrbitHelper);
//     //         });
//     // }
// // }

// fn orbit_visuals(
//     // mut commands: Commands,
//     query: Query<&Orbit>,
//     mut lines: ResMut<DebugLines>,

// ) {
//     let steps = 24;
//     let step_angle = 2. * PI / steps as f32;
//     for orbit in query.iter() {
//         // Ascending / Desceding
//         lines.line_colored(Vec3::ZERO, orbit.ascending_node(), 0.0, Color::rgb(0.5, 0.0, 1.0));
//         lines.line_colored(Vec3::ZERO, orbit.descending_node(), 0.0, Color::rgb(0.5, 0.0, 1.0));

//         // Apoapsis / Periapsis
//         lines.line_colored(Vec3::ZERO, orbit.periapsis_node(), 0.0, Color::rgb(0.3, 1.0, 0.0));
//         lines.line_colored(Vec3::ZERO, orbit.apoapsis_node(), 0.0, Color::rgb(0.3, 1.0, 0.0));

//         lines.line_colored(Vec3::ZERO, orbital_position_at_true_anomaly(*orbit, orbit.true_anomaly), 0.0, Color::rgb(1., 1., 1.));

//         let line_positions = orbit.orbit_ring();

//         for n in 0..line_positions.len() {
//             let current = line_positions[n];
//             let next;
//             if n == line_positions.len() - 1 {
//                 next = line_positions[0];
//             } else {
//                 next = line_positions[n+1];
//             }
            
//             lines.line(current, next, 0.0);
//         }
//     }
// }

// // Get transforms that are children of enenties with Orbit
// fn move_planets(
//     mut q_child: Query<(&mut Transform, &Parent), With<OrbitalBody>>,
//     q_parent: Query<(&Orbit, &GlobalTransform)>,
//     time: Res<Time>,
// ){
//     let speed = PI/30.0;
//     let mut total_offset = speed * time.seconds_since_startup() as f32;
//     if total_offset >= 2.*PI {
//         total_offset = total_offset.rem(2.*PI);
//     }

//     for (mut transform, parent) in q_child.iter_mut() {
//         let parent_result = q_parent.get(parent.0);
//         match parent_result {
//             Ok((orbit, global_transform)) => {
//                 let pos = orbital_position_at_true_anomaly(*orbit, orbit.true_anomaly + total_offset);

//                 transform.translation = pos;
//             },
//             Err(_) => continue
//         }
//     }
// }

// /// Rotates the orbital bodies along their own axis of rotation
// fn rotate_orbital_bodies(
//     mut query: Query<(&mut Transform, &OrbitalBody)>, 
//     time: Res<Time>,
// ) {
//     for (mut transform, body) in query.iter_mut() {
//         let up = transform.local_y();
//         transform.rotation *= Quat::from_axis_angle(up, body.angular_velocity * time.delta_seconds());
//     }
// }


// fn reference_plane(
//     mut commands: Commands,
//     // mut meshes: ResMut<Assets<Mesh>>,
//     // mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // commands.spawn_bundle(PbrBundle {
//     //     mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
//     //     material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
//     //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
//     //     ..Default::default()
//     // });
//     // light
//     commands.spawn_bundle(LightBundle {
//         transform: Transform::from_xyz(0.0, 8.0, 0.0),
//         ..Default::default()
//     });
// }


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


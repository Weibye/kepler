use std::f32::consts::PI;

use bevy::{math::Vec3, pbr::PbrBundle, prelude::{Assets, BuildChildren, Commands, GlobalTransform, Mesh, ResMut, StandardMaterial, Transform, shape}};
use bevy_mod_picking::{BoundVol, PickableBundle};
use rand::Rng;

use crate::orbit::{OrbitalBody, ReferenceFrame, Sun, orbit::{Orbit, orbital_position_at_true_anomaly}};


pub fn spawn_orbits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    println!("Creating orbits");
    let mut rng = rand::thread_rng();
    let reference_forward = -Vec3::Z;
    let reference_up = Vec3::Y;
    let reference_position = Vec3::new(0.0, 0.0, 0.0);
    let dot_radius = 0.07;
    // let sun_radius = 0.15;

    let (sun_transform, sun_body) = spawn_sun();

    // Orbit parameters


    let min_orbit_radius = sun_body.radius + 0.1;
    let max_orbit_radius = 2.5;

    commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(ReferenceFrame)
        .with_children(|reference_frame | {
            reference_frame
                .spawn()
                .insert(sun_body)
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere { radius: sun_body.radius, subdivisions: 1 })), 
                    ..Default::default()
                })
                .insert(Sun)
                .insert_bundle(PickableBundle::default())
                .insert(BoundVol::default())
            ;

            for _ in 0..3 {
                // Planets
                let semi_major_axis = rng.gen_range(min_orbit_radius..max_orbit_radius);
                let planet_orbit = Orbit {
                    semi_major_axis,
                    eccentricity: rng.gen_range(0.0..0.5) * semi_major_axis,
                    longitude_of_ascending_node: rng.gen_range(0.0..(2.0*PI)),
                    inclination: rng.gen_range(0.0..0.2),
                    argument_of_periapsis: rng.gen_range(0.0..(2.0*PI)),
                    true_anomaly: rng.gen_range(0.0..(2.0*PI)),
                    ref_forward: -Vec3::Z,
                    ref_up: Vec3::Y,
                    ref_pos: Vec3::ZERO,
                };

                reference_frame
                    .spawn()
                    .insert(planet_orbit)
                    .insert(Transform::default())
                    .insert(GlobalTransform::default())

                    .with_children(|planet_reference_frame| {
                        let planet_radius = rng.gen_range(0.05..0.10);
                        let planet_transform = Transform {
                            translation: orbital_position_at_true_anomaly(planet_orbit, planet_orbit.true_anomaly),
                            ..Default::default()
                        };
                        let planet_body = OrbitalBody {
                            radius: planet_radius,
                            mass: rng.gen_range(1.0..5.0),
                            angular_velocity: rng.gen_range(0.01..0.25),
                        };

                        planet_reference_frame
                            .spawn()
                            .insert(planet_body)
                            .insert_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Icosphere { radius: planet_body.radius, subdivisions: 1 })),
                                transform: planet_transform,
                                ..Default::default()
                            })
                            .insert_bundle(PickableBundle::default())
                            .insert(BoundVol::default())
                        ;
                    })
                ;
            }
        })
    ;
}

fn spawn_sun() -> (Transform, OrbitalBody) {

    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(0.10..0.20);
    let transform = Transform::default();
    let body = OrbitalBody {
        radius,
        mass: rng.gen_range(5.0..10.0),
        angular_velocity: rng.gen_range(0.15..0.25),
    };

    return (transform, body);
}

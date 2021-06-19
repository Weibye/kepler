use bevy::prelude::{Assets, BuildChildren, ChildBuilder, Commands, GlobalTransform, Mesh, Res, ResMut, StandardMaterial, Transform};
use kepler::{EllipticalOrbit, OrbitalBody};

use crate::orbit::bundles::{
    OrbitalBodyBundle,
    ReferenceFrameBundle,
};

use super::{HierarchyNode, WorldGenerationSettings, generate_world::generate_world};


pub(super) fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<WorldGenerationSettings>
) {
    let sun_frame_transform = Transform::default();
    let sun_frame = commands
        .spawn()
        .insert_bundle(ReferenceFrameBundle::from_transform(sun_frame_transform))
        .id();

    let orbital_body_sun = OrbitalBody::from_sphere(0.5, 1.0, 0.1);

    let sun_body = commands
        .spawn()
        .insert_bundle(OrbitalBodyBundle::from_orbital_body(orbital_body_sun, &mut meshes))
        .id();

    commands.entity(sun_frame).push_children(&[sun_body]);

    let planet_orbit = EllipticalOrbit::new(
        0.9, 
        4.5, 
        0.0, 
        0.0,
        0.0,
        0.0,
        25.0);

    let position = planet_orbit.get_position_vector(&sun_frame_transform);
    
    let body_planet = OrbitalBody::from_sphere(0.2, 0.1, -0.3);

    let planet_ref = commands
        .spawn()
        .insert_bundle(ReferenceFrameBundle::from_transform(Transform::from_translation(position)))
        .insert(planet_orbit)
        .id();

    let planet_body = commands
        .spawn()
        .insert_bundle(OrbitalBodyBundle::from_orbital_body(body_planet, &mut meshes))
        .id();

    
    commands.entity(planet_ref).push_children(&[planet_body]);
    commands.entity(sun_frame).push_children(&[planet_ref]);


    // let root = commands
    //     .spawn()
    //     .insert_bundle(ReferenceFrameBundle::from_transform(solar_system.reference_frame))
    //     .id()
    // ;

    // let root_body = commands
    //     .spawn_bundle(OrbitalBodyBundle::from_orbital_body(
    //         solar_system.body, &mut meshes
    //     ))
    //     .id()
    // ;

    // commands.entity(root).push_children(&[root_body]);

    // if let Some(planets) = solar_system.children {
    //     let planet_01 = &planets[0];
    //     let planet = commands
    //         .spawn()
    //         .insert_bundle(ReferenceFrameBundle::from_transform(planet_01.node.reference_frame))
    //         .insert(planet_01.node.orbit)
    //         .id()
    //     ;

    //     let planet_body = commands
    //         .spawn()
    //         .insert_bundle(OrbitalBodyBundle::from_orbital_body(planet_01.node.body, &mut meshes))
    //         .id()
    //     ;

    //     commands.entity(planet).push_children(&[planet_body]);
    //     commands.entity(root).push_children(&[planet]);
    // }

    // let planet_01 = 


        // .with_children(| builder | {

        //     builder.spawn_bundle(OrbitalBodyBundle::from_orbital_body(solar_system.body, &mut meshes));

        //     if let Some(nodes) = solar_system.children {
        //         for node in nodes {
        //             spawn_node(node, builder, &mut meshes);
        //         }
        //     }
        // })
    // ;
}


// fn spawn_orbit_node(mut commands: &mut Commands,) {
//     let entity = commands
//         .spawn()
//         .insert_bundle(ReferenceFrameBundle::from_transform(node.node.reference_frame))
//         .insert(node.node.orbit)
//         .id()
//     ;
// }
fn spawn_node(node: HierarchyNode, builder: &mut ChildBuilder, mut meshes: &mut ResMut<Assets<Mesh>>) {
    
    builder
        .spawn()
        .insert_bundle(ReferenceFrameBundle::from_transform(node.node.reference_frame))
        .insert(node.node.orbit)
        .with_children(|parent_builder | {
            parent_builder.spawn_bundle(OrbitalBodyBundle::from_orbital_body(node.node.body, &mut meshes));

            if let Some(child_nodes) = node.children {
                for child_node in child_nodes {
                    spawn_node(child_node, parent_builder, &mut meshes);
                }
            }
        })
    ;
}

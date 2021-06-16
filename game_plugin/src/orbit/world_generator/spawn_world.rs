use bevy::prelude::{Assets, BuildChildren, ChildBuilder, Commands, GlobalTransform, Mesh, Res, ResMut, StandardMaterial, Transform};

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
    let solar_system = generate_world(settings);

    let sun = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .id();

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

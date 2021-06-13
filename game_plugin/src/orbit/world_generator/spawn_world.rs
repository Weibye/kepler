use bevy::{prelude::{Assets, BuildChildren, ChildBuilder, Commands, Mesh, Res, ResMut, StandardMaterial}};

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

    commands
        .spawn_bundle(ReferenceFrameBundle::from_transform(solar_system.reference_frame))
        .with_children(| builder | {

            builder.spawn_bundle(OrbitalBodyBundle::from_orbital_body(solar_system.body, &mut meshes));

            if let Some(nodes) = solar_system.children {
                for node in nodes {
                    spawn_node(node, builder, &mut meshes);
                }
            }
        })
    ;
}

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

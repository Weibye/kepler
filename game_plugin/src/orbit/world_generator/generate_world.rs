use bevy::{math::Vec3, prelude::{Res, Transform}};

use crate::{orbit::{components::OrbitalBody, orbit_parameters::{OrbitParameters, orbital_position_at_true_anomaly}}, utils::quat_from_axes};

use super::{HierarchyNode, OrbitNode, RootNode, WorldGenerationSettings};

/// Generates the orbits to be used
pub(super) fn generate_world(settings: Res<WorldGenerationSettings>) -> RootNode {

    let root_reference = Transform::default();

    let solar_system_node = RootNode {
        reference_frame: root_reference,
        body: OrbitalBody { mass: 1.0, radius: 0.5, angular_velocity: 1.0 },
        children: {
            let mut nodes: Vec<HierarchyNode> = Vec::new();

            // All R-O pairs must have 1 B child
            // R-O pair can have multiple R-O child
            // Planets
            for _ in 0..3 {
                let planet = generate_node(&root_reference);

                // 
                let moon_01 = generate_node(&planet.reference_frame);
                let moon_02 = generate_node(&planet.reference_frame);
                let mut planet_children: Vec<HierarchyNode> = Vec::new();
                planet_children.push(HierarchyNode { node: moon_01,  children: None });
                planet_children.push(HierarchyNode { node: moon_02,  children: None });

                nodes.push(HierarchyNode { node: planet,  children: Some(planet_children) });
            }

            Some(nodes)
        },
    };

    return solar_system_node;
}

// fn generate_sun() -> (Transform, OrbitalBody) {
//     (Transform::default(), OrbitalBody { mass: 1.0, radius: 0.5, angular_velocity: 1.0 })
// }

fn transform_from_orbit(orbit: OrbitParameters, reference_frame: &Transform) -> Transform {
    let forward = orbit.ascending_dir(reference_frame);
    let up = orbit.orbital_normal(reference_frame);
    let right = up.cross(forward);

    Transform {
        translation: orbital_position_at_true_anomaly(orbit, orbit.true_anomaly, &reference_frame),
        rotation: quat_from_axes(right, up, forward),
        scale: Vec3::ONE,
    }
}

fn generate_node(parent_reference: &Transform) -> OrbitNode {

    let orbit = OrbitParameters::from_rand(); 

    OrbitNode {
        orbit,
        reference_frame: transform_from_orbit(orbit, parent_reference),
        body: OrbitalBody { mass: 1.0, radius: 0.1, angular_velocity: 0.2 }
    }
}


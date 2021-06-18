use std::f32::consts::PI;

use bevy::{math::Vec3, prelude::{Res, Transform}};
use kepler::OrbitalBody;
use rand::Rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

use crate::{orbit::orbit_parameters::{OrbitParameters, orbital_position_at_true_anomaly}, utils::quat_from_axes};

use super::{HierarchyNode, OrbitNode, RootNode, WorldGenerationSettings};

/// Generates the orbits to be used
pub(super) fn generate_world(settings: Res<WorldGenerationSettings>) -> RootNode {

    let root_reference = Transform::default();
    let mut rng: Pcg64 = Seeder::from("KEPLER").make_rng();

    let solar_system_node = RootNode {
        reference_frame: root_reference,
        body: OrbitalBody::from_sphere(0.5, 1.0, 1.0),
        children: {
            let mut nodes: Vec<HierarchyNode> = Vec::new();

            // All R-O pairs must have 1 B child
            // R-O pair can have multiple R-O child
            // Planets
            for _ in 0..3 {
                let planet = generate_node(&root_reference, &mut rng);

                // 
                let moon_01 = generate_node(&planet.reference_frame, &mut rng);
                let moon_02 = generate_node(&planet.reference_frame, &mut rng);
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

fn generate_node(parent_reference: &Transform, rng: &mut Pcg64) -> OrbitNode {

    // let mut rng = rand::thread_rng();
    
    let orbit = OrbitParameters {
        eccentricity: rng.gen_range(0.0..0.1),
        semi_major_axis: rng.gen_range(1.0..=2.5),
        longitude_of_ascending_node: rng.gen_range(0.0..PI*2.),
        inclination: rng.gen_range(0.0..0.10),// PI*2.),
        argument_of_periapsis: rng.gen_range(0.0..0.10),// PI*2.),
        true_anomaly: rng.gen_range(0.0..PI*2.),
    }; 

    OrbitNode {
        orbit,
        reference_frame: transform_from_orbit(orbit, parent_reference),
        body: OrbitalBody::from_sphere(0.1, 1.0, 0.2),
    }
}


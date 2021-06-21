use bevy_asset::Assets;
use bevy_ecs::prelude::{Bundle, ResMut};
use bevy_mod_picking::{BoundVol, PickableBundle};
use bevy_pbr::PbrBundle;
use bevy_render::mesh::{Mesh, shape};
use bevy_transform::components::Transform;

use crate::OrbitalBody;

#[derive(Bundle)]
pub struct OrbitalBodyBundle {
    body: OrbitalBody,
    #[bundle]
    geometry: PbrBundle,
    // #[bundle]
    // transform: TransformBundle,
    #[bundle]
    pickable: PickableBundle,
    bound_vol: BoundVol,
}

impl OrbitalBodyBundle {
    pub fn new(radius: f32, density: f32, spin_velocity: f32, transform: Transform, mesh_handle: &mut ResMut<Assets<Mesh>>) -> Self {
        OrbitalBodyBundle {
            body: OrbitalBody::from_sphere(radius, density, spin_velocity),
            geometry: PbrBundle {
                mesh: mesh_handle.add(Mesh::from(
                    shape::Icosphere { 
                        radius,
                        subdivisions: 1 
                    }
                )),
                transform,
                ..Default::default()
            },
            // transform: TransformBundle::from_transform(transform),
            pickable: PickableBundle::default(),
            bound_vol: BoundVol::default(),
        }
    }
}
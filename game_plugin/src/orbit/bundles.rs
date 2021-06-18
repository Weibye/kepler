use bevy::{pbr::PbrBundle, prelude::*};
use bevy_mod_picking::{BoundVol, PickableBundle};
use kepler::OrbitalBody;

use super::{components::ReferenceFrame, orbit_parameters::{OrbitParameters, orbital_position_at_true_anomaly}};

#[derive(Bundle)]
pub struct OrbitalBodyBundle {
    body: OrbitalBody,
    #[bundle]
    pbr: PbrBundle,
    #[bundle]
    pickable: PickableBundle,
    bound_vol: BoundVol,
}

impl OrbitalBodyBundle {
    pub fn from_radius(radius: f32, mesh_handle: &mut ResMut<Assets<Mesh>>) -> Self {
        OrbitalBodyBundle {
            body: OrbitalBody::from_sphere(radius, 1.0, 1.0),
            pbr: PbrBundle {
                mesh: mesh_handle.add(Mesh::from(
                        shape::Icosphere { 
                            radius, 
                            subdivisions: 1 
                        }
                    )), 
                ..Default::default()
            },
            pickable: PickableBundle::default(),
            bound_vol: BoundVol::default(),
        }
    }
    pub fn from_orbital_body(body: OrbitalBody, mesh_handle: &mut ResMut<Assets<Mesh>>) -> Self {
        OrbitalBodyBundle {
            body,
            pbr: PbrBundle {
                mesh: mesh_handle.add(Mesh::from(
                        shape::Icosphere { 
                            radius: body.radius, 
                            subdivisions: 1 
                        }
                    )), 
                ..Default::default()
            },
            pickable: PickableBundle::default(),
            bound_vol: BoundVol::default(),
        }
    }
}

#[derive(Bundle)]
pub struct ReferenceFrameBundle {
    marker: ReferenceFrame,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl ReferenceFrameBundle {
    pub fn from_translation(translation: Vec3) -> Self {
        ReferenceFrameBundle {
            marker: ReferenceFrame,
            transform: Transform::from_translation(translation),
            global_transform: GlobalTransform::default(),
        }
    }

    pub fn from_transform(transform: Transform) -> Self {
        ReferenceFrameBundle {
            marker: ReferenceFrame,
            transform,
            global_transform: GlobalTransform::default(),
        }
    }

    pub fn from_orbit_params(params: OrbitParameters, reference_frame: &Transform) -> Self {

        let translation = orbital_position_at_true_anomaly(params, params.true_anomaly, reference_frame);
        
        ReferenceFrameBundle {
            marker: ReferenceFrame,
            transform: Transform {
                translation,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            global_transform: GlobalTransform::default(),
        }
    }
}
impl Default for ReferenceFrameBundle {
    fn default() -> Self {
        ReferenceFrameBundle { 
            marker: ReferenceFrame,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}
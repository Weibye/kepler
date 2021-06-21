use bevy::{prelude::*, pbr::PbrBundle};
use bevy_mod_picking::{BoundVol, PickableBundle};
use kepler::{Ellipse, OrbitalBody, OrbitalPlane, TransformBundle};

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
    #[bundle]
    transform: TransformBundle
}

impl ReferenceFrameBundle {
    pub fn from_translation(translation: Vec3) -> Self {
        ReferenceFrameBundle {
            marker: ReferenceFrame,
            transform: TransformBundle::from_translation(translation),
        }
    }

    pub fn from_transform(transform: Transform) -> Self {
        ReferenceFrameBundle {
            marker: ReferenceFrame,
            transform: TransformBundle::from_transform(transform),
        }
    }
    pub fn from_transforms(local: Transform, global: GlobalTransform) -> Self {
        ReferenceFrameBundle {
            marker: ReferenceFrame,
            transform: TransformBundle::from_transforms(local, global),
        }
    }

    pub fn from_orbit_params(params: OrbitParameters, reference_frame: &Transform) -> Self {

        let translation = orbital_position_at_true_anomaly(params, params.true_anomaly, reference_frame);
        
        ReferenceFrameBundle {
            marker: ReferenceFrame,
            transform: TransformBundle::from_translation(translation),
        }
    }
}
impl Default for ReferenceFrameBundle {
    fn default() -> Self {
        ReferenceFrameBundle { 
            marker: ReferenceFrame,
            transform: TransformBundle::default(),
        }
    }
}
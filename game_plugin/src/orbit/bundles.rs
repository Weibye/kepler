use bevy::prelude::*;
use kepler::TransformBundle;

use super::{components::ReferenceFrame, orbit_parameters::{OrbitParameters, orbital_position_at_true_anomaly}};

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
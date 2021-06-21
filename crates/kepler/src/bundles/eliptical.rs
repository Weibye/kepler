use bevy_ecs::prelude::Bundle;
use bevy_math::{Quat, Vec3};
use bevy_transform::components::{GlobalTransform, Transform};

use crate::{Ellipse, OrbitalDirectionsLocal, OrbitalPlane, TransformBundle, transform_from_axis};

#[derive(Bundle)]
pub struct EllipticalOrbitBundle {
    ellipse: Ellipse,
    plane: OrbitalPlane,
    #[bundle]
    transform: TransformBundle,
}

impl EllipticalOrbitBundle {
    pub fn new(
        semi_major: f32, eccentricity: f32, 
        ascending_angle: f32, inclination_angle: f32, periapsis_angle: f32,
    ) -> Self {
        let plane = OrbitalPlane::new(ascending_angle, inclination_angle, periapsis_angle);

        let transform = Transform {
            translation: Vec3::ZERO,
            rotation: plane.get_rot(),
            scale: Vec3::ONE,
        };

        EllipticalOrbitBundle {
            ellipse: Ellipse::from_major(semi_major, eccentricity),
            plane,
            transform: TransformBundle::from_transform(transform),
        }
    }
}
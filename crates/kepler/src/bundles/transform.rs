use bevy_ecs::prelude::Bundle;
use bevy_math::Vec3;
use bevy_transform::components::{GlobalTransform, Transform};

#[derive(Bundle, Default)]
pub struct TransformBundle {
    transform: Transform,
    global_transform: GlobalTransform,
}

impl TransformBundle {
    pub fn from_xyz(x:f32, y:f32, z:f32) -> Self {
        TransformBundle::from_translation(Vec3::new(x, y, z))
    }

    pub fn from_translation(translation: Vec3) -> Self {
        TransformBundle {
            transform: Transform::from_translation(translation),
            global_transform: GlobalTransform::default(),
        }
    }

    pub fn from_transform(transform: Transform) -> Self {
        TransformBundle {
            transform,
            global_transform: GlobalTransform::default(),
        }
    }
    pub fn from_transforms(local: Transform, global: GlobalTransform) -> Self {
        TransformBundle {
            transform: local,
            global_transform: global,
        }
    }
}

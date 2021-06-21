use bevy_math::{Quat, Vec3};
use bevy_transform::components::{GlobalTransform, Transform};

use crate::quat_from_axes;

/// The plane the orbit takes place in.
///
/// Defined as the rotation of the forward axis and tilt 
/// of the right axis compared the the parent's frame of reference
pub struct OrbitalPlane {
    /// CCW rotation in radians between the parent
    /// forward and the orbitals plane's forward
    longitude_of_ascending_node: f32,
    /// Inclination
    ///
    /// CCW rotation around the local forward, 
    /// to determine the axial tilt of the orbital plane
    /// Notation: `i`
    inclination: f32,
    /// The orientation of the ellipse in the obital plane
    argument_of_periapsis: f32, 
}

impl OrbitalPlane {
    pub fn new(ascending_angle: f32, inclination_angle: f32, periapsis_angle: f32) -> Self {
        OrbitalPlane {
            longitude_of_ascending_node: ascending_angle,
            inclination: inclination_angle,
            argument_of_periapsis: periapsis_angle
        }
    }

    // Getters
    pub fn ascending_arg(&self) -> f32 { self.longitude_of_ascending_node }
    pub fn inclination_arg(&self) -> f32 { self.inclination }
    pub fn periapsis_arg(&self) -> f32 { self.argument_of_periapsis }

    // Setters
    pub fn set_periapsis_arg(&mut self, new_value: f32) { self.argument_of_periapsis = new_value; }
    pub fn set_ascending_arg(&mut self, new_value: f32) { self.longitude_of_ascending_node = new_value; }
    pub fn set_inclination_arg(&mut self, new_value: f32) { self.inclination = new_value; }

    pub fn get_rot(&self) -> Quat {
        let offset_ascending = Quat::from_axis_angle(Vec3::Y, self.ascending_arg());
        let offset_incl = Quat::from_axis_angle(Vec3::X, self.inclination_arg());

        let result = offset_ascending * offset_incl;

        result
    }

    pub fn get_rotation_global(&self, reference: &GlobalTransform) -> Quat {
        let up = self.zenith_global(reference);
        let forward = self.ascending_global(reference);
        let right = up.cross(forward);

        quat_from_axes(right, up, forward)
    }
}


pub trait OrbitalDirectionsLocal {
    /// Zenith of the orbital plane
    /// Also known as Normal of the orbital plane
    fn zenith_local(&self, reference: &Transform) -> Vec3;
    fn nadir_local(&self, reference: &Transform) -> Vec3;
    fn ascending_local(&self, reference: &Transform) -> Vec3;
    fn descending_local(&self, reference: &Transform) -> Vec3;
    fn periapsis_local(&self, reference: &Transform) -> Vec3;
    fn apoapsis_local(&self, reference: &Transform) -> Vec3;
}

impl OrbitalDirectionsLocal for OrbitalPlane {
    fn zenith_local(&self, reference: &Transform) -> Vec3 {
        Quat::from_axis_angle(self.ascending_local(reference), self.inclination) * reference.local_y()
    }

    fn nadir_local(&self, reference: &Transform) -> Vec3 {
        -self.zenith_local(reference)
    }

    fn ascending_local(&self, reference: &Transform) -> Vec3 {
        Quat::from_axis_angle(reference.local_y(), self.longitude_of_ascending_node) * reference.local_z()
    }

    fn descending_local(&self, reference: &Transform) -> Vec3 {
        -self.ascending_local(reference)
    }

    fn periapsis_local(&self, reference: &Transform) -> Vec3 {
        Quat::from_axis_angle(self.zenith_local(reference), self.argument_of_periapsis) * self.ascending_local(reference)
    }

    fn apoapsis_local(&self, reference: &Transform) -> Vec3 {
        -self.periapsis_local(reference)
    }
}

pub trait OrbitalDirectionsGlobal {
    /// Zenith of the orbital plane
    /// Also known as Normal of the orbital plane
    fn zenith_global(&self, reference: &GlobalTransform) -> Vec3;
    fn nadir_global(&self, reference: &GlobalTransform) -> Vec3;
    fn ascending_global(&self, reference: &GlobalTransform) -> Vec3;
    fn descending_global(&self, reference: &GlobalTransform) -> Vec3;
    fn periapsis_global(&self, reference: &GlobalTransform) -> Vec3;
    fn apoapsis_global(&self, reference: &GlobalTransform) -> Vec3;
}

impl OrbitalDirectionsGlobal for OrbitalPlane {
    fn zenith_global(&self, reference: &GlobalTransform) -> Vec3 {
        Quat::from_axis_angle(self.ascending_global(reference), self.inclination) * reference.local_y()
    }

    fn nadir_global(&self, reference: &GlobalTransform) -> Vec3 {
        -self.zenith_global(reference)
    }

    fn ascending_global(&self, reference: &GlobalTransform) -> Vec3 {
        Quat::from_axis_angle(reference.local_y(), self.longitude_of_ascending_node) * reference.local_z()
    }

    fn descending_global(&self, reference: &GlobalTransform) -> Vec3 {
        -self.ascending_global(reference)
    }

    fn periapsis_global(&self, reference: &GlobalTransform) -> Vec3 {
        Quat::from_axis_angle(self.zenith_global(reference), self.argument_of_periapsis) * self.ascending_global(reference)
    }

    fn apoapsis_global(&self, reference: &GlobalTransform) -> Vec3 {
        -self.periapsis_global(reference)
    }
}
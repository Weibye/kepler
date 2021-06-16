use crate::*;

/// A spherical body that can undergo orbit
pub struct OrbitalBody {
    mass: Mass,
    radius: Length,
    density: Density,
    volume: Volume,
    spin_velocity: AngularVelocity,
    // axial_tilt: Angle, // TODO: Quaternion?
}

impl OrbitalBody {
    fn new(mass: Mass, radius: Length, density: Density, volume: Volume, spin_velocity: AngularVelocity) -> Self {
        OrbitalBody {
            mass,
            radius,
            density,
            volume,
            spin_velocity,
        }
    }
}

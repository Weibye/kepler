use std::f32::consts::PI;
/// A spherical body that can undergo orbit

#[derive(Debug, Copy, Clone)]
pub struct OrbitalBody {
    pub mass: f32,
    pub radius: f32,
    pub density: f32,
    pub volume: f32,
    pub spin_velocity: f32,
    // axial_tilt: Angle, // TODO: Quaternion?
}

impl OrbitalBody {
    pub fn new(mass: f32, radius: f32, density: f32, volume: f32, spin_velocity: f32) -> Self {
        OrbitalBody {
            mass,
            radius,
            density,
            volume,
            spin_velocity,
        }
    }

    pub fn from_sphere(radius: f32, density: f32, spin_velocity: f32) -> Self {
        let volume = sphere_volume(radius);
        let mass = volume * density;

        OrbitalBody {
            radius,
            density,
            spin_velocity,
            volume,
            mass 
        }
    }
}

pub fn sphere_volume(radius: f32) -> f32 {
    4.0 / 3.0 * PI * radius.powf(3.0)
}
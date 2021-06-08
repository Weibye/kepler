use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};
use rand::{Rng, thread_rng};

#[derive(Debug, Copy, Clone)]
pub struct Orbit {
    /// Defines how elliptical the orbit is.
    /// Range 0 -> e -> 1
    // / When eccentricity = 0, the orbit is a perfect circle
    /// 1: most ellipse before stopping being closed
    pub eccentricity: f32, 
    /// The sum of the Periapsis and Apoapsis distances devided by two.
    /// If eccentricity is 0, this is the radius of the circle.
    pub semi_major_axis: f32,
    // orientation of the orbital plane
    pub longitude_of_ascending_node: f32,
    pub inclination: f32,
    /// Defines the orientation of the ellipse in the orbital plane
    /// Angle between the ascending node 
    pub argument_of_periapsis: f32,
    /// Angle between periapsis position of orbital body at T = 0.
    /// _Range: \[0..=2PI\]_
    /// defines the position of the orbital body along the orbital elipse at t = 0
    pub true_anomaly: f32,
    pub ref_forward: Vec3,
    pub ref_up: Vec3,
    pub ref_pos: Vec3,
}

impl Orbit {
    pub fn rand() -> Orbit {
        let mut rng = thread_rng();

        Orbit {
            eccentricity: rng.gen_range(0.0..=0.5),
            semi_major_axis: rng.gen_range(0.5..=2.5),
            longitude_of_ascending_node: rng.gen_range(0.0..(PI)),
            inclination: rng.gen_range(0.0..(PI)),
            argument_of_periapsis: rng.gen_range(0.0..(PI)),
            true_anomaly: rng.gen_range(0.0..(2.*PI)),
            ref_forward: -Vec3::Z,
            ref_up: Vec3::Y,
            ref_pos: Vec3::ZERO,
        }
    }

    pub fn orbit_ring(&self) -> Vec<Vec3> {
        let mut ring_positions = Vec::new();
        let steps = 36;
        let step_angle = 2. * PI / steps as f32;
        
        for n in 0..steps {
            ring_positions.push(orbital_position_at_true_anomaly(*self, step_angle * n as f32));
        }

        return ring_positions;
    }
}

/// Normalized directions for the orbital definition
pub trait OrbitalDirs {
    fn ascending_dir(&self) -> Vec3;
    fn descending_dir(&self) -> Vec3;
    fn periapsis_dir(&self) -> Vec3;
    fn apoapsis_dir(&self) -> Vec3;
    
    fn orbital_normal(&self) -> Vec3;
}
impl OrbitalDirs for Orbit {
    /// Direction of the normal of the orbital plane
    fn orbital_normal(&self) -> Vec3 {
        Quat::from_axis_angle(self.ascending_dir(), self.inclination) * self.ref_up
    }

    /// Direction of the ascending node
    fn ascending_dir(&self) -> Vec3 {
        Quat::from_axis_angle(self.ref_up, self.longitude_of_ascending_node) * self.ref_forward
    }

    /// Direction of descending node
    fn descending_dir(&self) -> Vec3 {
        -self.ascending_dir()
    }

    /// Direction of periapsis
    fn periapsis_dir(&self) -> Vec3 {
        Quat::from_axis_angle(self.orbital_normal(), self.argument_of_periapsis) * self.ascending_dir()
    }

    /// Direction of apoapsis
    fn apoapsis_dir(&self) -> Vec3 {
        -self.periapsis_dir()
    }
}

/// Positions of the orbital nodes
pub trait OrbitalPositions {
    fn ascending_node(&self) -> Vec3;
    fn descending_node(&self) -> Vec3;
    fn periapsis_node(&self) -> Vec3;
    fn apoapsis_node(&self) -> Vec3;
}
impl OrbitalPositions for Orbit {
    fn ascending_node(&self) -> Vec3 {
        self.ref_pos 
        + self.ascending_dir() 
        * radius_at_true_anomaly(
            self.semi_major_axis, 
            self.eccentricity, 
            self.longitude_of_ascending_node + self.true_anomaly
        )
    }

    fn descending_node(&self) -> Vec3 {
        self.ref_pos
        + self.descending_dir()
        * radius_at_true_anomaly(
            self.semi_major_axis, 
            self.eccentricity, 
            self.longitude_of_ascending_node + PI + self.true_anomaly
        )
    }

    fn periapsis_node(&self) -> Vec3 {
        let radius = self.semi_major_axis * (1. - self.eccentricity);

        self.ref_pos
        + self.periapsis_dir()
        * radius
    }

    fn apoapsis_node(&self) -> Vec3 {
        let radius = self.semi_major_axis * (1. + self.eccentricity);

        self.ref_pos
        + self.apoapsis_dir()
        * radius
    }
}

pub struct OrbitDetails {
    pub periapsis: Vec3,
    pub apoapsis: Vec3,
    pub ascending_node: Vec3,
    pub descending_node: Vec3,
    pub body_pos: Vec3,
}

struct ReferenceFrame {
    forward: Vec3,
    up: Vec3,
    position: Vec3,
}

pub fn orbit_positions(orbit: Orbit, reference_forward: Vec3, reference_up: Vec3, reference_position: Vec3) -> OrbitDetails {

    // Radii
    let radius_periapsis = orbit.semi_major_axis * (1. - orbit.eccentricity);
    let radius_apoapsis= orbit.semi_major_axis * (1. + orbit.eccentricity);

    let ascending_node_rot = Quat::from_axis_angle(reference_up, orbit.longitude_of_ascending_node);
    let ascending_node_direction = ascending_node_rot * reference_forward; // this should be reprojected down to the actual orbit position
    // let ascending_node_position = ascending_node_direction + reference_position;
    // Inclination
    let inclination_rot = Quat::from_axis_angle(ascending_node_direction, orbit.inclination);
    let orbit_normal = inclination_rot * reference_up;

    let periapsis_dir = Quat::from_axis_angle(orbit_normal, orbit.argument_of_periapsis) * ascending_node_direction;

    let periapsis_position = periapsis_dir * radius_periapsis + reference_position;
    let apoapsis_position = -periapsis_dir * radius_apoapsis + reference_position;

    let eccentricity_vector = (periapsis_position - apoapsis_position).normalize() * orbit.eccentricity;

    let true_anomaly_direction = Quat::from_axis_angle(orbit_normal, orbit.true_anomaly) * periapsis_dir; // todo: replaced by eccentricity vector? it has the same direction

    let true_anom_radius = radius_at_true_anomaly(orbit.semi_major_axis, orbit.eccentricity, orbit.true_anomaly);

    OrbitDetails {
        ascending_node: reference_position + ascending_node_direction,
        descending_node: reference_position - ascending_node_direction,
        periapsis: periapsis_position,
        apoapsis: apoapsis_position,
        body_pos: true_anomaly_direction * true_anom_radius,
    }
}

fn radius_at_true_anomaly(semi_major_axis: f32, eccentricity: f32, true_anomaly: f32) -> f32 {

    let semi_latus_rectum = semi_major_axis * (1. - eccentricity.powf(2.));
    let radius = semi_latus_rectum / (1. + eccentricity * true_anomaly.cos());

    radius
}

pub fn orbital_position_at_true_anomaly(orbit: Orbit, true_anomaly: f32) -> Vec3 {
    let direction = Quat::from_axis_angle(orbit.orbital_normal(), true_anomaly) * orbit.periapsis_dir();
    let radius = radius_at_true_anomaly(orbit.semi_major_axis, orbit.eccentricity, true_anomaly);

    orbit.ref_pos + direction * radius
}
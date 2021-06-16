use std::f32::consts::PI;
use bevy::{math::{Quat, Vec3}, prelude::Transform};
use rand::{Rng, thread_rng};

#[derive(Debug, Copy, Clone)]
pub struct OrbitParameters {
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

    // / ### Mean Anomaly
    // /
    // / `M = n(t-t1)` from mean angular motion
    // /
    // / `M = E - e sin E` from eccentric anomaly
    // /
    // / `M = M0 + n(t-t0)` => M0 = mean anomaly at epoch and t0 is the epoch
    // /
    // / => t => current time
    // / => t1 => time which the body is at pericenter / periapsis
    // / => (t-t1) => time difference between then and now
    // pub mean_anomaly: f32,

    // /// ### Eccentric Anomaly
    // /// 
    // pub eccentric_anomaly: f32,

    // /// ### Mean Angular Motion
    // ///
    // /// `n = 2PI / T`
    // ///
    // /// `T` => orbital period
    // pub mean_angular_motion: f32,

    // /// ### Orbital Period
    // ///
    // /// The time it takes for one full complete orbit
    // ///
    // /// `T`
    // pub orbital_period: f32
}

impl OrbitParameters {
    pub fn from_rand() -> OrbitParameters {
        let mut rng = thread_rng();

        OrbitParameters {
            eccentricity: rng.gen_range(0.0..1.0),
            semi_major_axis: rng.gen_range(0.5..=2.5),
            longitude_of_ascending_node: rng.gen_range(0.0..PI*2.),
            inclination: rng.gen_range(0.0..PI*2.),
            argument_of_periapsis: rng.gen_range(0.0..PI*2.),
            true_anomaly: rng.gen_range(0.0..PI*2.),
            // mean_anomaly: 0.0,
            // eccentric_anomaly: 0.0,
            // mean_angular_motion: 0.0,
            // orbital_period: 0.0,
        }
    }

    pub fn orbit_ring(&self, reference_frame: &Transform) -> Vec<Vec3> {
        let mut ring_positions = Vec::new();
        let steps = 36;
        let step_angle = 2. * PI / steps as f32;
        
        for n in 0..steps {
            ring_positions.push(orbital_position_at_true_anomaly(*self, step_angle * n as f32, reference_frame));
        }

        return ring_positions;
    }
}

impl OrbitParameters {
    /// Direction of the normal of the orbital plane
    pub fn orbital_normal(&self, reference_frame: &Transform) -> Vec3 {
        Quat::from_axis_angle(
            self.ascending_dir(reference_frame), 
            self.inclination) 
            * reference_frame.local_y()
    }

    /// Direction of the ascending node
    pub fn ascending_dir(&self, reference_frame: &Transform) -> Vec3 {
        Quat::from_axis_angle(
            reference_frame.local_y(), 
            self.longitude_of_ascending_node) 
            * reference_frame.local_z()
    }

    /// Direction of descending node
    pub fn descending_dir(&self, reference_frame: &Transform) -> Vec3 {
        -self.ascending_dir(reference_frame)
    }

    /// Direction of periapsis
    pub fn periapsis_dir(&self, reference_frame: &Transform) -> Vec3 {
        Quat::from_axis_angle(
            self.orbital_normal(reference_frame), 
            self.argument_of_periapsis) 
            * self.ascending_dir(reference_frame)
    }

    /// Direction of apoapsis
    pub fn apoapsis_dir(&self, reference_frame: &Transform) -> Vec3 {
        -self.periapsis_dir(reference_frame)
    }
}

impl OrbitParameters {
    fn ascending_node(&self, reference_frame: &Transform) -> Vec3 {
        reference_frame.translation
        + self.ascending_dir(reference_frame) 
        * radius_at_true_anomaly(
            self.semi_major_axis, 
            self.eccentricity, 
            self.longitude_of_ascending_node + self.true_anomaly
        )
    }

    fn descending_node(&self, reference_frame: &Transform) -> Vec3 {
        reference_frame.translation
        + self.descending_dir(reference_frame)
        * radius_at_true_anomaly(
            self.semi_major_axis, 
            self.eccentricity, 
            self.longitude_of_ascending_node + PI + self.true_anomaly
        )
    }

    fn periapsis_node(&self, reference_frame: &Transform) -> Vec3 {
        let radius = self.semi_major_axis * (1. - self.eccentricity);

        reference_frame.translation
        + self.periapsis_dir(reference_frame)
        * radius
    }

    fn apoapsis_node(&self, reference_frame: &Transform) -> Vec3 {
        let radius = self.semi_major_axis * (1. + self.eccentricity);

        reference_frame.translation
        + self.apoapsis_dir(reference_frame)
        * radius
    }
}

// pub struct OrbitDetails {
//     pub periapsis: Vec3,
//     pub apoapsis: Vec3,
//     pub ascending_node: Vec3,
//     pub descending_node: Vec3,
//     pub body_pos: Vec3,
// }


// pub fn orbit_positions(orbit: OrbitParameters, reference_forward: Vec3, reference_up: Vec3, reference_position: Vec3) -> OrbitDetails {

//     // Radii
//     let radius_periapsis = orbit.semi_major_axis * (1. - orbit.eccentricity);
//     let radius_apoapsis= orbit.semi_major_axis * (1. + orbit.eccentricity);

//     let ascending_node_rot = Quat::from_axis_angle(reference_up, orbit.longitude_of_ascending_node);
//     let ascending_node_direction = ascending_node_rot * reference_forward; // this should be reprojected down to the actual orbit position
//     // let ascending_node_position = ascending_node_direction + reference_position;
//     // Inclination
//     let inclination_rot = Quat::from_axis_angle(ascending_node_direction, orbit.inclination);
//     let orbit_normal = inclination_rot * reference_up;

//     let periapsis_dir = Quat::from_axis_angle(orbit_normal, orbit.argument_of_periapsis) * ascending_node_direction;

//     let periapsis_position = periapsis_dir * radius_periapsis + reference_position;
//     let apoapsis_position = -periapsis_dir * radius_apoapsis + reference_position;

//     let eccentricity_vector = (periapsis_position - apoapsis_position).normalize() * orbit.eccentricity;

//     let true_anomaly_direction = Quat::from_axis_angle(orbit_normal, orbit.true_anomaly) * periapsis_dir; // todo: replaced by eccentricity vector? it has the same direction

//     let true_anom_radius = radius_at_true_anomaly(orbit.semi_major_axis, orbit.eccentricity, orbit.true_anomaly);

//     OrbitDetails {
//         ascending_node: reference_position + ascending_node_direction,
//         descending_node: reference_position - ascending_node_direction,
//         periapsis: periapsis_position,
//         apoapsis: apoapsis_position,
//         body_pos: true_anomaly_direction * true_anom_radius,
//     }
// }

fn radius_at_true_anomaly(semi_major_axis: f32, eccentricity: f32, true_anomaly: f32) -> f32 {

    let semi_latus_rectum = semi_major_axis * (1. - eccentricity.powf(2.));
    let radius = semi_latus_rectum / (1. + eccentricity * true_anomaly.cos());

    radius
}

pub fn orbital_position_at_true_anomaly(orbit: OrbitParameters, true_anomaly: f32, reference_frame: &Transform) -> Vec3 {
    let direction = Quat::from_axis_angle(orbit.orbital_normal(reference_frame), true_anomaly) * orbit.periapsis_dir(reference_frame);
    let radius = radius_at_true_anomaly(orbit.semi_major_axis, orbit.eccentricity, true_anomaly);

    reference_frame.translation + direction * radius
}
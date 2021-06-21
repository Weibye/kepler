use std::{f32::consts::PI, ops::Rem};

use bevy_math::{Quat, Vec3};
use bevy_transform::components::Transform;
use rand::{Rng, thread_rng};

use crate::{calc_true_anomaly, eccentric_anomaly_solver, radius_at_true_anomaly};


pub struct EllipticalOrbit {
    eccentricity: f32,
    semimajor_axis: f32,

    /// True anomaly
    ///
    /// Notation: `θ`
    true_anomaly: f32,
    longitude_of_ascending_node: f32,
    argument_of_periapsis: f32,
    
    /// Inclination
    ///
    /// Notation: `i`
    inclination: f32,

    /// Orbital period
    ///
    /// Notation: `T`
    period: f32,
}

impl EllipticalOrbit {
    // Constructors
    pub fn new(
        eccentricity: f32, 
        semimajor_axis: f32, 
        true_anomaly: f32, 
        longitude_of_ascending_node: f32, 
        argument_of_periapsis: f32, 
        inclination: f32,
        period: f32,
    ) -> Self {
        EllipticalOrbit {
            eccentricity,
            semimajor_axis,
            true_anomaly,
            longitude_of_ascending_node,
            argument_of_periapsis,
            inclination,
            period,
        }
    }

    pub fn from_rand() -> Self {
        let mut rng = thread_rng();
        EllipticalOrbit {
            eccentricity: rng.gen_range(0.0..1.0),
            semimajor_axis: rng.gen_range(0.5..=2.5),
            longitude_of_ascending_node: rng.gen_range(0.0..PI*2.),
            inclination: rng.gen_range(0.0..PI*2.),
            argument_of_periapsis: rng.gen_range(0.0..PI*2.),
            true_anomaly: rng.gen_range(0.0..PI*2.),
            period: rng.gen_range(1.0..60.0),
        }
    }

    // Getters
    pub fn eccentricity(&self) -> f32 { self.eccentricity }
    pub fn semimajor_axis(&self) -> f32 { self.semimajor_axis }
    pub fn true_anomaly(&self) -> f32 { self.true_anomaly }
    pub fn longitude_of_ascending_node(&self) -> f32 { self.longitude_of_ascending_node }
    pub fn argument_of_periapsis(&self) -> f32 { self.argument_of_periapsis }
    pub fn inclination(&self) -> f32 { self.inclination }
    pub fn period(&self) -> f32 { self.period }

    // Setters
    pub fn set_true_anomaly(&mut self, value: f32) { self.true_anomaly = value; }


    // Orbital Position
    // pub fn get_position_vector(&self, reference: &Transform) -> Vec3 {
    //     let direction = Quat::from_axis_angle(self.zenith(reference), self.true_anomaly) * self.periapsis(reference);
    //     let radius = radius_at_true_anomaly(self.eccentricity, self.true_anomaly, self.semimajor_axis);

    //     direction.normalize() * radius
    // }


    // Average motion of mean anomaly
    pub fn mean_angular_motion(&self) -> f32 {
        2.0 * PI / self.period()
    }

    // helper value
    pub fn mean_anomaly(&self, time: f32) -> f32 {
        let time_at_periapsis = 0.0;
        let mean_anom = self.mean_angular_motion() * (time - time_at_periapsis);

        mean_anom.rem(2.0*PI)
    }

    pub fn true_anomaly_at_time(&self, time: f32) -> f32 {
        let eccentric_anomaly = eccentric_anomaly_solver(self.mean_anomaly(time), self.eccentricity);

        calc_true_anomaly(self.eccentricity, eccentric_anomaly)
    }
}

use std::{f32::consts::PI, ops::Rem};


pub struct Orbit {
    /// True anomaly
    ///
    /// Notation: `θ`
    true_anomaly: f32,
    /// Orbital period
    ///
    /// Notation: `T`
    period: f32,
}

impl Orbit {
    pub fn new(true_anomaly: f32, period: f32) -> Self {
        Orbit {
            true_anomaly,
            period,
        }
    }

    // Getters
    pub fn true_anomaly(&self) -> f32 { self.true_anomaly }
    pub fn period(&self) -> f32 { self.period }

    // Setters
    pub fn set_true_anomaly(&mut self, value: f32) { self.true_anomaly = value; }

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
}
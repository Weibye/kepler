use crate::Time;

/// Radian
///
/// Notation: `r`
pub struct Radian(f64);

impl Radian {
    pub fn new(value: f64) -> Self {
        Radian(value)
    }
    pub fn val(&self) -> &f64 { &self.0 }
}

/// Angle
///
/// Notation: `θ`
pub struct Angle(Radian);

/// Angular velocity
///
/// Notation: `ω`
///
/// Definition: `ω = dθ / dt`
pub struct AngularVelocity(f64);

impl AngularVelocity {
    pub fn new(value: f64) -> Self {
        AngularVelocity(value)
    }

    pub fn from_delta(delta_angle: Radian, delta_time: Time) -> Self {
        AngularVelocity(
            delta_angle.val() / delta_time.val()
        )
    }

    pub fn val(&self) -> &f64 { &self.0 }
}

pub struct AngularAcceleration(f64);
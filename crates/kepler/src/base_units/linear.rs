use bevy_math::DVec3;

/// Velocity
///
/// Notation: `v`
pub struct Veclocity(f64);
pub struct VelocityVec3(DVec3);

/// Acceleration
pub struct Acceleration(f64);
pub struct AccelerationVec3(DVec3);

/// Distance
///
/// Notation: `l`
///
/// Units: `m`
pub struct Length(f64);
pub struct LengthVec3(DVec3);
pub struct Position(DVec3);
use bevy::{core::Time, prelude::{Query, Res}};
use kepler::EllipticalOrbit;


pub(crate) fn drive_orbits(
    mut orbits: Query<&mut EllipticalOrbit>,
    time: Res<Time>,
) {
    for mut orbit in orbits.iter_mut() {
        let new_true_anomaly = orbit.true_anomaly_at_time(time.seconds_since_startup() as f32);
        orbit.set_true_anomaly(new_true_anomaly);
    }
}
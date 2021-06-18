use crate::*;


struct EllipticalOrbitDifficult {
    eccentricity: Eccentricity,
    semimajor_axis: SemiMajorAxis,
    inclination: Inclination,
    longitude_of_ascending_node: Angle,
    argument_of_periapsis: Angle,
    true_anomaly: Angle,
}

// impl EllipticalOrbit {}

struct OrbitalPlaneDifficult {
    longitude_of_ascending_node: Angle,
    argument_of_periapsis: Angle,
    true_anomaly: Angle,
}

struct CircularOrbitDifficult {
    radius: Length,
}
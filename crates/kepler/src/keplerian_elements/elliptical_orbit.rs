use crate::*;


struct EllipticalOrbit {
    eccentricity: Eccentricity,
    semimajor_axis: SemiMajorAxis,
    inclination: Inclination,
    longitude_of_ascending_node: Angle,
    argument_of_periapsis: Angle,
    true_anomaly: Angle,
}

// impl EllipticalOrbit {}

struct OrbitalPlane {
    longitude_of_ascending_node: Angle,
    argument_of_periapsis: Angle,
    true_anomaly: Angle,
}

struct CircularOrbit {
    radius: Length,
}
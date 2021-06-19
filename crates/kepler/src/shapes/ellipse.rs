use std::f32::consts::PI;

pub struct Ellipse {
    semi_major: f32,
    semi_minor: f32,
    eccentricity: f32,
}

impl Ellipse {
    // Constructors
    pub fn from_major(semi_major: f32, eccentricity: f32) -> Self {
        Ellipse {
            semi_major,
            semi_minor: get_semi_minor_axis(semi_major, eccentricity),
            eccentricity,
        }
    }
    pub fn from_semi(semi_major: f32, semi_minor: f32) -> Self {
        Ellipse {
            semi_major,
            semi_minor,
            eccentricity: get_eccentricity(semi_major, semi_minor),
        }
    }

    // Getters
    pub fn semi_major(&self) -> f32 { self.semi_major }
    pub fn semi_minor(&self) -> f32 { self.semi_minor }
    pub fn major_axis(&self) -> f32 { 2.0 * self.semi_major }
    pub fn minor_axis(&self) -> f32 { 2.0 * self.semi_minor }
    pub fn eccentricity(&self) -> f32 { self.eccentricity }

    pub fn area(&self) -> f32 { PI * self.semi_major * self.semi_minor }

    pub fn perimeter_point(&self, angle: f32) -> (f32, f32) {
        let x = self.semi_major * angle.cos();
        let y = self.semi_minor * angle.sin();

        (x, y)
    }



}

/// Returns the semi-major axis of an ellipse using the semi-major and eccentricity
pub fn get_semi_minor_axis(semi_major_axis: f32, eccentricity: f32) -> f32 {
    semi_major_axis * (1.0 - eccentricity.powf(2.0)).sqrt()
}

// pub fn get_semi_major_axis(semi_minor_axis: f32, eccentricity: f32) -> f32 {

// }

/// Returns the eccentricity of an ellipse using the semi major and minor axis
pub fn get_eccentricity(semi_major: f32, semi_minor: f32) -> f32 {
    (1.0 - semi_minor.powf(2.0) / semi_major.powf(2.0)).sqrt()
}


use std::f32::consts::PI;

/// An ellipse defined in XY space with the center as (X,Y) = (0,0)
///
/// https://en.wikipedia.org/wiki/Conic_section#Conic_parameters
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
            semi_minor: Self::get_semi_minor_axis(semi_major, eccentricity),
            eccentricity,
        }
    }
    pub fn from_semi(semi_major: f32, semi_minor: f32) -> Self {
        Ellipse {
            semi_major,
            semi_minor,
            eccentricity: Self::get_eccentricity(semi_major, semi_minor),
        }
    }

    // Getters
    pub fn semi_major(&self) -> f32 { self.semi_major }
    pub fn semi_minor(&self) -> f32 { self.semi_minor }
    pub fn major_axis(&self) -> f32 { 2.0 * self.semi_major }
    pub fn minor_axis(&self) -> f32 { 2.0 * self.semi_minor }
    pub fn eccentricity(&self) -> f32 { self.eccentricity }

    // Setters
    pub fn set_eccentricity(&mut self, value: f32) {
        self.eccentricity = value;
        self.semi_minor = Ellipse::get_semi_minor_axis(self.semi_major, self.eccentricity);
    }
    pub fn set_semi_minor(&mut self, value: f32) { self.semi_minor = value; }
    pub fn set_semi_major(&mut self, value: f32) { self.semi_major = value; }

    pub fn area(&self) -> f32 { PI * self.semi_major * self.semi_minor }

    /// Returns the point on the perimeter with angle `a` from the center
    ///
    /// `x` is along the major axis
    /// `y` is along the minor axis
    pub fn perimeter_point(&self, angle: f32) -> (f32, f32) {
        let x = self.semi_major * angle.cos();
        let y = self.semi_minor * angle.sin();

        (x, y)
    }

    /// The focal parameter 'p' is the distance from a focus to the corresponding directrix.
    pub fn focal_parameter(&self) -> f32 {
        self.semi_minor.powf(2.0) / (self.semi_major.powf(2.0) - self.semi_minor.powf(2.0)).sqrt()
    }

    /// The linear eccentricity 'c' is the distance between the center and a focus.
    pub fn linear_eccentricity(&self) -> f32 {
        (self.semi_major.powf(2.0) - self.semi_minor.powf(2.0)).sqrt()
    }
    /// The latus rectum is the chord parallel to the directrix and passing through a focus; its half-length is the semi-latus rectum (ℓ).
    pub fn semi_latus_rectum(&self) -> f32 {
        self.semi_minor.powf(2.0) / self.semi_major
    }

    // Statics
    /// Returns the semi-minor axis of an ellipse using the semi-major and eccentricity
    pub fn get_semi_minor_axis(semi_major_axis: f32, eccentricity: f32) -> f32 {
        semi_major_axis * (1.0 - eccentricity.powf(2.0)).sqrt()
    }

    // pub fn get_semi_major_axis(semi_minor_axis: f32, eccentricity: f32) -> f32 {

    // }

    /// Returns the eccentricity of an ellipse using the semi major and minor axis
    pub fn get_eccentricity(semi_major: f32, semi_minor: f32) -> f32 {
        (1.0 - semi_minor.powf(2.0) / semi_major.powf(2.0)).sqrt()
    }
}

/// Mass
///
/// Notation: `m`
///
/// Units: `kg`
pub struct Mass(f64);

impl Mass {
    pub fn new(value: f64) -> Self {
        Mass(value)
    }
    
    pub fn val(&self) -> &f64 { &self.0 }
}

/// Area
///
/// Notation: `A`
///
/// Units: `m²`
pub struct Area(f64);

/// Volume
///
/// Notation: `V`
///
/// Units: `m³`
pub struct Volume(f64);

impl Volume {
    fn new(volume: f64) -> Self {
        Volume(volume)
    }
    fn value(&self) -> f64 {
        self.0
    }
}

/// Density
///
/// Notation: `ρ`
///
/// Definition: `ρ = m * V`
///
/// Units: `kg / m³`
pub struct Density(f64);

impl Density {
    fn new(density: f64) -> Self {
        Density(density)
    }

    fn from_mass_volume(mass: Mass, volume: Volume) -> Self {
        Density(mass.0 / volume.0)
    }
}

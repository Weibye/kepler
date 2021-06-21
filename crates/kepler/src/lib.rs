mod base_units;
mod orbits;
mod body;
mod shapes;
mod plane;
mod bundles;
mod utils;

pub use base_units::*;
pub use orbits::*;
pub use body::*;
pub use shapes::*;
pub use plane::*;
pub use bundles::*;
pub use utils::*;

use std::f32::consts::PI as PI32;
use std::f64::consts::PI as PI64;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;
use rand::*;

/// In a two-body problem with inverse-square-law force, every orbit is a Kepler orbit. The eccentricity of this Kepler orbit is a non-negative number that defines its shape.
///
/// The eccentricity may take the following values:
///
/// - circular orbit: e = 0
/// - elliptic orbit: 0 < e < 1
/// - parabolic trajectory: e = 1
/// - hyperbolic trajectory: e > 1
struct Eccentricity(f64);


/// The standard gravitational parameter μ of a celestial body
/// is the product of the gravitational constant G and the mass M of the body.
///
/// Notation:
/// `μ`
///
/// Definition:
/// `μ = GM`
struct GravitationalParameter(f64);

impl GravitationalParameter {
    pub fn new(mass: Mass) -> GravitationalParameter {
        GravitationalParameter(mass.val() * NEWTONIAN_CONSTANT_OF_GRAVITATION)
    }
}

#[derive(Debug)]
struct MeanAnomaly(f32);


struct EscapeVelocity(f64); // Should simply be Velocity?

impl EscapeVelocity {
    fn from_mass(mass: Mass, radius: f64) -> Self {
        EscapeVelocity((2. * NEWTONIAN_CONSTANT_OF_GRAVITATION * mass.val() / radius).sqrt())
    }
}


/// Orbital State Vectors
/// https://en.wikipedia.org/wiki/Orbital_state_vectors
struct OrbitalPositionVector();
struct OrbitalVelocityVector();

/// Circular orbital velocity at radius r
/// v = sqrt(G*M/r^2)

/// Elliptical orbits
/// relative position vector at periapsis rp:
/// rp = p / 1 + e
/// p: semi-latus-rectum
/// e: eccentricity
///
/// ra = p / 1 - e
/// 2a = rp + ra
/// a = p / 1 - e^2
///
/// r = a(1-e^2) / 1 + e cos true_anom
struct Dummy;


// TODO: Make this an PID controller
pub fn eccentric_anomaly_solver(mean_anomaly: f32, eccentricity: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let equality_threshold = 0.000001;
    let iterations = 100;
    // let period = OrbitalPeriod::new(Time::new(2.5));
    // // let mean_anomaly = MeanAngularMotion::from_period(period);
    // let mean_anomaly = 2. * PI / 365.0 * 24.0 * 60.0 * 60.0;
    // let eccentricity = 0.3;

    // M = E - e sin E
    // initial value of E
    
    let lower_bound = mean_anomaly - 10.0;
    let upper_bound = mean_anomaly + 10.0;
    let mut guess = rng.gen_range(lower_bound..upper_bound); // rng.gen_range(0.0..10.0);
    let mut exit_iteration = 0;
    // TOOD: Replace with a PID controller
    for n in 0..iterations {
        // run the calc
        let result = calc_eccentric(guess, eccentricity);
        let difference = mean_anomaly - result;

        // println!("Guess {:?} | Target: {:?} | Result: {:?} | Difference: {:?} | Upper: {:?} | Lower: {:?} | ", guess, mean_anomaly, result, difference, upper_bound, lower_bound);
        if difference.abs() <= equality_threshold { break; }
        
        guess += difference * 0.995;
        exit_iteration = n;
    }
    // println!("E is {:?} | {:?}", guess, exit_iteration);

    guess
}

pub fn calc_eccentric(eccentric_anomaly: f32, eccentricity: f32) -> f32 {
    eccentric_anomaly - eccentricity * eccentric_anomaly.sin()
}

pub fn calc_true_anomaly(eccentricity: f32, eccentric_anomaly: f32) -> f32 {
    2.0 * (((1.0 + eccentricity) / (1.0 - eccentricity)).sqrt() * (eccentric_anomaly / 2.0).tan()).atan()
}

pub fn radius_at_true_anomaly(eccentricity: f32, true_anomaly: f32, semi_major_axis: f32) -> f32 {
    // Check if there should be a 1.0 + eccentricity below the divider
    (semi_major_axis * (1.0 - eccentricity.powf(2.0))) / (1.0 + eccentricity * true_anomaly.cos())
}

/// Orbital Period
pub fn orbital_period(semimajor_axis: f32, mass: f32) -> f32 {
    2. * PI32 * (semimajor_axis.powf(3.0) / mass * NEWTONIAN_CONSTANT_OF_GRAVITATION as f32).sqrt()
}




#[test]
fn test() {
    let e = 5.0;
    let E = 10.0;
    let result = calc_eccentric(E, e);
    let expected = 12.720105554446849067023738309256886408418215064581119457870920063;
    assert_eq!(result, expected);


    println!("Result: {:?}", result);   
}
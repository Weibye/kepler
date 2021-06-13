/// Marker component to go with a transform
pub struct ReferenceFrame;


#[derive(Copy, Clone, Debug)]
pub struct OrbitalBody {
    pub mass: f32,
    pub radius: f32,
    pub angular_velocity: f32,
}
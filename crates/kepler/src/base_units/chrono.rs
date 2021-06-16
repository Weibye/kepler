/// Time
///
/// Notation: `T`
///
/// Units: `second`
pub struct Time(f64);

impl Time {
    pub fn new(time: f64) -> Self {
        Time(time)
    }
    
    pub fn val(&self) -> &f64 { &self.0 }
}
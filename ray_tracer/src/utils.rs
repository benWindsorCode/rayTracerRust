use crate::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return (degrees * PI)/180.0;
}

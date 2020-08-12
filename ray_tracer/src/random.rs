use rand::prelude::*;

// Wrapper around function to generate rand in [0,1), to match tutorial func name
pub fn random_double() -> f64 {
    let mut rng = thread_rng();

    rng.gen()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max-min) * random_double()
}

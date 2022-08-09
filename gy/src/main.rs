use std::f64::consts::{PI, E};

use rand::Rng;

fn g(y: f64) -> f64 {
    let result = (E.powf(-1.0/(2.0*y.powf(2.0))))/(20.0*(y.powf(2.0))*((PI*2.0).powf(0.5)));
    return result;
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut sum = 0.0;
    let max = 100000000;
    for _ in 0..max {
        let generated_number = rng.gen_range(0.0..0.05);
        sum += g(generated_number);
    }
    println!("{:e}", sum / max as f64);
}

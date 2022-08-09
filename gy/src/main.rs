use std::f64::consts::{PI, E};

use rand::Rng;

fn g(y: f64) -> f64 {
    let result = (E.powf(-1.0/(2.0*y.powf(2.0))))/(20.0*(y.powf(2.0))*((PI*2.0).powf(0.5)));
    return result;
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut sum_g = 0.0;
    let max = 100000000;
    
    for _ in 0..max {
        let generated_number = rng.gen_range(0.0..0.05);
        sum_g += g(generated_number);
    }
    let precise_result = sum_g/max as f64;
    println!("{:e}", precise_result);

    let mut sum_uncertainty = 0.0;

    for _ in 0..max{
        let generated_number = rng.gen_range(0.0..0.05);
        sum_uncertainty += (g(generated_number)-precise_result).powf(2.0);
    }

    let uncertainty = (sum_uncertainty/max as f64).powf(0.5);

    println!("{:e}", uncertainty);
}

use std::io;
use rand::{Rng, thread_rng};

fn main() {
    println!("Generating Random Number");
    let mut rng = thread_rng();
    let mut a : f64 = rng.gen_range(0.0..1.0);

    println!("{}", a);


}

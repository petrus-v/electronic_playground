extern crate cupi;

use std::thread::sleep;
use std::time::{Duration, Instant};
use cupi::{CuPi, Logic};



fn main() {
    let cupi = CuPi::new().unwrap();

    let output = cupi.pin(4).unwrap().output();
    let input = cupi.pin(5).unwrap().input();
    output.write(Logic::Low);

    // Allow module to settle
    sleep(Duration::from_millis(500));


    let elapsed: f64;

    output.write(Logic::High);
    // Send 10us pulse to trigger
    sleep(Duration::new(0, 10));
    output.write(Logic::Low);
    let mut start = Instant::now();

    // println!("wainting High");
    loop {
        match input.read() {
            Logic::Low => {
                start = Instant::now();
            },
            Logic::High => {
                break;
            }
        };
    }
    // println!("wainting Low");
    loop {
        match input.read() {
            Logic::High=> {},
            Logic::Low => {
                elapsed = start.elapsed().subsec_nanos() as f64;
                break;
            }
        };
    }
    // Distance = elapsed time / 2 * velocity
    // where velocity = 340m / s
    let distance = elapsed * 0.000000001 / 2.0 * 340.0;

    println!("Distance {} (en m)", distance);
}

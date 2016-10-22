extern crate cupi;

use std::thread::sleep;
use std::time::{Duration, Instant};
use cupi::{CuPi, Logic};



fn main() {
    let cupi = CuPi::new().unwrap();

    let output = cupi.pin(23).unwrap().output();
    let input = cupi.pin(24).unwrap().input();
    output.write(Logic::Low);

    // Allow module to settle
    sleep(Duration::from_millis(500));


    let elapsed_ms : f64;

    output.write(Logic::High);
    // Send 10us pulse to trigger
    sleep(Duration::from_millis(10));
    output.write(Logic::Low);
    let start = Instant::now();

//    println!("wainting High");
    loop {
        match input.read() {
            Logic::Low => {print!(".")},
            Logic::High => {break;}
        };
    }
//    println!("wainting Low");
    loop {
        match input.read() {
            Logic::High=> {print!(".")},
            Logic::Low => {
                elapsed_ms = start.elapsed().subsec_nanos() as f64;
                break;
            }
        };
    }
    // Distance = elapsed time / 2 * velocity
    // where velocity = 340m / s
    let distance = elapsed_ms * 0.000000001 / 2.0 * 340.0;

    println!("Distance {} (en m)", distance);

}

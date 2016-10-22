extern crate cupi;

use cupi::{CuPi, Logic};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let cupi = CuPi::new().unwrap();
    let output = cupi.pin(18).unwrap().output();
    loop {
        output.write(Logic::High);
        sleep(Duration::from_millis(200));
        output.write(Logic::Low);
        sleep(Duration::from_millis(200));
    }
}

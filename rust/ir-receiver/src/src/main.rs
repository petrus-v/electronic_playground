extern crate cupi;

use cupi::{CuPi, Logic};

fn main() {
    let cupi = CuPi::new().unwrap();
    let input = cupi.pin(1).unwrap().input();
    let mut current: Logic = Logic::Low;
    loop {
        match input.read() {
            Logic::Low => {
                match current {
                    Logic::High => {
                        println!("Light turned off");
                        current = Logic::Low;
                    },
                    _ => {}
                }
            },
            Logic::High => {
                match current {
                    Logic::Low => {
                        println!("Light switched On");
                        current = Logic::High;
                    },
                    _ => {}
                }
            }
        }
    }
}

extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let my_led = Pin::new(18); // number depends on chip, etc.
    my_led.with_exported(|| {
        loop {
            my_led.set_direction(Direction::Low).expect("Fail to change direction");
            my_led.set_value(0).expect("Fail to set value to 0");
            sleep(Duration::from_millis(200));
            my_led.set_value(1).expect("Fail to set value to 1");
            sleep(Duration::from_millis(200));
        }
    }).expect("Export gpio 18");
}

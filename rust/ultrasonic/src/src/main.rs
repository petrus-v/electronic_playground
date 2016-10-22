extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::{Duration, Instant};
use sysfs_gpio::{Direction, Edge, Pin};



fn main() {
    let output = Pin::new(23);
    let input = Pin::new(24);

    let mut start = Instant::now();
    let mut elapsed_ms : f64 = -1.0;

    input.with_exported(|| {
        try!(input.set_direction(Direction::In));
        try!(input.set_edge(Edge::BothEdges));
        let mut poller = try!(input.get_poller());
        output.with_exported(|| {
            try!(output.set_direction(Direction::Out));
            try!(output.set_value(0));
            // Allow module to settle
            sleep(Duration::from_millis(1000));
            try!(output.set_value(1));
            // Send 10us pulse to trigger
            sleep(Duration::from_millis(20));
            output.set_value(0);
            start = Instant::now();
            Ok(())
        }).expect("Export gpio 23");

        loop {
            match try!(poller.poll(1000)) {
                Some(value) => {
                    println!(
                        "poll value: {}",
                        value
                    );
                    if value == 0 {
                        elapsed_ms = start.elapsed().subsec_nanos() as f64;
                        break;
                    } else {
                        start = Instant::now();
                    }
                },
                None => {break;}
            };
        }
        Ok(())
    }).expect("Export gpio 24");

    // Distance = elapsed time / 2 * velocity
    // where velocity = 340m / s
    let distance = elapsed_ms * 0.000000001 / 2.0 * 340.0;

    println!("Distance {} (en m)", distance);

}

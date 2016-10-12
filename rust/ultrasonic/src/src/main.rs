extern crate sysfs_gpio;

use std::io::prelude::*;
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant};
use sysfs_gpio::{Direction, Edge, Pin};



fn main() {
    let output = Pin::new(23);
    let input = Pin::new(24);

    let mut start = Instant::now();
    let mut elapsed_ms : f64 = 0.0;

    output.with_exported(|| {
        try!(output.set_direction(Direction::Out));
        try!(output.set_value(0));
        // Allow module to settle
        sleep(Duration::from_millis(100));
        try!(output.set_value(1));
        // Send 10us pulse to trigger
        sleep(Duration::from_millis(10));
        start = Instant::now();
        output.set_value(0)
    }).expect("Export gpio 23");

    input.with_exported(|| {
        try!(input.set_direction(Direction::In));
        try!(input.set_edge(Edge::FallingEdge));
        let mut poller = try!(input.get_poller());
        loop {
            match try!(poller.poll(1000)) {
                Some(value) => {
                    start = Instant::now();
                    println!("poll value: {}", value);
                },
                None => {
                    let mut stdout = stdout();
                    try!(stdout.write(b"."));
                    try!(stdout.flush());
                }
            }
        }
        try!(input.set_edge(Edge::RisingEdge));
        let mut poller = try!(input.get_poller());
        loop {
            match try!(poller.poll(1000)) {
                Some(value) => {
                    elapsed_ms = start.elapsed().subsec_nanos() as f64;
                    println!("poll value: {}", value);
                },
                None => {
                    let mut stdout = stdout();
                    try!(stdout.write(b"."));
                    try!(stdout.flush());
                }
            }
        }
        try!(input.set_edge(Edge::FallingEdge));
        let mut poller = try!(input.get_poller());
        loop {
            match try!(poller.poll(1000)) {
                Some(value) => {
                    elapsed_ms = start.elapsed().subsec_nanos() as f64;
                    println!("poll value: {}", value);
                },
                None => {
                    let mut stdout = stdout();
                    try!(stdout.write(b"."));
                    try!(stdout.flush());
                }
            }
        }
    }).expect("Export gpio 24");

    // Distance = elapsed time / 2 * velocity
    // where velocity = 340m / s
    let distance = elapsed_ms * 0.000000001 / 2.0 * 340.0;

    println!("Distance {} (en m)", distance);

}
# Ultrasonic sensor with rust

This is a port of [ultrasonic python version](../../python/ultrasonic) 
to rust.

## Build docker image

```bash
$ docker build -t rpi-raspbian-rust-ultrasonic .
```


## Compile & run rust program

```bash
$ docker run --rm -it \
      -v /home/pirate/electronic_playground/rust/ultrasonic/src/:/usr/src/ultrasonic/ \
      -v /sys:/sys rpi-raspbian-rust-blink cargo run
```

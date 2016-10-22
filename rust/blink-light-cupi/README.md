# Blink a LED with Rust

The Light is connected to a Raspberry Pi3 through the GPIO (PIN 12 - GPIO18)

This code is launched with in docker container.

`src/main.rs` is an example use [cupi](https://github.com/inre/cupi).


## Build docker image

```bash
$ docker build -t rpi-raspbian-rust-blink-cupi .
```


## Compile & run rust program

```bash
$ docker run --rm -it \
      -v /home/pirate/electronic_playground/rust/blink-light-cupi/src/:/usr/src/blink-light/ \
      --cap-add SYS_RAWIO --device /dev/mem rpi-raspbian-rust-blink-cupi cargo run
```

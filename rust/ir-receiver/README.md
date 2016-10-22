# IR receiver with Rust

The IR receiver is connected to a Raspberry Pi3 through the GPIO (PIN 12 - GPIO18)

This code is launched with in docker container.

`src/main.rs` is an example that use [cupi](https://github.com/inre/cupi).


## Build docker image

```bash
$ docker build -t rpi-raspbian-rust-ir-receiver .
```


## Compile & run rust program

```bash
$ docker run --rm -it \
      -v /home/pirate/electronic_playground/rust/ir-receiver/src/:/usr/src/ir-receiver/ \
      --cap-add SYS_RAWIO --device /dev/mem rpi-raspbian-rust-ir-receiver cargo run
```

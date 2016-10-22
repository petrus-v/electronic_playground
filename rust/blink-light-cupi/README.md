# Blink a LED with Rust

The Light is connected to a Raspberry Pi3 through the GPIO (PIN 12 - GPIO18)

This code is launched with in docker container.

`src/main.rs` is an example copied and adpated from [posborne's rust-sysfs-gpio](
https://github.com/rust-embedded/rust-sysfs-gpio).


## Build docker image

```bash
$ docker build -t rpi-raspbian-rust-blink .
```


## Compile & run rust program

```bash
$ docker run --rm -it \
      -v /home/pirate/electronic_playground/rust/blink-light/src/:/usr/src/blink-light/ \
      -v /sys:/sys rpi-raspbian-rust-blink cargo run
```

* `-v path/to/blink-light/src/:/usr/src/blink-light`: is used to map the volume
  that you can change code source, it will be seen in the Docker container without
  rebuild every times

* `-v /sys:/sys`: is mapped because rust-sysfs-gpio needs to access to
  `/sys/class/gpio/*` files. Read kernel documentation to get more informations
   about [how gopio sysfs works](
   https://www.kernel.org/doc/Documentation/gpio/sysfs.txt)

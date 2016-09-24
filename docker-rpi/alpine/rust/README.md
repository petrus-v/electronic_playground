# Alpine rust support for RPI.

This Dockerfile is based on alpine build supply by [hypriot](
https://github.com/hypriot/rpi-alpine-scratch) works. Expect to run
on RaspberryPi 3.

This image contains all sweets tools provide by rust while following
[rust install instructions](https://www.rust-lang.org/en-US/downloads.html)

## Building the image

### Example:

```bash
$ git clone https://github.com/petrus-v/electronic_playground
$ cd electronic_playground/docker-rpi/alpine/rust
$ docker build -t rpi-alpine-rust:beta --build-arg channel=beta .
```

### Option

Following options are available at build time:

* **channel**: The rust channel can be either one of the following
  values (please have a look to [rust](https://www.rust-lang.org/
  en-US/downloads.html) to know the current state of those channels):
    * *stable* (default): Rust sable, this is the default value
      if not provide
    * *beta*: Rust beta
    * *nightly*: Rust nightly

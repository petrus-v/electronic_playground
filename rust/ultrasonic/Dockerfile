FROM petrusv/rpi-raspbian-rust:nightly

ENV user=root
RUN mkdir -p /usr/src/ultrasonic
WORKDIR /usr/src/ultrasonic

COPY ./src /usr/src/ultrasonic
# build the project once to cache required crates
RUN cargo build
CMD ['cargo', 'run']

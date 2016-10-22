FROM petrusv/rpi-raspbian-rust:nightly

ENV user=root
RUN mkdir -p /usr/src/blink-light
WORKDIR /usr/src/blink-light

COPY ./src /usr/src/blink-light
# build the project once to cache required crates
RUN cargo build
CMD ['cargo', 'build']

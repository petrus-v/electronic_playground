FROM petrusv/rpi-raspbian-rust:nightly

ENV user=root
RUN mkdir -p /usr/src/ir-receiver
WORKDIR /usr/src/ir-receiver

COPY ./src /usr/src/ir-receiver
# build the project once to cache required crates
RUN cargo build
CMD ['cargo', 'run']

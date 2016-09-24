
FROM resin/rpi-raspbian:jessie-20160831
MAINTAINER Pierre Verkest <petrus-v@hotmail.fr>

# expected channel, can be overwride at build time
ARG channel=stable

ADD https://static.rust-lang.org/rustup.sh ./

# disable-sudo because we are already root!
RUN chmod +x ./rustup.sh \
    && apt-get update \
    && apt-get install \
        curl \
        gcc \
        libc6-dev \
        file \
        --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* \
    && ./rustup.sh --disable-sudo -s -- --channel=$channel

CMD ["rustc", "--version"]

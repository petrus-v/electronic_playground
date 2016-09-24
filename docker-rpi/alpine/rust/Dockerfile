FROM hypriot/rpi-alpine-scratch:v3.4
MAINTAINER Pierre Verkest <petrus-v@hotmail.fr>

# expected channel, can be overwride at build time
ARG channel=stable

ADD https://static.rust-lang.org/rustup.sh ./

# disable-sudo because we are already root!
RUN chmod +x ./rustup.sh \
    && apk add --no-cache --virtual .build-deps  \
        curl \
        file \
        gcc \
        libc6-compat \
    && ln -s /lib/libc.musl-armhf.so.1 /lib/ld-linux.so.3 \
    && ./rustup.sh --disable-sudo -s -- --channel=$channel

CMD ["rustc", "--version"]

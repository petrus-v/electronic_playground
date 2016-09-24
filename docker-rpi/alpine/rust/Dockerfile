FROM hypriot/rpi-alpine-scratch:v3.4
MAINTAINER Pierre Verkest <petrus-v@hotmail.fr>

# expected channel, can be overwride at build time
ARG channel=stable

ADD https://static.rust-lang.org/rustup.sh ./
RUN chmod +x ./ && \
    ./rustup.sh -s -- --channel=$channel

CMD ["rustc", "--version"]

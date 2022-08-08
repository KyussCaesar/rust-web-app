FROM debian:buster-slim

ENV MANUAL_DOCKER_CACHE=0

RUN \
  echo $MANUAL_DOCKER_CACHE \
  && apt-get update -y \
  && apt-get upgrade -y \
  && apt-get install -y --no-install-recommends \
    libssl1.1 \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/kyusscaesar/rust-web-app

ARG RUST_TARGET=debug
COPY ./target/$RUST_TARGET/rust-web-app .
CMD ["./rust-web-app"]

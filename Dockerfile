FROM debian:buster-slim

ENV MANUAL_DOCKER_CACHE=0

RUN \
  echo $MANUAL_DOCKER_CACHE && \
  apt-get update -y && \
  apt-get upgrade -y && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /opt/kyusscaesar/rust-web-app
COPY ./target/release/rust-web-app .
CMD ["rust-web-app"]

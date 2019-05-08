FROM rust:1.31

WORKDIR /usr/src/godwin
COPY . .

RUN apt-get update && apt-get install -y --force-yes libsodium-dev libssl1.0-dev
RUN cargo clean
RUN cargo install --path .

CMD ["godwin-bot"]
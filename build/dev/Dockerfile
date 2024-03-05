FROM rust:1.76.0 AS builder
WORKDIR /usr/src/homey
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build
COPY . .
RUN cargo build

CMD ["cargo", "run"]
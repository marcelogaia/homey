FROM rust:1.75 AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/homey /usr/local/bin/homey
CMD ["homey"]
FROM rust:1.75 AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/src/myapp/target/x86_64-unknown-linux-musl/release/homey /
CMD ["/homey"]
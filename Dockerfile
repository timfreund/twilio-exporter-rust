FROM rust:latest AS builder
WORKDIR /usr/src/twilio-exporter
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN find /usr/src/twilio-exporter
RUN cargo build --release

FROM debian:bookworm-slim
# FROM scratch
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates
WORKDIR /usr/bin
COPY --from=builder /usr/src/twilio-exporter/target/release/twilio-exporter /usr/bin
CMD ["/usr/bin/twilio-exporter"]

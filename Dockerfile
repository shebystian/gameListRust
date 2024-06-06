FROM rust:1.65.0-slim as builder

COPY ./ ./

# Build your program for release
RUN cargo build --release

# Run the binary
ENTRYPOINT ["./target/release/ms_bpd"]

EXPOSE 8080

# comentarios contenedor rust FROM rust:1.65 as builder, pesa 2,01 GB#
# rust:1.65.0-slim pesa 1.42GB
FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/pdfverse_backend ./pdfverse_backend

# Runtime image
FROM debian:bookworm-slim

# Install OpenSSL 3 and any other required dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Run as "app" user
RUN useradd -ms /bin/bash app

# Update library search paths for OpenSSL
RUN echo "/usr/local/lib64" > /etc/ld.so.conf.d/openssl.conf \
    && ldconfig

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/pdfverse_backend /app/pdfverse_backend


# Run the app
CMD ./pdfverse_backend

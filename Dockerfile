FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY benches ./benches
COPY tests ./tests

# Build release binary
RUN cargo build --release --bin pin-actions

# Runtime image
FROM debian:bookworm-slim

# Install git (required for git operations)
RUN apt-get update && \
    apt-get install -y git ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/pin-actions /usr/local/bin/pin-actions

# Create non-root user
RUN useradd -m -u 1000 pinactions && \
    chown -R pinactions:pinactions /usr/local/bin/pin-actions

USER pinactions
WORKDIR /workspace

ENTRYPOINT ["pin-actions"]
CMD ["--help"]

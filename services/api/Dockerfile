FROM python:3.11-bookworm

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev g++ cmake make libgomp1 libgfortran5 \
    && rm -rf /var/lib/apt/lists/*

# Install PyTorch CPU 2.4.0 (matches tch 0.17)
RUN pip install torch==2.4.0 --index-url https://download.pytorch.org/whl/cpu
ENV LIBTORCH_USE_PYTORCH=1

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY services/api ./services/api

# Build release binary
RUN cargo build --release --package api

EXPOSE 8080

CMD ["/app/target/release/api"]

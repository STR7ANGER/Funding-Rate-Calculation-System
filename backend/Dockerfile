FROM rust:1.83 as builder

WORKDIR /app

# Create a dummy workspace to cache dependencies
COPY Cargo.toml ./Cargo.toml
COPY backend/Cargo.toml ./backend/Cargo.toml

RUN mkdir -p backend/src && \
    echo "fn main() {}" > backend/src/main.rs

RUN cargo build -p funding-rate-backend --release || true

# Build with real sources
COPY . .
RUN cargo build -p funding-rate-backend --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/funding-rate-backend /app/funding-rate-backend

ENV API_HOST=0.0.0.0
ENV API_PORT=8080

EXPOSE 8080
CMD ["/app/funding-rate-backend"]


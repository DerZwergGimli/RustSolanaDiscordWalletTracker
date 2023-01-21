
FROM rust AS chef
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin SolanaDiscordWalletTracker

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/SolanaDiscordWalletTracker /usr/local/bin
RUN apt update && apt install -y libssl-dev openssl ca-certificates
RUN openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.pem -out cert.pem -subj "/C=GE/ST=London/L=London/O=Global Security/OU=IT Department/CN=example.com"
ENTRYPOINT ["/usr/local/bin/SolanaDiscordWalletTracker"]
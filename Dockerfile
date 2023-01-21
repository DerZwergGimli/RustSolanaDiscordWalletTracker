FROM rust:latest AS builder
WORKDIR build
RUN apt update && apt install -y libssl-dev pkg-config
COPY . .

RUN cargo build --release


FROM debian:bullseye
COPY --from=builder build/SolanaDiscordWalletTracker/target/release/SolanaDiscordWalletTracker ./
RUN apt update && apt install -y libssl-dev openssl ca-certificates
RUN openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.pem -out cert.pem -subj "/C=GE/ST=London/L=London/O=Global Security/OU=IT Department/CN=example.com"
ENTRYPOINT [ "./worker" ]
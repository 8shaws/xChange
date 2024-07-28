FROM rust:latest AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY crates/notif_worker/Cargo.toml crates/notif_worker/

RUN mkdir -p crates/notif_worker/src && echo "fn main() {}" > crates/notif_worker/src/main.rs

RUN cargo fetch

RUN rm crates/notif_worker/src/main.rs

COPY crates/notif_worker/src ./crates/notif_worker/src

RUN cargo build --release -p notif_worker

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/notif_worker ./xchange_notif_worker

CMD ["./xchange_notif_worker"]

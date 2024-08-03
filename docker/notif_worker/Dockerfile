FROM rust:latest AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY crates/notif_worker/Cargo.toml crates/notif_worker/
COPY crates/common/Cargo.toml crates/common/

RUN mkdir -p crates/notif_worker/src crates/common/src \
    && echo "fn main() {}" > crates/notif_worker/src/main.rs \
    && echo "fn main() {}" > crates/common/src/main.rs

RUN cargo fetch

RUN rm crates/notif_worker/src/main.rs crates/common/src/main.rs

COPY crates/notif_worker/src ./crates/notif_worker/src
COPY crates/common/src ./crates/common/src

RUN cargo build --release -p notif_worker

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/notif_worker ./xchange_notif_worker

CMD ["./xchange_notif_worker"]

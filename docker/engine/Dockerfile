FROM rust:latest AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/engine/Cargo.toml crates/engine/
COPY crates/common/Cargo.toml crates/common/

RUN mkdir -p crates/engine/src crates/common/src \
    && echo "fn main() {}" > crates/engine/src/main.rs \
    && echo "fn main() {}" > crates/common/src/main.rs

RUN cargo fetch

RUN rm crates/engine/src/main.rs crates/common/src/main.rs

COPY crates/engine/src ./crates/engine/src
COPY crates/common/src ./crates/common/src

RUN cargo build --release -p engine

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/engine ./xchange_engine

CMD ["./xchange_engine"]

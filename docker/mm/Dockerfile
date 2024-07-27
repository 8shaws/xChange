FROM rust:latest AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY crates/market_maker/Cargo.toml crates/market_maker/

RUN mkdir -p crates/market_maker/src && echo "fn main() {}" > crates/market_maker/src/main.rs

RUN cargo fetch

RUN rm crates/market_maker/src/main.rs

COPY crates/market_maker/src ./crates/market_maker/src

RUN cargo build --release -p market_maker

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/market_maker ./xchange_mm

CMD ["./xchange_mm"]

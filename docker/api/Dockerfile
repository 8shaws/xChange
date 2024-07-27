FROM rust:latest AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY crates/api/Cargo.toml crates/api/

RUN mkdir -p crates/api/src && echo "fn main() {}" > crates/api/src/main.rs

RUN cargo fetch

RUN rm crates/api/src/main.rs

COPY crates/api/src ./crates/api/src

RUN cargo build --release -p api

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/api ./xchange_api

EXPOSE 8080

CMD ["./xchange_api"]

# Chef base
FROM rust:1.59.0 as chef

WORKDIR /app
RUN apt update && apt install lld clang -y
RUN cargo install cargo-chef

# Planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin axum-zero2prod

# Runner
FROM ubuntu:20.04 AS runner

RUN apt update && apt install -y --no-install-recommends \
        openssl \
        ca-certificates \
    && apt autoremove \
    && apt clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/axum-zero2prod axum-zero2prod
COPY configuration .
ENV APP_ENVIRONMENT production

ENTRYPOINT [ "axum-zero2prod" ]
FROM lukemathwalker/cargo-chef:latest-rust-bullseye AS chef
WORKDIR /serenity-bot-template

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /serenity-bot-template/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin serenity-bot-template

FROM debian:bullseye-slim AS runtime
WORKDIR /serenity-bot-template
COPY --from=builder /serenity-bot-template/target/release/serenity-bot-template /usr/local/bin
ENTRYPOINT ["/usr/local/bin/serenity-bot-template"]
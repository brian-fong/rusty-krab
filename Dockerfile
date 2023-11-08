# ===== Build =====
FROM rust:1.72 AS build
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# ===== Run =====
FROM rust:1.72 AS run
WORKDIR /app
COPY --from=build /app/target/release/rusty-krab rusty-krab
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./rusty-krab"]

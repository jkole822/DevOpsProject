FROM rust:bullseye as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install sqlx-cli --no-default-features --features postgres
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
    libpq-dev \
    openssl \
    ca-certificates \
    libgcc-s1 \
    libstdc++6 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/auth /usr/local/bin/auth
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY --from=builder /usr/src/app/migrations ./migrations
COPY entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh
ENTRYPOINT ["./entrypoint.sh"]

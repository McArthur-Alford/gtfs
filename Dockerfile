# Chef Base
FROM docker.io/blackdex/rust-musl:x86_64-musl as chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

# Planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin gtfs

# Builder
FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json --bin gtfs
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --target x86_64-unknown-linux-musl --bin gtfs

FROM alpine AS runtime
RUN apk add --no-cache openssl
RUN addgroup -S user && adduser -S user -G user
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/gtfs /usr/local/bin/
CMD ["/usr/local/bin/gtfs"]

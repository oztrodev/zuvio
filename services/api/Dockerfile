# syntax=docker/dockerfile:1.7
# ---------------------------------------------------------------------------
# Zuvio API (Rust / Axum) - imagen de produccion multi-etapa
# Contexto de build: services/api   ->   docker build -t zuvio-api services/api
# ---------------------------------------------------------------------------

# ---------- Etapa 1: compilacion ----------
FROM rust:1-slim-bookworm AS builder
WORKDIR /app

# Cachea dependencias: compila primero con un main vacio
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && echo "" > src/lib.rs \
    && cargo build --release --locked \
    && rm -rf src target/release/deps/zuvio_api* target/release/zuvio-api* target/release/.fingerprint/zuvio-api*

# Compila el codigo real
COPY src ./src
RUN cargo build --release --locked --bin zuvio-api

# ---------- Etapa 2: runtime minimo ----------
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home --shell /usr/sbin/nologin zuvio

ENV TZ=America/Santiago \
    HOST=0.0.0.0 \
    PORT=4000 \
    ENVIRONMENT=production \
    RUST_LOG=info,zuvio_api=info

COPY --from=builder /app/target/release/zuvio-api /usr/local/bin/zuvio-api

USER zuvio
EXPOSE 4000

# Salud: GET /api/v1/health  (configurar en Railway / Render / Fly.io)
CMD ["zuvio-api"]

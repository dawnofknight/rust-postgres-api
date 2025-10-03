FROM --platform=$BUILDPLATFORM rust:latest as builder
WORKDIR /usr/src/app
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config cmake make gcc g++ \
    libssl-dev zlib1g-dev libsasl2-dev && \
    rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release

FROM --platform=$TARGETPLATFORM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libsasl2-2 zlib1g libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/scrape-consumer /app/
ENV RUST_LOG=info
ENTRYPOINT ["/app/scrape-consumer"]
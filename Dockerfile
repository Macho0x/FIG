# FIG Exchange Simulator
FROM rust:1.83-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p fig-exchange-sim

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/fig-exchange-sim /usr/local/bin/
EXPOSE 4433/udp
ENV RUST_LOG=info
CMD ["fig-exchange-sim"]

# Étape de build
FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# Étape runtime
FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/chat-server /usr/local/bin/chat-server
EXPOSE 8000
ENV RUST_LOG=info
CMD ["chat-server"]

FROM rust:1.68 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/algorithms-rust /usr/local/bin/algorithms-rust
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["algorithms-rust"]
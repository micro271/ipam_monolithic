FROM rust:1.82 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian

WORKDIR /app

COPY --from=builder /app/target/release/ipam_rs .

COPY .env .

COPY templates ./templates

COPY static ./static

CMD ["./api_ipam"]
FROM rust:1.75 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y openssl ca-certificates
COPY --from=builder /usr/src/app/target/release/netum_api /usr/local/bin/netum_api
EXPOSE 8000
CMD ["netum_api"]

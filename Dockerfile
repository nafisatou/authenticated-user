FROM rust:1.84 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/upload-service /app/upload-service
# create uploads directory (volume will mount over it if provided)
RUN mkdir -p /app/uploads
EXPOSE 8080
ENV BIND_ADDR=0.0.0.0:8080
CMD ["/app/upload-service"]
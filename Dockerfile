FROM rust:1.67.1 as builder
WORKDIR /usr/src/hello
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/hello /usr/local/bin/hello

CMD ["target/release/hello"]
FROM rust:1.67.1 as builder
WORKDIR /var/www/hello
COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /var/www/cargo/bin/hello /var/www/bin/hello

CMD ["target/release/hello"]
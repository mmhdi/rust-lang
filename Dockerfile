FROM rust:1.67.1 as builder
WORKDIR /hello
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/rust-rocket-sample /usr/local/bin/rust-rocket-sample

EXPOSE 3000

CMD ["hello"]
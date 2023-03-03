FROM rust:1.67.1 as builder
WORKDIR /hello
COPY . .
RUN cargo check

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/hello /usr/local/bin/hello

EXPOSE 3000

CMD ["hello"]
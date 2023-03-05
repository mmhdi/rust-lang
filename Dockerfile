FROM rust:1.67.1 as builder
WORKDIR /hello
COPY . .
RUN cargo run

EXPOSE 3000

CMD ["hello"]
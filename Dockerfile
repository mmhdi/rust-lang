FROM rust:1.67.1 as builder
WORKDIR /hello
COPY . .
RUN cargo check

CMD ["hello"]
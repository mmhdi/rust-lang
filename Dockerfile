FROM rust:1.67.1 as builder
WORKDIR /usr/src/hello
COPY . .

CMD ["target/release/hello"]
FROM rust:1.67 as builder

COPY . .

RUN cargo install --path .

CMD ["target/debug/hello"]

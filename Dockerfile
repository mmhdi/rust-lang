FROM rust:1.67.1 as builder
WORKDIR /usr/src/hello
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
ARG APP=/usr/src/hello

CMD ["hello"]
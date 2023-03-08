FROM rust:1.67.1 as builder
WORKDIR /hello
COPY . .
RUN rustc --explain E0277

EXPOSE 3000

CMD ["hello"]
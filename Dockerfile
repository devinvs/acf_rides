FROM rust:alpine3.16
COPY . .

RUN apk add --no-cache musl-dev
RUN cargo build --release

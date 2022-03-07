FROM rust:1.59

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

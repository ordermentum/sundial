FROM rust:1.52

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

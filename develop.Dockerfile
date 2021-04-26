FROM rust:1.51

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

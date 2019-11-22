FROM rust:1.39

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

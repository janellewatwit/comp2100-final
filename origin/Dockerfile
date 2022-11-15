FROM rust:latest

COPY . /app
WORKDIR /app
RUN cargo build --release
ENTRYPOINT [ "/app/target/release/origin" ]

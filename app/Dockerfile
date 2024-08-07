FROM rust:latest

WORKDIR /myapp

COPY . .

RUN cargo build --release && cargo run

CMD ["./target/release/app"]
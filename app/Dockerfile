FROM rust:latest

WORKDIR /myapp

COPY . .

RUN cargo build --release

CMD ["./target/release/mini-projet_mon_gestionnaire_de_compte_bancaire_simplifie"]
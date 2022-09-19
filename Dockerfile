FROM rust:1.63
COPY . .
RUN cargo build --release
CMD ["./target/release/smo-rusty-online"]

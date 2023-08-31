# Import Basic Image.
FROM rust:latest

COPY . .

# Cargo build Rust Project.
RUN cargo build

# Run bash on start.
CMD ["./target/release/backend_server"]

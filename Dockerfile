# Import Basic Image.
FROM rust:latest as build

# Cargo build Rust Project.
RUN \
  ls && \
  cargo build --release

# Build a production environment Docker Image.
FROM ubuntu:22.04
LABEL author="Snowball_233"

# Copy the binary files into Docker Image.
# | Please replace the specific path according to your needs.
COPY --from=build ./target/release /home/BackendServer

# Run bash on start.
CMD ["bash"]

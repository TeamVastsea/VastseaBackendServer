# Import Basic Image.
FROM rust:latest as build

# Set up work path.
WORKDIR .

# Set up Target Environment variable.
ENV OUT_DIR ./target

# Cargo build Rust Project.
RUN cargo build --release

# Build a production environment Docker Image.
FROM ubuntu:22.04
LABEL author="Snowball_233"

# Switch to Root account.
USER root

# Copy the binary files into Docker Image.
# | Please replace the specific path according to your needs.
COPY --from=build /home/app/target/release /home/BackendServer

# Clean up build cache.
RUN \
  apt-get clean && \
  apt-get autoclean && \
  rm -rf /var/lib/apt/lists/*

# Run bash on start.
CMD ["bash"]

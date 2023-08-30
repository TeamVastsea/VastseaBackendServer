# Import Background Image
FROM rust:latest as build

# Set up work path
WORKDIR /home/app

# Copy the Rust Project Files to Docker Image
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# Set up Target Environment variable
ENV OUT_DIR /home/app/target/

# Cargo build Rust Project
RUN cargo build --release

# Build a production environment Docker Image
FROM ubuntu:22.04
LABEL author="Snowball_233"

# Switch to Root account
USER root

# Ubuntu Initialization
RUN \
  ln -snf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && \
  echo 'Asia/Shanghai' > /etc/timezone && \
  apt-get update && \
  apt-get -y upgrade && \
  apt-get install -y htop vim

# Copy the binary files into Docker Image
# Please replace the specific path according to your needs
COPY --from=build /home/app/target/release/ /home/BackendServer

# Clean up build cache
RUN \
  apt-get clean && \
  apt-get autoclean && \
  rm -rf /var/lib/apt/lists/*

CMD ["bash", "cd /home/BackendServer"]

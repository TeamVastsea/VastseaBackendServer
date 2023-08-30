# Import Basic Image.
FROM rust:latest as build

# Set up work path.
WORKDIR /home/app

# Copy the Rust Project Files to Docker Image.
# | Please pay attention to the COPY command, which is defined in Docker docs as follows:
# | `Note: The directory itself is not copied, just its contents.`
# | URL: https://docs.docker.com/engine/reference/builder/
COPY ./Cargo.toml /home/app
COPY ./src /home/app/src
COPY ./build.rs /home/app

# Set up Target Environment variable.
ENV OUT_DIR /home/app/target

# Cargo build Rust Project.
RUN cargo build --release

# Build a production environment Docker Image.
FROM ubuntu:22.04
LABEL author="Snowball_233"

# Switch to Root account.
USER root

# Ubuntu Initialization.
# | Please replace the time zone according to your needs.
RUN \
  ln -snf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && \
  echo 'Asia/Shanghai' > /etc/timezone && \
  apt-get update && \
  apt-get -y upgrade && \
  apt-get install -y htop vim

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

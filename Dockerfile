# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef AS chef
WORKDIR /app

# prepare recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build artifact
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
## build dependencies
RUN cargo chef cook --release --recipe-path recipe.json
## build application
COPY . .
RUN cargo build --release

# build slim image
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/backend_server /home/BackendServer/backend_server
COPY --from=builder /app/target/release/key_tool /home/BackendServer/key_tool

RUN \
  apt-get update && \
  apt-get install -y curl vim openssl

RUN \
  apt-get clean && \
  apt-get autoclean && \
  rm -rf /var/lib/apt/lists/*

CMD ["bash"]

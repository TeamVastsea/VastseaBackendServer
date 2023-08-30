FROM rust:latest as build

# 设置工作路径
WORKDIR /app

# 将你的 Rust 项目文件（Cargo.toml 和 src 目录）复制到镜像中
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# 构建 Rust 项目
RUN cargo build --release

# 创建最终的生产镜像
FROM ubuntu:22.04

USER root

# 复制编译好的二进制文件到最终镜像中
COPY --from=build /app/target/release/BackendServer /home/BackendServer

# 设置运行命令
CMD ["./BackendServer"]

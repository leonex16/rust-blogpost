FROM rust:alpine3.14

RUN ["apk", "upgrade"]
RUN ["apk", "add", "--no-cache ", "musl-dev"]
RUN ["apk", "add", "libxcb-dev", "libxkbcommon-dev"]
RUN ["apk", "add", "build-base"]
RUN ["cargo", "vendor"]

# RUN ["cargo", "install", "cargo-watch"]

WORKDIR /app
COPY . .
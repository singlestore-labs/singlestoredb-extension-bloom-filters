FROM rust:slim-bullseye

RUN rustup target add wasm32-wasi wasm32-unknown-unknown && \
  rustup component add rust-src clippy rustfmt && \
  cargo install cargo-wasi

ENV LANG C.UTF-8

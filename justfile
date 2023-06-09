default:
  just --list

all: build test clippy fmt-check

build:
  cargo build

clippy:
  cargo clippy --all-targets --all-features

fmt:
  cargo +nightly fmt

fmt-check:
  cargo +nightly fmt --all -- --check

test:
  cargo test

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"

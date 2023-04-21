release:
  cargo build --release

build:
  cargo build --frozen

check:
  clear
  cargo c --frozen

format:
  cargo fmt
  cargo clippy

vendor:
  cargo vendor

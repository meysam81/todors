release:
  cargo build --release

build:
  cargo build --frozen

run:
  cargo run --frozen -- $(ARGS)

watch:
  cargo watch -s "just run"

format:
  cargo fmt
  cargo clippy

vendor:
  cargo vendor

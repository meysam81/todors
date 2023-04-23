release:
  cargo build --release

build:
  cargo build --frozen

check:
  clear
  cargo c --frozen

format:
  clear
  cargo fmt
  cargo clippy

vendor:
  cargo vendor

serve-http:
  cargo r --frozen -- serve http

serve-grpc:
  cargo r --frozen -- serve grpc

test:
  cargo t --frozen

clean:
  cargo clean

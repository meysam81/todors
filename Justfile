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

# https://github.com/fullstorydev/grpcurl/releases/tag/v1.8.7
ping-grpc:
  grpcurl -proto proto/healthcheck.proto -import-path proto/ -plaintext -d '{"message": "Hello Rust!"}' localhost:50051 healthcheck.HealthCheck/Check

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
  cargo clippy --fix --allow-staged --allow-dirty

vendor:
  cargo vendor

serve-http:
  cargo watch -s sh -- sh -c "clear; cargo run --frozen -- serve http"

serve-grpc:
  cargo r --frozen -- serve grpc

test:
  cargo t --frozen

clean:
  cargo clean

# https://github.com/fullstorydev/grpcurl/releases/tag/v1.8.7
grpc-client-ping:
  grpcurl -proto proto/healthcheck.proto -import-path proto/ -plaintext -d '{"message": "Hello Rust!"}' localhost:50051 healthcheck.HealthCheck/Check


grpc-client-list-todos:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext localhost:50051 todo.Todo/ListTodos

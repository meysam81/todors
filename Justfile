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
  cargo watch -s sh -- sh -c "clear; cargo run --frozen -- serve grpc"

test:
  cargo t --frozen

clean:
  cargo clean

# https://github.com/fullstorydev/grpcurl/releases/tag/v1.8.7
grpc-client-ping:
  grpcurl -proto proto/healthcheck.proto -import-path proto/ -plaintext -d '{"message": "Hello Rust!"}' localhost:50051 healthcheck.HealthCheck/Check


grpc-client-create-todo:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext -d '{"title": "Hello Rust!", "done": true}' localhost:50051 todo.Todo/Create

grpc-client-delete-todo:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext -d '{"id": "1"}' localhost:50051 todo.Todo/Delete

grpc-client-get-todo:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext -d '{"id": "1"}' localhost:50051 todo.Todo/Get

grpc-client-list-todos:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext localhost:50051 todo.Todo/List

grpc-client-update-todo:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext -d '{"id": "1", "title": "Hello Rust!", "done": true}' localhost:50051 todo.Todo/Update

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
  cargo watch -w src -w proto -s sh -- sh -c "clear; cargo run --frozen -- serve http"

serve-grpc:
  cargo watch -w src -w proto -s sh -- sh -c "clear; cargo run --frozen -- serve grpc"

test $CARGO_INCREMENTAL="0" $RUSTFLAGS="-Cinstrument-coverage" $LLVM_PROFILE_FILE="coverage/cargo-test-%p-%m.profraw":
  mkdir -p coverage
  cargo t --frozen
  grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/coverage.lcov

coverage-html:
  mkdir -p coverage/html
  grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/

serve-coverage-html:
  python -m http.server -d coverage/html 9000

clean:
  cargo clean

# https://github.com/fullstorydev/grpcurl/releases/tag/v1.8.7
grpc-client-ping:
  grpcurl -proto proto/healthcheck.proto -import-path proto/ -plaintext -d '{"message": "Hello Rust!"}' localhost:50051 healthcheck.HealthCheck/Check

grpc-client-create-todo:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext -d "{\"title\": \"Hello Rust! $$\", \"done\": true}" localhost:50051 todo.Todo/Create

grpc-client-delete-todo ID:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext -d '{"id": "{{ID}}"}' localhost:50051 todo.Todo/Delete

grpc-client-get-todo ID:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext -d '{"id": "{{ID}}"}' localhost:50051 todo.Todo/Get

grpc-client-list-todos *BODY:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext {{BODY}} localhost:50051 todo.Todo/List

grpc-client-update-todo ID:
  grpcurl -proto ./proto/todo.proto -import-path ./proto/ -plaintext -d "{\"id\": \"{{ID}}\", \"title\": \"Hello Rust! $$\"}" localhost:50051 todo.Todo/Update

sqlx-prepare:
  cargo sqlx prepare --database-url "sqlite://$HOME/.todors/db.sqlite"

maturin-develop:
  clear
  maturin develop --frozen

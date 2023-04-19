FROM curlimages/curl AS mold

USER root
WORKDIR /usr/src/mold

RUN curl -LO 'https://github.com/rui314/mold/releases/download/v1.11.0/mold-1.11.0-x86_64-linux.tar.gz' && \
    tar -xzf mold-1.11.0-x86_64-linux.tar.gz && \
    mv mold-1.11.0-x86_64-linux/bin/mold /usr/local/bin

FROM rust:1-alpine AS builder

WORKDIR /usr/src/todors
COPY --from=mold /usr/local/bin/mold /usr/local/bin/mold
COPY . .

RUN apk add --update clang build-base && \
    cargo install --path .

FROM scratch

COPY --from=builder /usr/local/cargo/bin/todors /usr/local/bin/todors

ENTRYPOINT ["/usr/local/bin/todors"]

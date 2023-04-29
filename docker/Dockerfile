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

RUN apk add --update clang build-base protoc protobuf-dev && \
    cargo install --path . && \
    addgroup -g 1000 todors && \
    adduser -D -u 1000 -h /data -G todors todors && \
    mkdir -p /data && \
    chown -R 1000:1000 /data

FROM scratch AS runner

ARG DATA_DIR=/data
ENV DATA_DIR=${DATA_DIR}
ENV DB_URL="${DATA_DIR}/todors.db"
VOLUME [ "${DATA_DIR}" ]

COPY --from=builder --chown=1000:1000 /usr/local/cargo/bin/todors /usr/local/bin/todors
COPY --from=builder --chown=1000:1000 /data /data

ONBUILD ENTRYPOINT ["/usr/local/bin/todors"]


FROM alpine:3 AS supervisor

RUN apk add --update supervisor

RUN addgroup -g 1000 todors && \
    adduser -D -u 1000 -h /data -G todors todors && \
    mkdir -p /data && \
    chown -R 1000:1000 /data

COPY --from=builder --chown=1000:1000 /usr/local/cargo/bin/todors /usr/local/bin/todors
COPY --chown=1000:1000 ./supervisord.conf /etc/supervisord.conf

USER 1000:1000
ENTRYPOINT ["supervisord"]


FROM runner AS http

USER 1000:1000
CMD [ "serve", "http", "-H", "0.0.0.0" ]

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
    cargo install --path .

FROM scratch AS runner


COPY --from=builder --chown=1000:1000 /usr/local/cargo/bin/todors /usr/local/bin/todors

ONBUILD ENTRYPOINT ["/usr/local/bin/todors"]


FROM curlimages/curl AS supervisor

USER root
WORKDIR /usr/src/supervisor

RUN apk add --update supervisor

FROM runner AS all

COPY --from=supervisor --chown=1000:1000 /usr/bin/supervisord /usr/local/bin/supervisord
COPY --chown=1000:1000 ./supervisord.conf /etc/supervisord.conf

USER 1000:1000
CMD ["supervisord"]


FROM runner AS http

USER 1000:1000
CMD [ "serve", "http", "-H", "0.0.0.0" ]

# Build
FROM rust:1.73.0-alpine as builder

RUN --mount=type=cache,target=/var/cache/apk \
  apk update \
  && apk add musl-dev \
  && rustup target add aarch64-unknown-linux-musl

WORKDIR /build

COPY queue_worker_svc/Cargo.toml Cargo.lock .
COPY shared/ /shared

RUN --mount=type=cache,target=/build/target \
  mkdir src \
  && echo "fn main() {}" > src/main.rs \
  && cargo build --release --target aarch64-unknown-linux-musl

COPY queue_worker_svc/src/ src/

RUN --mount=type=cache,target=/build/target \
  touch src/main.rs \
  && cargo build --release --target aarch64-unknown-linux-musl \
  && mkdir /output \
  && cp target/aarch64-unknown-linux-musl/release/queue_worker_svc* /output/

# Run
FROM alpine as runtime

WORKDIR /opt/queue_worker_svc

COPY --from=builder output/queue_worker_svc* .

ENTRYPOINT ["./queue_worker_svc" ]

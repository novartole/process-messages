# Build
FROM rust:1.73.0-alpine as builder

RUN --mount=type=cache,target=/var/cache/apk \
  apk update \
  && apk add musl-dev \
  && rustup target add aarch64-unknown-linux-musl

WORKDIR /build

COPY web_app/Cargo.toml Cargo.lock .
COPY shared/ /shared

RUN --mount=type=cache,target=/build/target \
  mkdir src \
  && echo "fn main() {}" > src/main.rs \
  && cargo build --release --target aarch64-unknown-linux-musl

COPY web_app/src/ src/

RUN --mount=type=cache,target=/build/target \
  touch src/main.rs \
  && cargo build --release --target aarch64-unknown-linux-musl \
  && mkdir /output \
  && cp target/aarch64-unknown-linux-musl/release/web_app* /output/

# Run
FROM alpine as runtime

WORKDIR /opt/web_app

COPY --from=builder output/web_app* .

ENTRYPOINT ["./web_app" ]

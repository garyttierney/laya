FROM docker.io/library/rust:1.81-bookworm AS builder

ARG TARGET="x86_64-unknown-linux-gnu"
ARG TARGET_CPU="native"

WORKDIR /build
COPY . /build

RUN --mount=type=cache,target=/var/cache/apt \
    apt-get update -y && apt-get install -y linux-headers-generic git libc6-dev
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/work/target \
    rustup install stable && RUSTFLAGS="-C target-feature=+crt-static -C target-cpu=${TARGET_CPU}" cargo build --target ${TARGET} --bin laya --release

FROM scratch
ARG TARGET
ARG TARGET_CPU

COPY --from=builder /build/target/${TARGET}/release/laya /bin/laya

ENTRYPOINT ["/bin/laya"]

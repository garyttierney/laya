ARG TARGET="x86_64-unknown-linux-gnu"
ARG TARGET_CPU="native"

FROM docker.io/library/fedora:latest AS builder
SHELL ["/bin/bash", "-c"]

ARG TARGET
ARG TARGET_CPU

RUN --mount=type=cache,target=/var/cache/dnf dnf install -y git clang binutils  libcxx-devel libstdc++-devel libstdc++ lld

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.82.0

RUN --mount=type=cache,target=/usr/local/rustup curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain "${RUST_VERSION}"

COPY . /work
WORKDIR /work

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    rustup install stable && \
    CXX=clang++ CC=clang LD=lld RUSTFLAGS="-C target-cpu=${TARGET_CPU}" cargo build --target ${TARGET} --bin laya --release

FROM scratch
ARG TARGET
ARG TARGET_CPU

COPY --from=builder /work/target/${TARGET}/release/laya /bin/laya

ENTRYPOINT ["/bin/laya"]

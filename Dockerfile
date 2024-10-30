ARG TARGET="x86_64-unknown-linux-gnu"
ARG TARGET_CPU="native"

FROM docker.io/library/fedora:latest AS builder
SHELL ["/bin/bash", "-c"]

ARG TARGET
ARG TARGET_CPU

RUN --mount=type=cache,target=/var/cache/dnf dnf install -y git clang binutils  libcxxabi-static libcxxabi-devel libcxx-devel libcxx-static libcxx libstdc++-devel libstdc++-static glibc-static lld

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.82.0


WORKDIR /work
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/rustup \
    --mount=type=bind,target=/work,rw \
    curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain "${RUST_VERSION}" && \
    rustup install stable && \
    rustup target add ${TARGET} && \
    TARGET_CPU=${TARGET_CPU} RUSTC=/work/share/rustc.wrap cargo build --locked --offline --target ${TARGET} --bin laya --release && \
    mkdir /out/ && mv /work/target/${TARGET}/release/laya /out/ && rm -Rf /work

FROM scratch
ARG TARGET
ARG TARGET_CPU

COPY --from=builder /out/laya /bin/laya

ENTRYPOINT ["/bin/laya"]

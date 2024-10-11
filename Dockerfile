FROM docker.io/library/rust:1.81-bookworm as builder

ARG TARGET="x86_64-unknown-linux-gnu"
ARG TARGET_CPU="native"

RUN apt-get update -y && apt-get install -y linux-headers-generic git libc6-dev
RUN rustup install stable

WORKDIR /build
COPY . /build

RUN RUSTFLAGS="-C target-feature=+crt-static -C target-cpu=${TARGET_CPU}" cargo build --target ${TARGET} --bin laya-server --release
RUN ls /build/target/release

FROM scratch
COPY --from=builder /build/target/${TARGET}/release/laya-server /bin/laya-server

ENTRYPOINT ["/bin/laya-server"]

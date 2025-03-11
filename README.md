# Laya: efficient standards-based image API 💅

Laya is a work-in-progress server implementation of the [IIIF Image API](https://iiif.io/api/image/3.0/) (version
3.0) with an emphasis on safety, performance, and spec-conformance.

### Users

Coming soon™

### Developers

#### Requirements:
- Linux or WSL2 (kernel version 5.8 or above), with `io_uring` enabled.
- Rust 1.80 (the project builds on stable, but nightly is used for `rustfmt`).

#### Building:

During development: ```cargo check``` (or ```cargo build``` to produce binaries).

For an optimized build: ```cargo build --release```.

#### Contributing:

Before committing:
```bash
cargo build
cargo clippy --workspace --all-targets --all-features -- -Dwarnings
cargo +nightly fmt --all
```

### Licensing

Laya is dual-licensed under [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT).

[openjp2-rs](https://github.com/Neopallium/openjp2/tree/master/openjp2-rs), a Rust port of OpenJPEG, is included as a subtree and is released under [2-Clause BSD](https://github.com/Neopallium/openjp2/blob/master/openjp2-rs/LICENSE).

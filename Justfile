default:
    @just --list | grep -v default

check: fmt clippy test build

fmt:
    cargo fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features

test:
    cargo test --workspace

build:
    cargo build --workspace --release

doc:
    cargo doc --workspace --no-deps

fmt-fix:
    cargo fmt --all

clippy-fix:
    cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged

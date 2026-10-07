default:
    @just --list

check-images:
    #!/usr/bin/env bash
    set -euo pipefail
    repository_root="$PWD"
    cd "$repository_root/cli"
    cargo build --quiet --locked --release
    printf '%s\0' \
        "$repository_root"/blockchains/*/logo.png \
        "$repository_root"/blockchains/*/assets/*/logo.png \
        | xargs -0 -n 500 target/release/cli check

download source mode="top" id="":
    cd cli && cargo run --locked -- --source {{ source }} --mode {{ mode }} --id "{{ id }}"

lint:
    cd cli && cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings

test:
    cd cli && cargo test --locked

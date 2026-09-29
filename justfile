wallet_core := "../wallet/core"

default:
    @just --list

check-images:
    #!/usr/bin/env bash
    set -euo pipefail
    repository_root="$PWD"
    cd "$repository_root/{{ wallet_core }}"
    cargo build --quiet --release --package img-downloader
    checker="$PWD/target/release/img-downloader"
    printf '%s\0' \
        "$repository_root"/blockchains/*/logo.png \
        "$repository_root"/blockchains/*/assets/*/logo.png \
        | xargs -0 -n 500 "$checker" check

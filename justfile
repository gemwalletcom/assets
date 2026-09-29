wallet_core := "../wallet/core"

default:
    @just --list

check-images base="HEAD^" head="HEAD":
    #!/usr/bin/env bash
    set -euo pipefail
    repository_root="$PWD"
    images=()
    while IFS= read -r -d '' image; do
        images+=("$repository_root/$image")
    done < <(./.github/scripts/changed-files.sh --null --diff-filter ACM "{{ base }}" "{{ head }}" \
        ':(glob)blockchains/*/logo.png' \
        ':(glob)blockchains/*/assets/*/logo.png')
    if (( ${#images[@]} == 0 )); then
        echo "No changed images"
        exit 0
    fi
    cd "$repository_root/{{ wallet_core }}"
    cargo run --quiet --release --package img-downloader -- check "${images[@]}"

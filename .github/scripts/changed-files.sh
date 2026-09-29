#!/usr/bin/env bash
set -euo pipefail

null_output=false
diff_filter=ACMRD
while [[ ${1:-} == --* ]]; do
  case $1 in
    --null) null_output=true; shift ;;
    --diff-filter) diff_filter=$2; shift 2 ;;
    *) echo "Unknown option: $1" >&2; exit 2 ;;
  esac
done

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 [--null] [--diff-filter FILTER] BASE HEAD [PATH ...]" >&2
  exit 2
fi

base=$1
head=$2
shift 2
options=(--name-only --no-renames --diff-filter="$diff_filter")
$null_output && options+=(-z)

git diff "${options[@]}" "$base" "$head" -- "$@"

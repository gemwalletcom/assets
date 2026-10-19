#!/bin/bash

# Generate assets.json for each chain containing list of token addresses with images
# Every chain gets one, [] when it has no token logos, so readers can tell an empty list from a failed fetch
# Optimized for speed - uses ls instead of find

for chain_dir in blockchains/*/; do
  assets_dir="${chain_dir}assets"
  output_file="${chain_dir}assets.json"

  # List directories directly, filter those with logo.png, sort for consistent order
  addresses=$(ls -1 "$assets_dir" 2>/dev/null | while read -r addr; do
    [ -f "$assets_dir/$addr/logo.png" ] && echo "$addr"
  done | LC_ALL=C sort)

  if [ -n "$addresses" ]; then
    # Convert to JSON array
    echo "$addresses" | jq -R . | jq -s . > "$output_file"
  else
    echo "[]" > "$output_file"
  fi
done

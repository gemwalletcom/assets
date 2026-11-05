#!/bin/bash
set -euo pipefail

ZONE_ID=$1
API_KEY=$2
BASE_URL=$3
FILES=$4
BATCH_SIZE=100 # Cloudflare's limit of URLs per purge request

if [ ! -s "$FILES" ]; then
  echo "No files to purge"
  exit 0
fi

echo "Purging $(wc -l < "$FILES" | tr -d ' ') files from cache:"
echo ""

BATCHES=$(mktemp)
trap 'rm -f "$BATCHES"' EXIT
# jq's _nwise is internal and gone in jq 1.8 (runners updated 2026-11-04), so slice the list by hand.
jq -Rnc --arg base "$BASE_URL" --argjson size "$BATCH_SIZE" \
  '[inputs | "\($base)/\(.)"] as $urls | [range(0; ($urls | length); $size) | {files: $urls[. : . + $size]}] | .[]' \
  "$FILES" > "$BATCHES"

FAILED=0
while read -r BODY; do
  jq -r '.files[] | "  → \(.)"' <<< "$BODY"
  if ! curl -fsS --retry 3 -o /dev/null -X POST \
    "https://api.cloudflare.com/client/v4/zones/$ZONE_ID/purge_cache" \
    -H "Authorization: Bearer $API_KEY" \
    -H "Content-Type: application/json" \
    --data "$BODY"; then
    echo "::error::Purge failed; these files stay cached until their TTL expires"
    FAILED=1
  fi
done < "$BATCHES"

echo ""
exit "$FAILED"

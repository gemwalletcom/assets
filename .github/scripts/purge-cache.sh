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
done < <(jq -Rnc --arg base "$BASE_URL" --argjson size "$BATCH_SIZE" '[inputs | "\($base)/\(.)"] | _nwise($size) | {files: .}' "$FILES")

echo ""
exit "$FAILED"

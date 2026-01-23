#!/bin/bash

ZONE_ID=$1
API_KEY=$2
BASE_URL=$3

URLS=""
for FILE in $(git diff --name-only HEAD~1 HEAD -- blockchains/ | head -30); do
  URLS="$URLS\"$BASE_URL/$FILE\","
done

if [ -n "$URLS" ]; then
  URLS=${URLS%,}
  curl -X POST "https://api.cloudflare.com/client/v4/zones/$ZONE_ID/purge_cache" \
    -H "Authorization: Bearer $API_KEY" \
    -H "Content-Type: application/json" \
    --data "{\"files\":[$URLS]}"
fi

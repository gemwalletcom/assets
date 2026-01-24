#!/bin/bash

ZONE_ID=$1
API_KEY=$2
BASE_URL=$3
FILES=$4

if [ ! -s "$FILES" ]; then
  echo "No files to purge"
  exit 0
fi

COUNT=$(wc -l < "$FILES" | tr -d ' ')
echo "Purging $COUNT files from cache:"
echo ""

while read -r FILE; do
  echo "  → $BASE_URL/$FILE"
  RESPONSE=$(curl -s -w "%{http_code}" -X POST \
    "https://api.cloudflare.com/client/v4/zones/$ZONE_ID/purge_cache" \
    -H "Authorization: Bearer $API_KEY" \
    -H "Content-Type: application/json" \
    --data "{\"files\":[\"$BASE_URL/$FILE\"]}")

  HTTP_CODE="${RESPONSE: -3}"
  if [ "$HTTP_CODE" != "200" ]; then
    echo "    ⚠ Failed (HTTP $HTTP_CODE)"
  fi
  sleep 0.5
done < "$FILES"

echo ""
echo "Done"

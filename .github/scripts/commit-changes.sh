#!/bin/bash
set -euo pipefail

MESSAGE=${1:-"Update assets"}

git config --global user.name 'github-actions[bot]'
git config --global user.email 'github-actions[bot]@users.noreply.github.com'
git add blockchains

if git diff --cached --quiet; then
  echo "Nothing to commit"
  echo "has_changes=false" >> "$GITHUB_OUTPUT"
  printf '## Asset changes\n\nNo asset changes.\n' >> "$GITHUB_STEP_SUMMARY"
else
  IMAGE_COUNT=$(git diff --cached --name-only -- ':(glob)blockchains/**/*.png' | wc -l | tr -d ' ')
  if [ "$IMAGE_COUNT" -gt 0 ]; then
    MESSAGE="$MESSAGE ($IMAGE_COUNT images)"
  fi

  echo "Files to commit:"
  git diff --cached --name-only
  echo ""
  git commit -m "$MESSAGE"
  git push
  echo "has_changes=true" >> "$GITHUB_OUTPUT"
  COMMIT=$(git rev-parse HEAD)
  {
    printf '## Asset changes\n\n%s images changed.\n\n' "$IMAGE_COUNT"
    printf '[View commit](%s/%s/commit/%s)\n\n' "$GITHUB_SERVER_URL" "$GITHUB_REPOSITORY" "$COMMIT"
    printf 'Changed images (up to 200): A = added, M = updated, D = deleted.\n\n```text\n'
    git diff-tree --root --no-commit-id --no-renames --name-status -r "$COMMIT" -- ':(glob)blockchains/**/*.png' | sed -n '1,200p'
    printf '```\n'
  } >> "$GITHUB_STEP_SUMMARY"
fi

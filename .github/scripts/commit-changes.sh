#!/bin/bash
set -euo pipefail

MESSAGE=${1:-"Update assets"}

# A fetcher that checked out main before another fetcher pushed has its push rejected, and its
# logos then wait for the next slot. Rebase onto the moved main and push again, up to 3 tries.
# -X ours keeps the remote's copy where both fetchers added the same logo, and a chain's
# assets.json is a single list both rewrite, so every list is rebuilt from the images that
# ended up in the tree before the retry goes out.
push_with_rebase() {
  local tries=3 try ahead
  for ((try = 1; try <= tries; try++)); do
    if git push; then
      return 0
    fi
    if [ "$try" -eq "$tries" ]; then
      echo "::error::Push rejected $tries times; these logos wait for the next fetcher slot"
      return 1
    fi
    echo "Push rejected: main moved since checkout. Rebasing onto it, push try $((try + 1)) of $tries."
    if ! git pull --rebase --strategy-option=ours origin main; then
      echo "::error::Could not rebase onto the moved main; nothing was pushed"
      git rebase --abort 2>/dev/null || true
      return 1
    fi
    ahead=$(git rev-list origin/main..HEAD --count)
    if [ "$ahead" -eq 0 ]; then
      echo "Our commit turned out to be already on main; nothing left to push"
      return 0
    fi
    "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/generate-assets-list.sh"
    git add blockchains
    git diff --cached --quiet || git commit --amend --no-edit
  done
}

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
  push_with_rebase
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

#!/bin/bash

MESSAGE=${1:-"Update assets"}

git config --global user.name 'github-actions[bot]'
git config --global user.email 'github-actions[bot]@users.noreply.github.com'
git add . ':!core'

if git diff --cached --quiet; then
  echo "Nothing to commit"
  echo "has_changes=false" >> $GITHUB_OUTPUT
else
  echo "Files to commit:"
  git diff --cached --name-only
  echo ""
  git commit -m "$MESSAGE"
  git push
  echo "has_changes=true" >> $GITHUB_OUTPUT
fi

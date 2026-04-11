#!/bin/bash

set -e 

MESSAGE="$1"

if [ -z "$MESSAGE" ]; then
    echo "Usage : ./deploy.sh \"your commit message\""
    exit 1 
fi 

echo "Running cargo check"
cargo check

echo "Committing to dev..."
git add . 
git commit -m "$MESSAGE"

echo "Merging dev into main and pushing ..."
git checkout main
git merge dev --no-edit
git push origin main

git checkout dev

echo "Deployed."
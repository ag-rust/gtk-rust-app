#!/bin/bash

# Release this app
# variables:
# - DRY=1: Will run all steps but not commit anything
# - REMOTE=1: Will not set an additional git remote (which is required when executed on gitlab)

if [[ "$REMOTE" == 1 ]]; then
	echo set remote origin and git user to "gitlab ci"
    git remote set-url origin https://${GITLAB_USERNAME}:${GITLAB_TOKEN}@${CI_SERVER_HOST}/${CI_PROJECT_PATH}.git
    git config --global user.email "gitlab ci"
    git config --global user.name "gitlab ci"
fi

TAG_VERSION_OLD=$(grep version Cargo.toml | head -1 | sed 's/version = "//g' | sed 's/"//g')
cog bump --auto
TAG_VERSION=$(grep version Cargo.toml | head -1 | sed 's/version = "//g' | sed 's/"//g')
echo $TAG_VERSION_OLD - $TAG_VERSION

if [[ "$TAG_VERSION_OLD" == "$TAG_VERSION" ]]; then
    echo bump patch
    cog bump --patch
    TAG_VERSION=$(grep version Cargo.toml | head -1 | sed 's/version = "//g' | sed 's/"//g')
fi

if [[ "$DRY" == "" ]]; then
    git push origin HEAD:$CI_COMMIT_BRANCH
    git tag -d v$TAG_VERSION
    git tag v$TAG_VERSION
    git push origin v$TAG_VERSION
else
    echo Dry running:
    echo git push origin HEAD:$CI_COMMIT_BRANCH
    echo git tag -d v$TAG_VERSION
    echo git tag v$TAG_VERSION
    echo git push origin v$TAG_VERSION
    cargo publish --token $CRATES_IO_TOKEN
fi

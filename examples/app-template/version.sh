#!/bin/bash

# Pre checks
# -------------------------------------------------------------------------------------------

if ! git diff-index --quiet HEAD --; then
    echo "You have local file changes. Please commit all changes before version bumping."
    exit 1
fi

# Setup repo, version, tag prefix, determine next version
# -------------------------------------------------------------------------------------------
mkdir -p target
REPO=$GITHUB_SERVER_URL/$GITHUB_REPOSITORY
echo repo url: $REPO

if [[ "$TAG_PREFIX" == "" ]]; then
    TAG_PREFIX=v
fi
echo tag prefix: $TAG_PREFIX

VERSION=$(grep version Cargo.toml | head -1 | sed 's/version = "//g' | sed 's/"//g')
echo old version: $VERSION
git log "origin/${GITHUB_REF##*/}" $TAG_PREFIX$VERSION..HEAD --pretty=format:"- %s [%h]($REPO/commit/%H)" > ./target/commits

# check for lines starting with [- .*!:] -> breaking changes
if grep -q -- "- .*"'!': ./target/commits; then
    echo "major release"
    NEXT_VERSION=$(echo $VERSION | awk -F. '{print $1+1"."0"."0}')
# check for lines starting with [- feat.*:] -> breaking changes
elif grep -q -- "- feat.*:" ./target/commits; then
    echo "minor release"
    NEXT_VERSION=$(echo $VERSION | awk -F. '{print $1"."$2+1"."0}')
elif grep -q -- "- fix.*:" ./target/commits; then
    echo "bugfix release"
    NEXT_VERSION=$(echo $VERSION | awk -F. '{print $1"."$2"."$3+1}')
else
    echo "bump only"
    BUMP_ONLY=true
    NEXT_VERSION=$(echo $VERSION | awk -F. '{print $1"."$2"."$3+1}')
fi
echo release version: $NEXT_VERSION

# Changelog generation
# -------------------------------------------------------------------------------------------

echo add commits to changelog
OLD_LOG_START=$(grep -n "## Version" CHANGELOG.md | cut -f1 -d: | head -1)
cat CHANGELOG.md | head "-`expr $OLD_LOG_START - 1`" > target/TEMP_CHANGELOG.md
echo -e "## Version $NEXT_VERSION" >> target/TEMP_CHANGELOG.md
if grep -q -- "- .*"'!': ./target/commits; then
    echo -e "\n### Breaking changes\n" >> target/TEMP_CHANGELOG.md
    grep -- "- .*"'!': ./target/commits >> target/TEMP_CHANGELOG.md || true
    echo -e "\n" >> target/TEMP_CHANGELOG.md
fi
if grep -q -- "- feat.*:" ./target/commits; then
    echo -e "\n### Features\n" >> target/TEMP_CHANGELOG.md
    grep -- "- feat.*:" ./target/commits >> target/TEMP_CHANGELOG.md || true
fi
if grep -q -- "- fix.*:" ./target/commits; then
    echo -e "\n### Bugfixes\n" >> target/TEMP_CHANGELOG.md
    grep -- "- fix.*:" ./target/commits >> target/TEMP_CHANGELOG.md || true
fi
if [[ "$BUMP_ONLY" == "true" ]]; then
    echo -e "\nVersion bump\n" >> target/TEMP_CHANGELOG.md
fi

# remove all feat.*: fix.*: .*!: prefixes
sed -i "s/- .*!: /- /g" target/TEMP_CHANGELOG.md
sed -i "s/- feat: /- /g" target/TEMP_CHANGELOG.md
sed -i "s/- feat\(.*\): /- /g" target/TEMP_CHANGELOG.md
sed -i "s/- fix: /- /g" target/TEMP_CHANGELOG.md
sed -i "s/- fix\(.*\): /- /g" target/TEMP_CHANGELOG.md

echo -e "\n" >> target/TEMP_CHANGELOG.md
cat CHANGELOG.md | tail "-n+$OLD_LOG_START" >> target/TEMP_CHANGELOG.md
mv target/TEMP_CHANGELOG.md CHANGELOG.md
echo modified changelog:
cat CHANGELOG.md

# Increment version and push
# -------------------------------------------------------------------------------------------

echo "Increment version: $VERSION > $NEXT_VERSION"
sed -i "0,/version = \"$VERSION\"/s//version = \"$NEXT_VERSION\"/" Cargo.toml
sed -i "0,/version = \"$VERSION\"/s//version = \"$NEXT_VERSION\"/" runtime-gateway-cli/Cargo.toml
sed -i "0,/version = \"$VERSION\"/s//version = \"$NEXT_VERSION\"/" runtime-gateway-lib/Cargo.toml
sed -i "0,/version = \"$VERSION\"/s//version = \"$NEXT_VERSION\"/" runtime-gateway-windows/Cargo.toml
sed -i "0,/define MyAppVersion \"$VERSION\"/s//define MyAppVersion \"$NEXT_VERSION\"/" runtime-gateway-windows-installer/installer.iss

echo -e "$NEXT_VERSION" >> versions

echo commit and push changes

git add -A
git status
git commit -m "chore: release $TAG_PREFIX$NEXT_VERSION"
git tag $TAG_PREFIX$NEXT_VERSION
git push --atomic origin main $TAG_PREFIX$NEXT_VERSION

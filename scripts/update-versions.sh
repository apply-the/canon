#!/usr/bin/env bash
set -e

# Get workspace version from Cargo.toml
VERSION=$(grep -m1 '^version =' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')

if [ -z "$VERSION" ]; then
    echo "Error: Could not find version in Cargo.toml"
    exit 1
fi

echo "Updating documentation references to version $VERSION..."

find . -type f \( -name "*.md" -o -name "*.rs" -o -name "*.toml" \) \
    -not -path "*/target/*" \
    -not -path "*/node_modules/*" \
    -not -path "*/.git/*" \
    -not -path "*/.vitepress/cache/*" \
    -not -path "*/.vitepress/dist/*" \
    -exec perl -pi -e "s|blob/[0-9]+\.[0-9]+\.[0-9]+|blob/$VERSION|g; \
                       s|tree/[0-9]+\.[0-9]+\.[0-9]+|tree/$VERSION|g; \
                       s|raw/[0-9]+\.[0-9]+\.[0-9]+|raw/$VERSION|g; \
                       s|Boundline [0-9]+\.[0-9]+\.[0-9]+|Boundline $VERSION|g; \
                       s|Canon [0-9]+\.[0-9]+\.[0-9]+|Canon $VERSION|g" {} +

echo "Version update complete."

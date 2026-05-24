#!/usr/bin/env bash
set -euo pipefail

# Usage: ./release.sh [target-triple]
#   If target-triple is omitted, builds for the current host target.
#
# Examples:
#   ./release.sh                                    # native build
#   ./release.sh x86_64-unknown-linux-gnu
#   ./release.sh aarch64-apple-darwin
#
# Cross-compilation requires the Rust target installed:
#   rustup target add aarch64-apple-darwin

TARGET="${1:-}"
BINARY="proj-core"

if [[ -z "$TARGET" ]]; then
    TARGET="$(rustc -vV | grep host | awk '{print $2}')"
    echo "No target specified, using host: $TARGET"
fi

echo "Building $BINARY for $TARGET ..."

cargo build --release --target "$TARGET"

ARCHIVE="$BINARY-$TARGET.tar.gz"
DIR="target/$TARGET/release"

tar czf "$ARCHIVE" -C "$DIR" "$BINARY"

echo ""
echo "Created: $(pwd)/$ARCHIVE"
ls -lh "$ARCHIVE"

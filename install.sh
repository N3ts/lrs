#!/usr/bin/env bash

set -e

BINARY_NAME="lrs"
BUILD_PATH="./target/release/$BINARY_NAME"
INSTALL_PATH="/usr/local/bin/$BINARY_NAME"

# ==============================================================================

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: Please install 'cargo' - see requirements in 'README.md'" >&2
    exit 1
fi

echo "Building..."
sleep 0.25
if ! cargo build --release; then
    echo "Error: failed to build '$BINARY_NAME'" >&2
    exit 2
fi
echo "Build successful!"
echo

if [ ! -f "$BUILD_PATH" ]; then
    echo "Error: Binary not found in '$BUILD_PATH'" >&2
    exit 3
fi

echo "Moving binary to '$INSTALL_PATH'..."
sleep 0.25

if ! sudo cp "$BUILD_PATH" "$INSTALL_PATH"; then
    echo "Error: Could not move binary '$BUILD_PATH' to '$INSTALL_PATH'" >&2
    exit 4
fi

echo "Done!"
#!/bin/bash
set -e

# suntheme installer script
# Usage: curl -fsSL https://raw.githubusercontent.com/lukalavric/suntheme/main/install.sh | bash

REPO="lukalavric/suntheme"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Darwin)
        case "$ARCH" in
            x86_64) TARGET="x86_64-apple-darwin" ;;
            arm64)  TARGET="aarch64-apple-darwin" ;;
            *)      echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    Linux)
        case "$ARCH" in
            x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
            *)      echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

# Get latest release tag
echo "Fetching latest release..."
LATEST=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)

if [ -z "$LATEST" ]; then
    echo "Failed to fetch latest release"
    exit 1
fi

echo "Latest version: $LATEST"

# Download URL
URL="https://github.com/$REPO/releases/download/$LATEST/suntheme-$TARGET.tar.gz"

# Create temp directory
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# Download and extract
echo "Downloading suntheme for $TARGET..."
curl -fsSL "$URL" | tar -xz -C "$TMP_DIR"

# Install
echo "Installing to $INSTALL_DIR..."
if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_DIR/suntheme" "$INSTALL_DIR/"
else
    sudo mv "$TMP_DIR/suntheme" "$INSTALL_DIR/"
fi

chmod +x "$INSTALL_DIR/suntheme"

echo ""
echo "suntheme installed successfully!"
echo ""
echo "Run 'suntheme init' to get started."

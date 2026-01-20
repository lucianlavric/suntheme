#!/bin/bash
set -e

REPO="lucianlavric/suntheme"
BINARY="suntheme"
INSTALL_DIR="/usr/local/bin"

# Detect architecture
ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

case "$OS" in
    linux)
        case "$ARCH" in
            x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    darwin)
        case "$ARCH" in
            x86_64) TARGET="x86_64-apple-darwin" ;;
            arm64) TARGET="aarch64-apple-darwin" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

ASSET="suntheme-${TARGET}.tar.gz"

# Get latest release URL
LATEST=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep "browser_download_url.*${ASSET}" | cut -d '"' -f 4)

if [ -z "$LATEST" ]; then
    echo "Could not find release for $TARGET"
    exit 1
fi

echo "Downloading $BINARY for $TARGET..."
TMPDIR=$(mktemp -d)
curl -sL "$LATEST" -o "$TMPDIR/$ASSET"

echo "Installing to $INSTALL_DIR..."
tar -xzf "$TMPDIR/$ASSET" -C "$TMPDIR"
sudo mv "$TMPDIR/$BINARY" "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/$BINARY"

rm -rf "$TMPDIR"

echo "Installed $BINARY to $INSTALL_DIR/$BINARY"

# Verify installation
if command -v suntheme &> /dev/null; then
    echo "Run 'suntheme init' to get started."
else
    echo ""
    echo "Note: $INSTALL_DIR is not in your PATH."
    echo "Add it to your shell config:"
    echo ""
    echo "  # For bash (~/.bashrc):"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
    echo ""
    echo "  # For zsh (~/.zshrc):"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
    echo ""
    echo "Then restart your terminal or run: source ~/.bashrc (or ~/.zshrc)"
fi

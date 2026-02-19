#!/bin/sh
set -e

REPO="dilettante-trading/polyoxide"
BINARY="polyoxide"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS
case "$(uname -s)" in
    Linux*)  OS="unknown-linux-gnu" ;;
    Darwin*) OS="apple-darwin" ;;
    *)       echo "Unsupported OS: $(uname -s)"; exit 1 ;;
esac

# Detect architecture
case "$(uname -m)" in
    x86_64)  ARCH="x86_64" ;;
    aarch64) ARCH="aarch64" ;;
    arm64)   ARCH="aarch64" ;;
    *)       echo "Unsupported architecture: $(uname -m)"; exit 1 ;;
esac

TARGET="${ARCH}-${OS}"

# Get latest version
echo "Fetching latest version..."
VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | grep 'cli-v' | head -1 | cut -d'"' -f4)

if [ -z "$VERSION" ]; then
    echo "Could not determine latest version"
    exit 1
fi

echo "Latest version: $VERSION"

# Download
URL="https://github.com/$REPO/releases/download/$VERSION/polyoxide-cli-${TARGET}.tar.gz"
echo "Downloading $URL..."

TMP_DIR=$(mktemp -d)
curl -fsSL "$URL" | tar xz -C "$TMP_DIR"

# Install
mkdir -p "$INSTALL_DIR"
mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/$BINARY"
rm -rf "$TMP_DIR"

echo ""
echo "Installed $BINARY to $INSTALL_DIR/$BINARY"

# Check if in PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo ""
    echo "Add $INSTALL_DIR to your PATH:"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi

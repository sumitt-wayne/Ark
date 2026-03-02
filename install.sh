#!/bin/bash

set -e

REPO="sumitt-wayne/Ark"
BINARY="ark"

echo "Installing Ark..."

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$ARCH" = "x86_64" ]; then
    ARCH="x86_64"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    ARCH="aarch64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

if [ "$OS" = "linux" ]; then
    TARGET="${ARCH}-unknown-linux-gnu"
elif [ "$OS" = "darwin" ]; then
    TARGET="${ARCH}-apple-darwin"
else
    echo "Unsupported OS: $OS"
    echo "Windows users: download ark.exe from https://github.com/$REPO/releases"
    exit 1
fi

LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)

if [ -z "$LATEST" ]; then
    echo "Error: Could not fetch latest version."
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$LATEST/ark-${TARGET}"

echo "Downloading Ark $LATEST..."
curl -fsSL "$URL" -o /tmp/ark

chmod +x /tmp/ark
sudo mv /tmp/ark /usr/local/bin/ark

echo ""
echo "✓ Ark installed successfully!"
echo "  Version: $(ark --version)"
echo ""
echo "Get started:"
echo "  ark start"

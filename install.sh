#!/bin/bash

set -e

# colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # no color

echo -e "${GREEN}🌱 seedenv Installer${NC}"
echo ""

# detect os and architecture
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Darwin)
        OS_NAME="macos"
        case "$ARCH" in
            x86_64)
                BINARY_ARCH="x86_64"
                ;;
            arm64)
                BINARY_ARCH="aarch64"
                ;;
            *)
                echo -e "${RED}❌ unsupported architecture: $ARCH${NC}"
                exit 1
                ;;
        esac
        ;;
    Linux)
        OS_NAME="linux"
        case "$ARCH" in
            x86_64)
                BINARY_ARCH="x86_64"
                ;;
            *)
                echo -e "${RED}❌ unsupported architecture: $ARCH${NC}"
                exit 1
                ;;
        esac
        ;;
    MINGW64_NT*|MSYS_NT*)
        OS_NAME="windows"
        BINARY_ARCH="x86_64"
        ;;
    *)
        echo -e "${RED}❌ unsupported os: $OS${NC}"
        exit 1
        ;;
esac

echo -e "${YELLOW}detected: $OS_NAME $BINARY_ARCH${NC}"

# Get latest version from GitHub API
VERSION=$(curl -s https://api.github.com/repos/rfqma/seedenv/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$VERSION" ]; then
    echo -e "${RED}❌ Failed to get latest version${NC}"
    exit 1
fi

REPO="rfqma/seedenv"

case "$OS_NAME-$BINARY_ARCH" in
    macos-x86_64)
        BINARY_NAME="seedenv-macos-x86_64"
        ;;
    macos-aarch64)
        BINARY_NAME="seedenv-macos-aarch64"
        ;;
    linux-x86_64)
        BINARY_NAME="seedenv-linux-x86_64"
        ;;
    windows-x86_64)
        BINARY_NAME="seedenv-windows-x86_64.exe"
        ;;
    *)
        echo -e "${RED}❌ no pre-built binary for $OS_NAME-$BINARY_ARCH${NC}"
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY_NAME"

if [ "$OS_NAME" = "windows" ]; then
    INSTALL_DIR="${PROGRAMFILES:-C:/Program Files}/seedenv"
    BINARY_PATH="$INSTALL_DIR/seedenv.exe"
else
    INSTALL_DIR="/usr/local/bin"
    BINARY_PATH="$INSTALL_DIR/seedenv"
fi

echo -e "${YELLOW}downloading from: $DOWNLOAD_URL${NC}"

# temp directory
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# download binary
if command -v curl &> /dev/null; then
    curl -sSL -o "$TEMP_DIR/seedenv-binary" "$DOWNLOAD_URL"
elif command -v wget &> /dev/null; then
    wget -q -O "$TEMP_DIR/seedenv-binary" "$DOWNLOAD_URL"
else
    echo -e "${RED}❌ neither curl nor wget found. please install one of them.${NC}"
    exit 1
fi

# check if download succeeded
if [ ! -f "$TEMP_DIR/seedenv-binary" ] || [ ! -s "$TEMP_DIR/seedenv-binary" ]; then
    echo -e "${RED}❌ failed to download seedenv${NC}"
    exit 1
fi

# make binary executable
chmod +x "$TEMP_DIR/seedenv-binary"

# install binary
if [ "$OS_NAME" = "windows" ]; then
    mkdir -p "$INSTALL_DIR"
    cp "$TEMP_DIR/seedenv-binary" "$BINARY_PATH"
    echo -e "${GREEN}✅ installed to: $BINARY_PATH${NC}"
    echo -e "${YELLOW}add $INSTALL_DIR to your PATH if not already done${NC}"
else
    if [ -w "$INSTALL_DIR" ]; then
        cp "$TEMP_DIR/seedenv-binary" "$BINARY_PATH"
    else
        echo -e "${YELLOW}need sudo to install to $INSTALL_DIR${NC}"
        sudo cp "$TEMP_DIR/seedenv-binary" "$BINARY_PATH"
        sudo chmod +x "$BINARY_PATH"
    fi
    echo -e "${GREEN}✅ installed to: $BINARY_PATH${NC}"
fi

# verify installation
if command -v seedenv &> /dev/null; then
    echo -e "${GREEN}✅ seedenv is ready to use!${NC}"
    echo ""
    echo -e "${GREEN}run seedenv:${NC}"
    echo "  seedenv"
else
    echo -e "${YELLOW}⚠️  seedenv might not be in PATH. try:${NC}"
    echo "  $BINARY_PATH"
fi

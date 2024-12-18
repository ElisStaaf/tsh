#!/bin/bash

set -e

echo "Installing TSH..."

REPO_URL="https://github.com/ElisStaaf/tsh"
INSTALL_DIR="/tmp/tsh_install"

git clone "$REPO_URL" "$INSTALL_DIR"

cd "$INSTALL_DIR"

echo "Compiling the project..."
cargo build --release

BIN_PATH="/usr/local/bin/tsh"
echo "Installing the executable to $BIN_PATH..."
sudo mv target/release/tsh  "$BIN_PATH"

echo "Removing temporary files..."
rm -rf "$INSTALL_DIR"

echo "TSH has been installed correctly!"
echo "Execute \"TSH\" to run."

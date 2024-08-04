#!/bin/bash

if ! command -v cargo &> /dev/null; then
    echo "Cargo (Rust) not found. Please install Rust before running this script."
    exit 1
fi

set -e  # Exit immediately if a command exits with a non-zero status

# Define the binary name and the target directory
BINARY_NAME="oxidize"
TARGET_DIR="/usr/local/bin"

# Build the project
echo "Building the Rust project..."
cargo build --release

# Install the binary
echo "Installing the binary to $TARGET_DIR/$BINARY_NAME..."
sudo mv target/release/$BINARY_NAME $TARGET_DIR/

# Verify installation
echo "Verifying installation..."
if command -v $BINARY_NAME &> /dev/null; then
    echo "$BINARY_NAME successfully installed."
    $BINARY_NAME --help
else
    echo "Installation failed. $BINARY_NAME not found in $TARGET_DIR."
    exit 1
fi

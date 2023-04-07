#!/bin/bash

set -e

echo "Building Awqat..."
echo "Running ($ cargo build --release)..."
cargo build --release

if [ $? -ne 0 ]; then
  echo "Failed to build Awqat."
  exit 1
fi

if command -v upx &> /dev/null; then
  echo "Compressing Awqat's Executable..."
  echo "Running ($ upx --best --lzma target/release/awqat)..."
  upx --best --lzma target/release/awqat
fi

echo "Copying Awqat to the /usr/local/bin/ directory..."
echo "Running (# cp ./target/release/awqat /usr/local/bin/awqat)..."
sudo cp ./target/release/awqat /usr/local/bin/awqat

if [ $? -ne 0 ]; then
  echo "Failed to copy Awqat to /usr/local/bin/."
  exit 1
fi

echo "Alhamdulillah! Exiting... "

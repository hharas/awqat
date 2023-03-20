#!/bin/bash

set -e

echo "Building Awqat..."
echo "Running ($ cargo build --release)..."
cargo build --release

if [ $? -ne 0 ]; then
  echo "Failed to build Awqat."
  exit 1
fi

echo "Copying Awqat to the /usr/bin/ directory..."
echo "Running (# cp ./target/release/awqat /usr/bin/awqat)..."
sudo cp ./target/release/awqat /usr/bin/awqat

if [ $? -ne 0 ]; then
  echo "Failed to copy Awqat to /usr/bin/."
  exit 1
fi

echo "Alhamdulillah! Exiting... "

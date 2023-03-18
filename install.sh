#!/bin/bash

set -e

echo "Building Awqat..."
cargo build --quiet --release

if [ $? -ne 0 ]; then
  echo "Failed to build Awqat."
  exit 1
fi

echo "Copying Awqat to the /usr/bin/ directory..."
sudo cp ./target/release/awqat /usr/bin/awqat

if [ $? -ne 0 ]; then
  echo "Failed to copy Awqat to /usr/bin/."
  exit 1
fi

echo "Alhamdulillah! Exiting... "

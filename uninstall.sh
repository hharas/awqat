#!/bin/bash

set -e

echo "Uninstalling awqat..."
rm /usr/local/bin/awqat

if [ $? -ne 0 ]; then
  echo "Failed to uninstall awqat from /usr/local/bin/."
  exit 1
fi

echo "Alhamdulillah! Exiting..."

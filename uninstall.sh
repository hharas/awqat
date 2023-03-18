#!/bin/bash

set -e

echo "Uninstalling awqat..."
sudo rm /usr/bin/awqat

if [ $? -ne 0 ]; then
  echo "Failed to uninstall awqat from /usr/bin/."
  exit 1
fi

echo "Alhamdulillah! Exiting..."

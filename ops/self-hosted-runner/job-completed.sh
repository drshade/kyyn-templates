#!/usr/bin/env bash
set -euo pipefail

if [[ -d /runner/_work ]]; then
  find /runner/_work -mindepth 1 -maxdepth 1 -exec rm -rf -- {} +
fi

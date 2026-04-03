#!/bin/bash
set -e

# Validate Termux
if [ -z "$PREFIX" ]; then
    echo "Requires Termux environment."
    exit 1
fi

echo "[Info] Ensuring dependencies (curl, jq, wget, bash)..."
pkg install -y curl jq wget bash

# Calculate RAM using Android's free/dumpsys equivalent
# Bionic libc free outputs in KB usually
TOTAL_RAM_KB=$(free | awk '/Mem:/ {print $2}')
TOTAL_RAM_GB=$(awk "BEGIN {print $TOTAL_RAM_KB/1024/1024}")

echo "[Info] Device Total RAM: ~${TOTAL_RAM_GB} GB"

if awk "BEGIN {exit !($TOTAL_RAM_GB < 4.0)}"; then
    echo -e "\033[33m[Cảnh Báo] Thiết bị của bạn có ít hơn 4GB RAM.\033[0m"
    echo "Các model nội bộ (TinyLLM) sẽ chạy khá chậm hoặc bị văng (Crash)."
    echo "Nên sử dụng API Keys từ OpenRouter hoặc OpenAI."
fi

# Make our bin scripts accessible down the line
mkdir -p "$PREFIX/bin"

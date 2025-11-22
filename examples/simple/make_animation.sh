#!/usr/bin/env bash

set -euo pipefail

OUTPUT_DIR="out"
VIDEO_FILE="output.mp4"
FRAMERATE=60

cleanup() {
    if [[ -d "$OUTPUT_DIR" ]]; then
        echo "[DEBUG] Cleaning up $OUTPUT_DIR..."
        rm -rf "$OUTPUT_DIR"
    fi
}

trap cleanup EXIT

echo "[DEBUG] Creating output directory..."
mkdir -p "$OUTPUT_DIR"

echo "[DEBUG] Running application..."
cargo run --release

echo "[DEBUG] Checking for output files..."
if ! ls "$OUTPUT_DIR"/output-*.ppm 1> /dev/null 2>&1; then
    echo "[DEBUG] Error: No PPM files found in $OUTPUT_DIR"
    exit 1
fi

if ! command -v ffmpeg &> /dev/null; then
    echo "[DEBUG] Error: ffmpeg is not installed"
    exit 1
fi

echo "[DEBUG] Generating video..."
ffmpeg -y -i "$OUTPUT_DIR/output-%02d.ppm" -r "$FRAMERATE" "$VIDEO_FILE"

echo "[DEBUG] Done! Video created: $VIDEO_FILE"

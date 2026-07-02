#!/usr/bin/env bash
set -euo pipefail

SAMPLES_DIR="$(dirname "$0")/../samples"
mkdir -p "$SAMPLES_DIR"

BASE_URL="https://raw.githubusercontent.com/fluid-music/open-drums/main/tr-909/TR909all"

fetch() {
    local dest="$1"
    local src="$2"
    if [ ! -f "$SAMPLES_DIR/$dest" ]; then
        echo "Downloading $dest ..."
        curl -sSL "$BASE_URL/$src" -o "$SAMPLES_DIR/$dest"
    else
        echo "$dest already exists, skipping"
    fi
}

fetch "kick.wav"    "BT0A0D0.WAV"
fetch "snare.wav"   "ST0T0S0.WAV"
fetch "low_tom.wav" "LT0D0.WAV"
fetch "mid_tom.wav" "MT0D0.WAV"
fetch "hi_tom.wav"  "HT0D0.WAV"
fetch "clap.wav"    "HANDCLP1.WAV"
fetch "hihat.wav"   "HHCD0.WAV"
fetch "cymbal.wav"  "CSHD0.WAV"

echo "Done! All samples in $SAMPLES_DIR"

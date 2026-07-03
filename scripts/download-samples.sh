#!/usr/bin/env bash
set -euo pipefail

OVERWRITE=0
MACHINE=""
for arg in "$@"; do
  case "$arg" in
    --overwrite) OVERWRITE=1 ;;
    *) MACHINE="$arg" ;;
  esac
done
[ -z "$MACHINE" ] && MACHINE="tr909"
MACHINE="${MACHINE#tr-}"
MACHINE="tr-${MACHINE#tr}"

SAMPLES_DIR="$(dirname "$0")/../samples"
mkdir -p "$SAMPLES_DIR"

BASE_URL="https://raw.githubusercontent.com/fluid-music/open-drums/main"

fetch() {
    local dest="$1"
    local src="$2"
    if [ "$OVERWRITE" -eq 1 ] || [ ! -f "$SAMPLES_DIR/$dest" ]; then
        echo "Downloading $dest ..."
        curl -sSL "$BASE_URL/$MACHINE/$SUBDIR/$src" -o "$SAMPLES_DIR/$dest"
    else
        echo "$dest already exists, skipping"
    fi
}

case "$MACHINE" in
  tr-909)
    SUBDIR="TR909all"
    fetch "kick.wav"    "BT0A0D0.WAV"
    fetch "snare.wav"   "ST0T0S0.WAV"
    fetch "low_tom.wav" "LT0D0.WAV"
    fetch "mid_tom.wav" "MT0D0.WAV"
    fetch "hi_tom.wav"  "HT0D0.WAV"
    fetch "clap.wav"    "HANDCLP1.WAV"
    fetch "hihat.wav"   "HHCD0.WAV"
    fetch "cymbal.wav"  "CSHD0.WAV"
    ;;
  tr-808)
    SUBDIR="TR808WAV"
    fetch "kick.wav"    "BD/BD0000.WAV"
    fetch "snare.wav"   "SD/SD0000.WAV"
    fetch "low_tom.wav" "LT/LT00.WAV"
    fetch "mid_tom.wav" "MT/MT00.WAV"
    fetch "hi_tom.wav"  "HT/HT00.WAV"
    fetch "clap.wav"    "CP/CP.WAV"
    fetch "hihat.wav"   "CH/CH.WAV"
    fetch "cymbal.wav"  "CY/CY0000.WAV"
    ;;
  tr-707)
    SUBDIR="TR707WAV"
    fetch "kick.wav"    "BassDrum1.wav"
    fetch "snare.wav"   "Snare1.wav"
    fetch "low_tom.wav" "LowTom.wav"
    fetch "mid_tom.wav" "MedTom.wav"
    fetch "hi_tom.wav"  "HiTom.wav"
    fetch "clap.wav"    "HandClap.wav"
    fetch "hihat.wav"   "HhC.wav"
    fetch "cymbal.wav"  "Crash.wav"
    ;;
  *)
    echo "Unknown machine: $MACHINE"
    echo "Usage: $0 [tr909|tr808|tr707]"
    exit 1
    ;;
esac

echo "Done! ($MACHINE samples in $SAMPLES_DIR)"

#!/usr/bin/env bash
# Render the TurkmenAI product tour on a Kaggle notebook (Internet must be ON),
# then optionally send the MP4 to Telegram.
#
#   TELEGRAM_BOT_TOKEN=xxx bash kaggle-render.sh
#
# Run from the video/ directory (this script's folder).
set -euo pipefail
cd "$(dirname "$0")"

echo "== 1/5 system libraries for headless Chrome =="
if command -v apt-get >/dev/null 2>&1; then
  sudo apt-get update -y >/dev/null 2>&1 || apt-get update -y >/dev/null 2>&1 || true
  PKGS="libnss3 libatk1.0-0 libatk-bridge2.0-0 libcups2 libdrm2 libxkbcommon0 libxcomposite1 libxdamage1 libxfixes3 libxrandr2 libgbm1 libasound2 libpango-1.0-0 libcairo2 fonts-liberation"
  (sudo apt-get install -y $PKGS || apt-get install -y $PKGS) >/dev/null 2>&1 || true
fi

echo "== 2/5 Node.js (need >= 18) =="
NODE_MAJOR="$(node -v 2>/dev/null | sed 's/v\([0-9]*\).*/\1/' || echo 0)"
if [ "${NODE_MAJOR:-0}" -lt 18 ]; then
  curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash - >/dev/null 2>&1 || true
  sudo apt-get install -y nodejs >/dev/null 2>&1 || apt-get install -y nodejs >/dev/null 2>&1 || true
fi
node -v; npm -v

echo "== 3/5 install project dependencies =="
npm install --no-audit --no-fund

echo "== 4/5 render (Tour, 9:16) =="
mkdir -p out
npx remotion render Tour out/turkmenai-tour.mp4 --image-format=jpeg --concurrency=2
ls -lh out/turkmenai-tour.mp4

echo "== 5/5 deliver =="
if [ -n "${TELEGRAM_BOT_TOKEN:-}" ]; then
  pip install requests >/dev/null 2>&1 || true
  python send_telegram.py out/turkmenai-tour.mp4
else
  echo "TELEGRAM_BOT_TOKEN not set — skipping Telegram send. The file is at video/out/turkmenai-tour.mp4"
fi
echo "Done."

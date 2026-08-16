#!/usr/bin/env python3
"""Send the rendered video to Telegram.

The bot token is read from the TELEGRAM_BOT_TOKEN environment variable — never
hardcode it in the repo. chat_id is auto-detected from getUpdates (send any
message to your bot first) unless TELEGRAM_CHAT_ID is set.

Usage:
    TELEGRAM_BOT_TOKEN=xx: python send_telegram.py out/turkmenai-tour.mp4
"""
import os
import sys
import requests

token = os.environ.get("TELEGRAM_BOT_TOKEN")
if not token:
    sys.exit("Set TELEGRAM_BOT_TOKEN in the environment (do not commit it).")
path = sys.argv[1] if len(sys.argv) > 1 else "out/turkmenai-tour.mp4"
if not os.path.exists(path):
    sys.exit(f"File not found: {path}")

chat_id = os.environ.get("TELEGRAM_CHAT_ID")
if not chat_id:
    updates = requests.get(f"https://api.telegram.org/bot{token}/getUpdates", timeout=30).json()
    results = updates.get("result", [])
    ids = []
    for u in results:
        msg = u.get("message") or u.get("channel_post") or {}
        chat = msg.get("chat") or {}
        if chat.get("id") is not None:
            ids.append(chat["id"])
    if not ids:
        sys.exit("No chat_id found. Open Telegram, send any message to your bot, then re-run.")
    chat_id = ids[-1]
    print(f"Detected chat_id: {chat_id}")

lang = ""
for code in ("-en", "-ru", "-tk"):
    if code in os.path.basename(path):
        lang = f" [{code[1:].upper()}]"
caption = f"TurkmenAI Local — product tour (v0.3.0){lang}"
with open(path, "rb") as f:
    resp = requests.post(
        f"https://api.telegram.org/bot{token}/sendVideo",
        data={"chat_id": chat_id, "caption": caption, "supports_streaming": True},
        files={"video": (os.path.basename(path), f, "video/mp4")},
        timeout=600,
    )
print(resp.status_code, resp.text[:300])
resp.raise_for_status()
print("Sent to Telegram.")

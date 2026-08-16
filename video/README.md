# TurkmenAI Local — product tour video (Remotion, 9:16)

On-brand vertical product tour (1080×1920, ~36s) recreating the real app UI in
fast single-focus beats. No fabricated benchmark numbers — capabilities only.
Three languages ship as separate compositions: `Tour` (EN), `TourRU`, `TourTK`.

## Run on Kaggle (fast Internet)

1. Create a new **Kaggle Notebook**, and in the right panel set
   **Settings → Internet → On**.
2. In a cell, get this folder (clone the repo branch):

   ```bash
   !git clone --branch main --depth 1 https://github.com/ambartsumov/turkmenai-local.git
   ```

3. Render and deliver. Paste your bot token where shown (keep it private):

   ```bash
   !cd turkmenai-local/video && TELEGRAM_BOT_TOKEN='PASTE_YOUR_TOKEN' bash kaggle-render.sh
   ```

   Before running, open Telegram and **send any message to your bot** once, so
   the script can auto-detect your chat id. (Or set `TELEGRAM_CHAT_ID` too.)

This renders **all three languages** (`out/turkmenai-tour-en.mp4`, `-ru.mp4`,
`-tk.mp4`) and sends each to your Telegram, captioned with its language.

## Run locally

```bash
cd video
npm install
npm run studio     # preview/scrub in the browser
npm run render     # writes out/turkmenai-tour.mp4
```

## Notes

- The bot token is a secret. It is **never** committed — `send_telegram.py`
  reads `TELEGRAM_BOT_TOKEN` from the environment. If it ever leaks, rotate it in
  @BotFather.
- Rendering needs headless Chrome; `kaggle-render.sh` installs the required
  system libraries automatically.

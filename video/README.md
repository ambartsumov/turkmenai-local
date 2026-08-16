# TurkmenAI Local — product tour video (Remotion, 9:16)

On-brand vertical product tour (1080×1920, ~42s) recreating the real app UI. No
fabricated benchmark numbers — capabilities only. Three languages ship as
separate compositions: `Tour` (EN), `TourRU`, `TourTK`.

## Run on Kaggle (fast Internet)

1. Create a new **Kaggle Notebook**, and in the right panel set
   **Settings → Internet → On**.
2. In a cell, get this folder (clone the repo branch):

   ```bash
   !git clone --branch feat/hf-catalog-datasets --depth 1 https://github.com/ambartsumov/turkmenai-local.git
   ```

3. Render and deliver. Paste your bot token where shown (keep it private):

   ```bash
   !cd turkmenai-local/video && TELEGRAM_BOT_TOKEN='PASTE_YOUR_TOKEN' bash kaggle-render.sh
   ```

   Before running, open Telegram and **send any message to your bot** once, so
   the script can auto-detect your chat id. (Or set `TELEGRAM_CHAT_ID` too.)

The MP4 lands at `video/out/turkmenai-tour.mp4` and is sent to your Telegram.

### Render the RU / TK versions

```bash
!cd turkmenai-local/video && npx remotion render TourRU out/turkmenai-tour-ru.mp4 --image-format=jpeg
!cd turkmenai-local/video && npx remotion render TourTK out/turkmenai-tour-tk.mp4 --image-format=jpeg
```

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

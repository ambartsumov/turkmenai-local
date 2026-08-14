#!/usr/bin/env bash
# =============================================================================
# TurkmenAI Local — Kaggle / Lightning AI build & verification script
# =============================================================================
# Run this in a Kaggle Notebook (Internet: ON, GPU optional) or Lightning AI
# Studio. It does the heavy work that a weak/slow local machine cannot:
#
#   1. Installs the full toolchain (Rust, Node 22, pnpm, Tauri Linux deps)
#   2. Clones the repo
#   3. Typecheck + i18n check + website build
#   4. Rust workspace tests + format check
#   5. Builds the REAL Linux desktop bundle (AppImage / .deb / .rpm)
#   6. REAL model smoke test: downloads a tiny public GGUF and runs an actual
#      llama.cpp OpenAI-compatible inference (uses fast net + optional GPU)
#   7. REAL screenshots of the actual UI via headless Chromium (Playwright)
#   8. (optional) pushes screenshots back to the repo if GITHUB_TOKEN is set
#
# Cross-platform installers (Windows .exe, macOS .dmg) are NOT built here —
# Kaggle is Linux-only. Those are built for free on GitHub Actions' real
# Windows/macOS runners via .github/workflows/release.yml (tag `vX.Y.Z`).
#
# USAGE (Kaggle cell):
#   !curl -fsSL https://raw.githubusercontent.com/ambartsumov/turkmenai-local/main/scripts/kaggle-build.sh | bash
# or upload this file and:  !bash kaggle-build.sh
#
# To push screenshots back, set a token first (fine-grained, Contents: RW):
#   %env GITHUB_TOKEN=ghp_xxx
# =============================================================================
set -uo pipefail
REPO_URL="https://github.com/ambartsumov/turkmenai-local.git"
WORK="${WORK:-$HOME/turkmenai-build}"
MODEL_REPO="Qwen/Qwen2.5-0.5B-Instruct-GGUF"     # tiny, permissive, public
MODEL_FILE="qwen2.5-0.5b-instruct-q4_k_m.gguf"   # ~400 MB
step(){ echo -e "\n\033[1;36m=== $* ===\033[0m"; }
ok(){   echo -e "\033[1;32m[PASS]\033[0m $*"; }
warn(){ echo -e "\033[1;33m[WARN]\033[0m $*"; }
fail(){ echo -e "\033[1;31m[FAIL]\033[0m $*"; }

step "0. Environment"
uname -a; echo "CPUs: $(nproc)"; free -h 2>/dev/null | head -2
nvidia-smi -L 2>/dev/null && export HAS_GPU=1 || { echo "No GPU (CPU-only run)"; export HAS_GPU=0; }

step "1. System dependencies"
export DEBIAN_FRONTEND=noninteractive
sudo apt-get update -qq
sudo apt-get install -y -qq \
  build-essential curl wget git file patchelf pkg-config libssl-dev \
  libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev \
  xvfb imagemagick jq cmake >/dev/null 2>&1 && ok "apt deps installed" || warn "some apt deps failed (continuing)"

step "2. Rust toolchain"
if ! command -v cargo >/dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y >/dev/null
fi
source "$HOME/.cargo/env" 2>/dev/null || true
rustc --version && ok "rust ready"

step "3. Node 22 + pnpm"
if ! command -v node >/dev/null || [ "$(node -v | cut -c2 | tr -d .)" -lt 2 ]; then
  curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash - >/dev/null 2>&1
  sudo apt-get install -y -qq nodejs >/dev/null 2>&1
fi
sudo npm install -g pnpm@10 >/dev/null 2>&1
node -v; pnpm -v && ok "node/pnpm ready"

step "4. Clone repository"
rm -rf "$WORK"; git clone --depth 1 "$REPO_URL" "$WORK"
cd "$WORK"; ok "cloned $(git rev-parse --short HEAD)"

step "5. Install JS dependencies"
pnpm install --frozen-lockfile && ok "pnpm install" || { fail "pnpm install"; exit 1; }

step "6. Typecheck + i18n + website build"
pnpm run check   && ok "check"        || fail "check"
pnpm run build   && ok "website build (dist/public)" || fail "website build"

step "7. Rust tests + format"
cargo test --workspace --exclude turkmenai-desktop && ok "cargo test" || fail "cargo test"
cargo fmt --all -- --check && ok "cargo fmt" || warn "cargo fmt differences"

step "8. Build Linux desktop bundle (Tauri)"
if pnpm desktop:build; then
  ok "desktop bundle built"
  find target -path '*/release/bundle/*' \( -name '*.AppImage' -o -name '*.deb' -o -name '*.rpm' \) -exec ls -lh {} \;
else
  warn "desktop:build failed — check log above"
fi

step "9. REAL model inference smoke test (llama.cpp)"
set +e
pip -q install "huggingface_hub[cli]" >/dev/null 2>&1
mkdir -p "$WORK/models"
hf download "$MODEL_REPO" "$MODEL_FILE" --local-dir "$WORK/models" >/dev/null 2>&1 \
  || huggingface-cli download "$MODEL_REPO" "$MODEL_FILE" --local-dir "$WORK/models" >/dev/null 2>&1
MODEL_PATH="$(find "$WORK/models" -name "$MODEL_FILE" | head -1)"
if [ -z "$MODEL_PATH" ]; then warn "model download failed — skipping inference test"; else
  ok "model: $MODEL_PATH ($(du -h "$MODEL_PATH" | cut -f1))"
  # Prebuilt llama.cpp server (CUDA if GPU present, else CPU)
  pip -q install llama-cpp-python >/dev/null 2>&1
  NGL=0; [ "$HAS_GPU" = 1 ] && NGL=99
  python3 - "$MODEL_PATH" "$NGL" <<'PY'
import sys, json
from llama_cpp import Llama
mp, ngl = sys.argv[1], int(sys.argv[2])
llm = Llama(model_path=mp, n_gpu_layers=ngl, n_ctx=512, verbose=False)
out = llm.create_chat_completion(messages=[{"role":"user","content":"Say hello in one short sentence."}], max_tokens=32)
txt = out["choices"][0]["message"]["content"].strip()
print("[MODEL OUTPUT]", txt)
assert len(txt) > 0, "empty completion"
print("[PASS] real local inference works")
PY
fi
set -e

step "10. REAL UI screenshots (headless Chromium)"
set +e
pnpm dlx playwright@latest install --with-deps chromium >/dev/null 2>&1
npx --yes http-server dist/public -p 4321 >/tmp/srv.log 2>&1 &
SRV=$!; sleep 4
mkdir -p docs/screenshots
node -e '
const {chromium}=require("playwright");
(async()=>{
  const b=await chromium.launch();
  for (const [name,w,h,path] of [
    ["home-desktop",1440,900,"/"],
    ["console-desktop",1440,900,"/console"],
    ["home-mobile",390,844,"/"]]) {
    const p=await b.newPage({viewport:{width:w,height:h}});
    await p.goto("http://127.0.0.1:4321"+path,{waitUntil:"networkidle"}).catch(()=>{});
    await p.waitForTimeout(1200);
    await p.screenshot({path:"docs/screenshots/"+name+".png",fullPage:name.startsWith("home")});
    console.log("shot",name); await p.close();
  }
  await b.close();
})().catch(e=>{console.error(e);process.exit(1)});
' && ok "screenshots in docs/screenshots/" || warn "screenshot step failed"
kill $SRV 2>/dev/null
set -e

step "11. (optional) Push screenshots back to GitHub"
if [ -n "${GITHUB_TOKEN:-}" ] && ls docs/screenshots/*.png >/dev/null 2>&1; then
  git config user.email "kaggle@turkmenai"; git config user.name "TurkmenAI Kaggle Build"
  git add docs/screenshots/*.png
  git commit -qm "docs: real UI screenshots from Kaggle build" || true
  git push "https://x-access-token:${GITHUB_TOKEN}@github.com/ambartsumov/turkmenai-local.git" HEAD:main && ok "screenshots pushed" || warn "push failed"
else
  warn "GITHUB_TOKEN not set — screenshots kept locally only (download them from the notebook)"
fi

step "DONE"
echo "Artifacts under: $WORK/target/*/release/bundle/  and  $WORK/docs/screenshots/"

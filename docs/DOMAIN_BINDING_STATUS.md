# Custom Domain Status — `turkmenai.tech`

**Updated:** 2026-08-14

Hosting moved from the Manus preview (`turkmenai-lxyvv4qu.manus.space`) to **GitHub Pages**, built by `.github/workflows/pages.yml` from `client/`. This removes the earlier blocker ("waiting for platform custom-domain access") because GitHub Pages custom-domain records are public and fixed — see [`DOMAIN_SETUP.md`](../DOMAIN_SETUP.md) for the exact records.

## Repository-side configuration (done)

- GitHub Pages enabled, `build_type: workflow`, source branch `main`.
- Pages `cname` set to `turkmenai.tech` via the repository's Pages API settings.
- `client/public/CNAME` committed with `turkmenai.tech` so every Actions build ships the file GitHub Pages needs.
- `vite.config.ts` `base` fixed to `/` (previously `/turkmenai-local/`, which only makes sense for the `username.github.io/repo` path, not an apex custom domain).
- Latest `pages.yml` run: **success** (build + deploy).

## Outstanding (owner action, Namecheap)

Add the DNS records listed in `DOMAIN_SETUP.md` under `turkmenai.tech → Advanced DNS`, and remove the existing URL redirect. No nameserver change, no DNS API token, and no Namecheap login is available to this environment — this step must be done manually in the Namecheap dashboard.

## Verification after DNS propagates

```bash
curl -I https://turkmenai.tech
curl -I https://www.turkmenai.tech
```
Then confirm in **GitHub → repo → Settings → Pages** that it shows "DNS check successful" and enable **Enforce HTTPS**.

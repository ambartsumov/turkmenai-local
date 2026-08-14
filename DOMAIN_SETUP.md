# Domain Setup — `turkmenai.tech`

## Current status (2026-08-14)

| Item | Status |
|---|---|
| Registered domain | `turkmenai.tech` is active at Namecheap. |
| Current DNS mode | Namecheap BasicDNS. Do **not** change nameservers. |
| Hosting | GitHub Pages, deployed from `.github/workflows/pages.yml` on every push to `main` that touches `client/**`. |
| Custom-domain binding | Configured on the GitHub side: `client/public/CNAME` contains `turkmenai.tech`, and the repository's Pages setting has `cname = turkmenai.tech`. **Waiting on the Namecheap DNS records below** before the domain resolves. |
| Public website (interim) | https://turkmenai-lxyvv4qu.manus.space (still live; keep until `https://turkmenai.tech` verifies). |

## Exact DNS records required (Namecheap → Advanced DNS)

These are GitHub's standard, published Pages IP addresses — not guessed values (https://docs.github.com/en/pages/configuring-a-custom-domain-for-your-github-pages-site).

| Type | Host | Value | TTL |
|---|---|---|---|
| A | `@` | `185.199.108.153` | Automatic |
| A | `@` | `185.199.109.153` | Automatic |
| A | `@` | `185.199.110.153` | Automatic |
| A | `@` | `185.199.111.153` | Automatic |
| CNAME | `www` | `ambartsumov.github.io.` | Automatic |

Steps:

1. Namecheap → **Domain List → turkmenai.tech → Advanced DNS**. Keep **Namecheap BasicDNS** selected.
2. Remove the existing URL Redirect record for `@` (and for `www` if present).
3. Add the four `A` records above for host `@`, and the one `CNAME` record for host `www`.
4. Do not add a CNAME for `@` — apex domains cannot use CNAME; only the `A` records above are valid for `@`.
5. Save. DNS propagation typically takes minutes to a few hours.

## Verification

```bash
curl -I https://turkmenai.tech
curl -I https://www.turkmenai.tech
```

Once DNS has propagated, GitHub issues a Let's Encrypt certificate automatically (usually within ~15–60 minutes of the records resolving correctly) and the repository's **Settings → Pages** will show "DNS check successful" with an option to enforce HTTPS. Until then, `https://turkmenai-lxyvv4qu.manus.space` remains the authoritative live address.

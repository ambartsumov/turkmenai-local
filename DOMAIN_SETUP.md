# Domain Setup — `turkmenai.tech`

## Current status

| Item | Status |
|---|---|
| Registered domain | `turkmenai.tech` is active at Namecheap. |
| Current DNS mode | Namecheap BasicDNS. Do **not** change nameservers. |
| Current redirect | A Namecheap HTTP redirect exists and must be removed only when replacement records are ready. |
| Public website now | https://turkmenai-lxyvv4qu.manus.space |
| Custom-domain binding | **Waiting for custom-domain feature access.** The platform has not issued a DNS target, so no A/CNAME value may be guessed. |

## Exact next actions once custom-domain access is available

1. In the TurkmenAI Local project, open **Settings → Domains**, choose **Add existing domain**, and enter `turkmenai.tech`.
2. Select the option to configure **both** the apex domain and `www`, if the setting is offered. Copy the exact record type, host/name, target/value and TTL shown by the platform.
3. In Namecheap, open **Domain List → turkmenai.tech → Advanced DNS**. Keep **Namecheap BasicDNS** selected.
4. Do **not** change to Namecheap web-hosting nameservers, CustomDNS, PremiumDNS or wildcard DNS records.
5. Remove the existing URL redirect. Add only the platform-issued records for the apex and `www`. If Namecheap already has a conflicting A or CNAME record for the same host, remove that conflicting record only.
6. Before clicking **Save**, compare the visible values character-for-character against the platform’s issued values and obtain an explicit confirmation from the domain owner.
7. Return to **Settings → Domains** and wait for verification and certificate provisioning. Do not claim the custom domain is live until both `https://turkmenai.tech` and `https://www.turkmenai.tech` load over HTTPS.

## Verification

After records are saved and have propagated, verify the public result without exposing account data:

```bash
curl -I https://turkmenai.tech
curl -I https://www.turkmenai.tech
```

The expected result is a successful HTTPS response or an intentional canonical redirect between apex and `www` to the published site. DNS propagation and certificate issuance can take time; the existing `manus.space` URL remains the authoritative live address until that verification passes.

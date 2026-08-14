# Custom Domain Status — `turkmenai.tech`

**Checked:** 2026-08-14

The domain is registered and active at Namecheap under **Namecheap BasicDNS**. Before it can serve TurkmenAI Local, the existing HTTP redirect must be removed and replaced with the precise DNS records provided by the hosting platform.

No nameservers, contact data, payment information, or DNS records were changed during this review. The public site remains available at `https://turkmenai-lxyvv4qu.manus.space`.

## Required next actions

| Step | Owner | Safety condition |
|---|---|---|
| Add `turkmenai.tech` in the project’s custom-domain settings | Account owner | Requires a plan with custom-domain access. |
| Obtain the platform-issued apex and `www` DNS records | Platform | Do not guess A/CNAME targets. |
| Remove the existing Namecheap HTTP redirect and add only the issued records | Registrar owner | Obtain explicit confirmation immediately before saving. |
| Verify certificate issuance and HTTPS on apex and `www` | Project owner / release process | Allow DNS propagation before declaring the domain live. |

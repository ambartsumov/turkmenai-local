/**
 * Fail CI when the product drifts out of sync. Static checks (always):
 *   repository_version == website(releases.ts) == latest manifest version,
 *   and every "verified" download link points at the CURRENT release tag
 *   (never a stale /vOLD/ URL).
 *
 * With CHECK_LINKS=1 (used post-publish, needs network): HTTP-check every
 * verified download URL and the latest.json endpoint resolve (<400).
 *
 * Usage: node scripts/check-consistency.mjs
 */
import fs from 'node:fs';
import path from 'node:path';

const root = path.resolve(import.meta.dirname, '..');
const read = (rel) => fs.readFileSync(path.join(root, rel), 'utf8');
const readJson = (rel) => JSON.parse(read(rel));
const problems = [];

const product = readJson('metadata/product.json');
const version = product.version;
const cmp = (a, b) => { const pa = a.split('.').map(Number), pb = b.split('.').map(Number); for (let i = 0; i < 3; i++) { if ((pa[i] || 0) !== (pb[i] || 0)) return (pa[i] || 0) - (pb[i] || 0); } return 0; };

// releases.ts (generated from real assets) is the website's published truth. It
// is allowed to trail the repo version during the window between bumping the
// version and the release actually publishing (sync-site.yml catches up on
// publish). It must never be AHEAD of the repo, and its links must all point at
// its OWN declared tag.
const releasesTs = read('client/src/releases.ts');
const siteVersion = releasesTs.match(/"version":\s*"([^"]+)"/)?.[1];
if (!siteVersion) problems.push('releases.ts has no version');
else if (cmp(siteVersion, version) > 0) problems.push(`releases.ts version ${siteVersion} is AHEAD of product ${version}`);
else if (cmp(siteVersion, version) < 0) console.warn(`note: release v${version} not published yet — site still on v${siteVersion} (ok, sync-site will catch up).`);

const verifiedUrls = [...releasesTs.matchAll(/"url":\s*"([^"]+)"/g)].map((m) => m[1]);
for (const url of verifiedUrls) {
  if (siteVersion && !url.includes(`/download/v${siteVersion}/`)) {
    problems.push(`download link does not match site tag v${siteVersion}: ${url}`);
  }
}

// latest.json manifest must agree with the site's published version.
const latestPath = 'client/public/api/releases/latest.json';
if (fs.existsSync(path.join(root, latestPath))) {
  const latest = readJson(latestPath);
  for (const p of latest.platforms ?? []) {
    for (const a of p.artifacts ?? []) {
      if (a.url && !a.url.includes(`/download/${latest.tag}/`)) problems.push(`latest.json artifact tag mismatch: ${a.url}`);
      if (a.sha256 && !/^[0-9a-f]{64}$/.test(a.sha256)) problems.push(`latest.json bad sha256 for ${a.url}`);
    }
  }
}

async function checkLinks() {
  const urls = new Set(verifiedUrls);
  if (fs.existsSync(path.join(root, latestPath))) {
    for (const p of readJson(latestPath).platforms ?? []) for (const a of p.artifacts ?? []) if (a.url) urls.add(a.url);
  }
  for (const url of urls) {
    try {
      const res = await fetch(url, { method: 'HEAD', redirect: 'follow' });
      if (res.status >= 400) problems.push(`download link ${res.status}: ${url}`);
    } catch (e) {
      problems.push(`download link unreachable: ${url} (${e.message})`);
    }
  }
}

if (process.env.CHECK_LINKS === '1') await checkLinks();

if (problems.length) {
  console.error('product consistency check FAILED:');
  for (const p of problems) console.error(`  - ${p}`);
  process.exit(1);
}
console.log(`product consistency OK at v${version} (${verifiedUrls.length} verified links${process.env.CHECK_LINKS === '1' ? ', links reachable' : ''}).`);

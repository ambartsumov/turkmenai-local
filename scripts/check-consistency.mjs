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

// releases.ts (generated) — parse its embedded version + verified URLs.
const releasesTs = read('client/src/releases.ts');
const tsVersion = releasesTs.match(/"version":\s*"([^"]+)"/)?.[1];
if (tsVersion !== version) problems.push(`releases.ts version ${tsVersion} != product ${version}`);

const verifiedUrls = [...releasesTs.matchAll(/"url":\s*"([^"]+)"/g)].map((m) => m[1]);
for (const url of verifiedUrls) {
  if (!url.includes(`/download/v${version}/`)) {
    problems.push(`stale download link (not v${version}): ${url}`);
  }
}

// latest.json manifest, if generated.
const latestPath = 'client/public/api/releases/latest.json';
if (fs.existsSync(path.join(root, latestPath))) {
  const latest = readJson(latestPath);
  if (latest.version !== version) problems.push(`latest.json version ${latest.version} != product ${version}`);
  for (const p of latest.platforms ?? []) {
    for (const a of p.artifacts ?? []) {
      if (a.url && !a.url.includes(`/download/v${version}/`)) problems.push(`latest.json stale artifact: ${a.url}`);
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

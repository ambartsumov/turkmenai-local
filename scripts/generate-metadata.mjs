/**
 * Generate the machine-readable product surface the website and tools consume.
 * Everything here is DERIVED from authoritative sources (metadata/*.json, the
 * canonical registry/ catalogs, real GitHub Release assets and git) — nothing
 * is hand-maintained twice. Outputs land in client/public/api/** so Vite ships
 * them as static, highly cacheable /api/*.json endpoints, plus a couple of
 * repo-root status files for tooling.
 *
 * Usage: node scripts/generate-metadata.mjs [tag]   (tag defaults to v<version>)
 */
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { resolveAssets, TARGETS } from './lib/releases.mjs';

const root = path.resolve(import.meta.dirname, '..');
const readJson = (rel) => JSON.parse(fs.readFileSync(path.join(root, rel), 'utf8'));
const git = (args) => {
  try { return execFileSync('git', args, { cwd: root, encoding: 'utf8' }).trim(); }
  catch { return null; }
};

const product = readJson('metadata/product.json');
const features = readJson('metadata/features.json').features;
const platforms = readJson('metadata/platforms.json').platforms;
const modelsRegistry = readJson('registry/catalog.json');
const datasetsRegistry = readJson('registry/datasets.json');

const tag = process.argv[2] || `v${product.version}`;
const generated_at = new Date().toISOString();
const commit = git(['rev-parse', 'HEAD']);

// --- releases (real assets) ----------------------------------------------
const { assets, publishedAt, base } = resolveAssets(tag);
const verified = assets.filter((a) => a.status === 'verified');
// Collapse to one entry per platform (a platform can ship several packages).
const platformArtifacts = [...new Set(TARGETS.map((t) => t.platform))].map((platform) => {
  const items = assets.filter((a) => a.platform === platform);
  return {
    platform,
    status: items.some((i) => i.status === 'verified') ? 'verified' : 'building',
    artifacts: items
      .filter((i) => i.status === 'verified')
      .map((i) => ({ package: i.packageName, url: i.url, size: i.size, sha256: i.sha256 })),
  };
});

const latest = {
  schema_version: 1,
  product: product.product,
  version: product.version,
  tag,
  channel: product.channel,
  published_at: publishedAt,
  generated_at,
  release_url: `${product.repository}/releases/tag/${tag}`,
  platforms: platformArtifacts,
};

const releasesIndex = {
  schema_version: 1,
  latest: { version: product.version, tag, channel: product.channel },
  releases: [{ version: product.version, tag, channel: product.channel, published_at: publishedAt, url: `${product.repository}/releases/tag/${tag}` }],
};

// --- catalogs (published snapshot of the canonical registry) --------------
const modelsIndex = { schema_version: 1, generated_at, source: 'registry/catalog.json', models: modelsRegistry.models };
const datasetsIndex = { schema_version: 1, generated_at, source: 'registry/datasets.json', datasets: datasetsRegistry.datasets };

// --- changelog (grouped strictly from feature metadata) -------------------
const versions = [...new Set(features.map((f) => f.since))].sort((a, b) => cmpSemver(b, a));
const changelog = {
  schema_version: 1,
  generated_at,
  versions: versions.map((v) => ({
    version: v,
    features: features.filter((f) => f.since === v).map((f) => ({ id: f.id, name: f.name.en, category: f.category, status: f.status })),
  })),
};

// --- product status (auditable snapshot) ----------------------------------
const status = {
  schema_version: 1,
  product: product.product,
  version: product.version,
  channel: product.channel,
  release_tag: tag,
  generated_at,
  repository_commit: commit,
  platforms_available: verified.length ? [...new Set(verified.map((v) => v.platform))] : [],
  feature_counts: countBy(features, 'status'),
  planned_not_yet_available: features.filter((f) => f.status === 'planned').map((f) => f.id),
};

// --- write ----------------------------------------------------------------
const api = path.join(root, 'client', 'public', 'api');
writeJson(path.join(api, 'product.json'), { schema_version: 1, generated_at, ...product });
writeJson(path.join(api, 'features', 'index.json'), { schema_version: 1, generated_at, features });
writeJson(path.join(api, 'platforms', 'index.json'), { schema_version: 1, generated_at, platforms });
writeJson(path.join(api, 'models', 'index.json'), modelsIndex);
writeJson(path.join(api, 'datasets', 'index.json'), datasetsIndex);
writeJson(path.join(api, 'releases', 'latest.json'), latest);
writeJson(path.join(api, 'releases', 'index.json'), releasesIndex);
writeJson(path.join(api, 'changelog.json'), changelog);
writeJson(path.join(api, 'product-status.json'), status);
// Repo-root copies for tooling / CI consistency checks.
writeJson(path.join(root, 'PRODUCT_CHANGELOG.json'), changelog);
writeJson(path.join(root, 'product-status.json'), status);

console.log(`metadata generated for ${tag}: ${verified.length}/${assets.length} verified artifacts, ${features.length} features, ${modelsRegistry.models.length} models, ${datasetsRegistry.datasets.length} datasets.`);

function writeJson(abs, obj) {
  fs.mkdirSync(path.dirname(abs), { recursive: true });
  fs.writeFileSync(abs, `${JSON.stringify(obj, null, 2)}\n`);
}
function countBy(list, key) {
  return list.reduce((acc, item) => { acc[item[key]] = (acc[item[key]] || 0) + 1; return acc; }, {});
}
function cmpSemver(a, b) {
  const pa = a.split('.').map(Number), pb = b.split('.').map(Number);
  for (let i = 0; i < 3; i++) { if ((pa[i] || 0) !== (pb[i] || 0)) return (pa[i] || 0) - (pb[i] || 0); }
  return 0;
}

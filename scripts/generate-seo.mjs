/**
 * Regenerate SEO surface (sitemap.xml, robots.txt) from authoritative product
 * metadata so the canonical domain and page list never drift. Network-free:
 * safe to run on every website build. JSON-LD lives inline in client/index.html.
 */
import fs from 'node:fs';
import path from 'node:path';

const root = path.resolve(import.meta.dirname, '..');
const product = JSON.parse(fs.readFileSync(path.join(root, 'metadata/product.json'), 'utf8'));
const origin = product.website.replace(/\/$/, '');
const lastmod = new Date().toISOString().slice(0, 10);

// Only real, crawlable routes. Detail pages are derived from the SAME metadata
// the site renders, so the sitemap never advertises a page that does not exist.
const readJson = (rel) => { try { return JSON.parse(fs.readFileSync(path.join(root, rel), 'utf8')); } catch { return null; } };
const catalog = readJson('registry/catalog.json');
const datasets = readJson('registry/datasets.json');
const releaseIndex = readJson('client/public/api/releases/index.json');

const routes = [
  { loc: '/', changefreq: 'weekly', priority: '1.0' },
  { loc: '/console', changefreq: 'monthly', priority: '0.5' },
  { loc: '/releases', changefreq: 'weekly', priority: '0.6' },
  { loc: '/models', changefreq: 'weekly', priority: '0.7' },
  { loc: '/datasets', changefreq: 'weekly', priority: '0.6' },
  { loc: '/advisor', changefreq: 'monthly', priority: '0.6' },
];
for (const m of (catalog?.models ?? [])) routes.push({ loc: `/models/${m.id}`, changefreq: 'monthly', priority: '0.5' });
for (const d of (datasets?.datasets ?? [])) routes.push({ loc: `/datasets/${d.id}`, changefreq: 'monthly', priority: '0.5' });
for (const r of (releaseIndex?.releases ?? [])) routes.push({ loc: `/releases/${r.tag}`, changefreq: 'monthly', priority: '0.5' });

const urls = routes
  .map((r) => `  <url>\n    <loc>${origin}${r.loc}</loc>\n    <lastmod>${lastmod}</lastmod>\n    <changefreq>${r.changefreq}</changefreq>\n    <priority>${r.priority}</priority>\n  </url>`)
  .join('\n');
const sitemap = `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`;

const robots = `User-agent: *\nAllow: /\n\nSitemap: ${origin}/sitemap.xml\n`;

const pub = path.join(root, 'client', 'public');
fs.writeFileSync(path.join(pub, 'sitemap.xml'), sitemap);
fs.writeFileSync(path.join(pub, 'robots.txt'), robots);
console.log(`SEO regenerated for ${origin} (${routes.length} routes, lastmod ${lastmod}).`);

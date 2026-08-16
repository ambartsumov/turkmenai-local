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

// Only real, crawlable routes. Model/dataset/release detail pages are added here
// as they ship — never advertise a page that does not exist.
const routes = [
  { loc: '/', changefreq: 'weekly', priority: '1.0' },
  { loc: '/console', changefreq: 'monthly', priority: '0.5' },
];

const urls = routes
  .map((r) => `  <url>\n    <loc>${origin}${r.loc}</loc>\n    <lastmod>${lastmod}</lastmod>\n    <changefreq>${r.changefreq}</changefreq>\n    <priority>${r.priority}</priority>\n  </url>`)
  .join('\n');
const sitemap = `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`;

const robots = `User-agent: *\nAllow: /\n\nSitemap: ${origin}/sitemap.xml\n`;

const pub = path.join(root, 'client', 'public');
fs.writeFileSync(path.join(pub, 'sitemap.xml'), sitemap);
fs.writeFileSync(path.join(pub, 'robots.txt'), robots);
console.log(`SEO regenerated for ${origin} (${routes.length} routes, lastmod ${lastmod}).`);

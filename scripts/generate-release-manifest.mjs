import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

const root = path.resolve(import.meta.dirname, '..');
const bundleRoot = path.join(root, 'target', 'release', 'bundle');
const output = path.resolve(root, process.env.RELEASE_MANIFEST_PATH || 'release-manifest/artifacts.json');
const packageJson = JSON.parse(fs.readFileSync(path.join(root, 'package.json'), 'utf8'));
const extensions = ['.deb', '.rpm', '.appimage', '.msi', '.exe', '.dmg'];

function collect(directory) {
  if (!fs.existsSync(directory)) return [];
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const absolute = path.join(directory, entry.name);
    return entry.isDirectory() ? collect(absolute) : [absolute];
  });
}

const artifacts = collect(bundleRoot)
  .filter((file) => extensions.some((extension) => file.toLowerCase().endsWith(extension)))
  .map((file) => ({
    file: path.relative(root, file).split(path.sep).join('/'),
    bytes: fs.statSync(file).size,
    sha256: crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex'),
  }))
  .sort((left, right) => left.file.localeCompare(right.file));

if (artifacts.length === 0) throw new Error('No native release artifacts were found in target/release/bundle.');
fs.mkdirSync(path.dirname(output), { recursive: true });
fs.writeFileSync(output, `${JSON.stringify({
  schema_version: 1,
  product: 'TurkmenAI Local',
  version: packageJson.version,
  source_revision: process.env.GITHUB_SHA || null,
  runner: process.env.RELEASE_RUNNER || process.platform,
  generated_at: new Date().toISOString(),
  artifacts,
}, null, 2)}\n`);
console.log(`Wrote ${output} with ${artifacts.length} artifact(s).`);

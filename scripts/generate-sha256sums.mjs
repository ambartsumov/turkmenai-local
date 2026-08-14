import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

const root = path.resolve(import.meta.dirname, '..');
const bundleRoot = path.join(root, 'target', 'release', 'bundle');
const output = path.resolve(root, process.env.SHA256SUMS_PATH || 'release-manifest/SHA256SUMS.txt');
const extensions = ['.deb', '.rpm', '.appimage', '.msi', '.exe', '.dmg'];

function collect(directory) {
  if (!fs.existsSync(directory)) return [];
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const absolute = path.join(directory, entry.name);
    return entry.isDirectory() ? collect(absolute) : [absolute];
  });
}

const lines = collect(bundleRoot)
  .filter((file) => extensions.some((extension) => file.toLowerCase().endsWith(extension)))
  .sort((left, right) => left.localeCompare(right))
  .map((file) => `${crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex')}  ${path.basename(file)}`);

if (lines.length === 0) throw new Error('No native release artifacts were found in target/release/bundle.');
fs.mkdirSync(path.dirname(output), { recursive: true });
fs.writeFileSync(output, `${lines.join('\n')}\n`);
console.log(`Wrote ${output} with ${lines.length} checksum(s).`);

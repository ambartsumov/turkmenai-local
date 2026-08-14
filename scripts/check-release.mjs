import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

const root = path.resolve(import.meta.dirname, '..');
const requiredIcons = ['desktop/src-tauri/icons/icon.png', 'desktop/src-tauri/icons/icon.ico', 'desktop/src-tauri/icons/icon.icns'];
for (const icon of requiredIcons) if (!fs.existsSync(path.join(root, icon))) throw new Error(`Missing release icon: ${icon}`);
const html = fs.readFileSync(path.join(root, 'client/index.html'), 'utf8');
if (!html.includes('/assets/turkmenai-mark.svg')) throw new Error('Missing local web favicon asset reference (/assets/turkmenai-mark.svg)');
if (!fs.existsSync(path.join(root, 'client/public/assets/turkmenai-mark.svg'))) throw new Error('Missing bundled favicon file client/public/assets/turkmenai-mark.svg');
if (html.includes('manus.space')) throw new Error('index.html still references the temporary manus.space host');

function collect(directory) {
  if (!fs.existsSync(directory)) return [];
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const absolute = path.join(directory, entry.name);
    return entry.isDirectory() ? collect(absolute) : [absolute];
  });
}

const bundleRoot = path.join(root, 'target', 'release', 'bundle');
const artifacts = collect(bundleRoot).filter((file) => /\.(deb|rpm|appimage|msi|exe|dmg)$/i.test(file));
if (artifacts.length === 0) throw new Error('Missing native artifacts under target/release/bundle.');
const expected = process.platform === 'linux' ? ['.deb', '.rpm', '.appimage'] : process.platform === 'win32' ? ['.msi', '.exe'] : process.platform === 'darwin' ? ['.dmg'] : [];
for (const extension of expected) {
  if (!artifacts.some((artifact) => artifact.toLowerCase().endsWith(extension))) throw new Error(`Missing ${extension} native artifact for ${process.platform}.`);
}
for (const artifact of artifacts) {
  const relative = path.relative(root, artifact).split(path.sep).join('/');
  const sum = crypto.createHash('sha256').update(fs.readFileSync(artifact)).digest('hex');
  console.log(`${sum}  ${relative}`);
}

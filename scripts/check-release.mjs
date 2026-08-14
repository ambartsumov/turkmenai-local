import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

const root = path.resolve(import.meta.dirname, '..');
const requiredIcons = ['desktop/src-tauri/icons/icon.png', 'desktop/src-tauri/icons/icon.ico', 'desktop/src-tauri/icons/icon.icns'];
for (const icon of requiredIcons) if (!fs.existsSync(path.join(root, icon))) throw new Error(`Missing release icon: ${icon}`);
const html = fs.readFileSync(path.join(root, 'client/index.html'), 'utf8');
if (!html.includes('/manus-storage/turkmenai-logo-symbol_d3087e01.png')) throw new Error('Missing external web favicon asset reference');
const artifacts = [
  'target/release/bundle/deb/TurkmenAI Local_0.1.0_amd64.deb',
  'target/release/bundle/rpm/TurkmenAI Local-0.1.0-1.x86_64.rpm',
  'target/release/bundle/appimage/TurkmenAI Local_0.1.0_amd64.AppImage',
];
for (const artifact of artifacts) {
  const absolute = path.join(root, artifact);
  if (!fs.existsSync(absolute)) throw new Error(`Missing native artifact: ${artifact}`);
  const sum = crypto.createHash('sha256').update(fs.readFileSync(absolute)).digest('hex');
  console.log(`${sum}  ${artifact}`);
}

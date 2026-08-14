import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

const root = path.resolve(import.meta.dirname, '..');
const command = (name, args) => {
  try { return execFileSync(name, args, { cwd: root, encoding: 'utf8' }).trim(); }
  catch { return 'unavailable'; }
};
const metadata = {
  product: 'TurkmenAI Local',
  version: '0.1.0',
  generated_utc: new Date().toISOString(),
  source_commit: command('git', ['rev-parse', 'HEAD']),
  source_dirty: command('git', ['status', '--porcelain']) !== '',
  target: `${os.platform()}-${os.arch()}`,
  node: process.version,
  rustc: command('rustc', ['--version']),
  cargo: command('cargo', ['--version']),
};
const output = process.argv[2] ?? path.join(root, 'release', 'BUILD_METADATA.json');
fs.mkdirSync(path.dirname(output), { recursive: true });
fs.writeFileSync(output, `${JSON.stringify(metadata, null, 2)}\n`);
console.log(output);

/**
 * Shared release-asset resolution. One place decides which public installer
 * filenames map to which platform, and reads the REAL uploaded assets of a
 * GitHub Release. Both the website download matrix and the machine-readable
 * /api/releases/*.json are generated from this — never hand-typed.
 */
import { execFileSync } from 'node:child_process';

export const REPO = 'ambartsumov/turkmenai-local';

// The eight display rows, in order, mapped to their published filenames.
export const TARGETS = [
  { os: 'Linux', arch: 'x64', packageName: 'AppImage', file: 'TurkmenAI-Local-Linux-x64.AppImage', platform: 'linux-x64' },
  { os: 'Debian / Ubuntu', arch: 'x64', packageName: 'DEB', file: 'TurkmenAI-Local-Linux-x64.deb', platform: 'linux-x64' },
  { os: 'Fedora / RHEL', arch: 'x64', packageName: 'RPM', file: 'TurkmenAI-Local-Linux-x64.rpm', platform: 'linux-x64' },
  { os: 'Linux', arch: 'ARM64', packageName: 'AppImage', file: 'TurkmenAI-Local-Linux-arm64.AppImage', platform: 'linux-arm64' },
  { os: 'Windows 10 / 11', arch: 'x64', packageName: 'EXE (NSIS)', file: 'TurkmenAI-Local-Windows-x64.exe', platform: 'windows-x64' },
  { os: 'Windows 10 / 11', arch: 'ARM64', packageName: 'EXE (NSIS)', file: 'TurkmenAI-Local-Windows-arm64.exe', platform: 'windows-arm64' },
  { os: 'macOS', arch: 'Apple Silicon', packageName: 'DMG', file: 'TurkmenAI-Local-macOS-arm64.dmg', platform: 'macos-arm64' },
  { os: 'macOS', arch: 'Intel x64', packageName: 'DMG', file: 'TurkmenAI-Local-macOS-x64.dmg', platform: 'macos-x64' },
];

/**
 * Read a release's real assets. Returns a Map<filename, {size, apiUrl}> and a
 * Map<filename, sha256> parsed from the uploaded SHA256SUMS.txt when present.
 * Never throws: an unreadable/absent release yields empty maps (all "building").
 */
export function resolveAssets(tag, repo = REPO) {
  const bySize = new Map();
  let publishedAt = null;
  try {
    const raw = execFileSync('gh', ['release', 'view', tag, '--repo', repo, '--json', 'assets,publishedAt'], { encoding: 'utf8' });
    const parsed = JSON.parse(raw);
    publishedAt = parsed.publishedAt || null;
    for (const a of parsed.assets) bySize.set(a.name, { size: a.size ?? null });
  } catch (error) {
    console.error(`releases: could not read ${tag} from ${repo}: ${error.message}`);
  }

  const checksums = parseChecksums(tag, repo, bySize);

  const base = `https://github.com/${repo}/releases/download/${tag}`;
  const assets = TARGETS.map((t) => {
    const meta = bySize.get(t.file);
    const verified = Boolean(meta);
    return {
      ...t,
      status: verified ? 'verified' : 'building',
      url: verified ? `${base}/${t.file}` : null,
      size: verified ? meta.size : null,
      sha256: verified ? checksums.get(t.file) ?? null : null,
    };
  });
  return { assets, publishedAt, base };
}

/**
 * Fetch and parse SHA256SUMS.txt from the release. We only ever surface a
 * checksum that came from the actual published manifest — never a computed
 * guess. Best-effort: returns an empty Map when the file is absent.
 */
function parseChecksums(tag, repo, bySize) {
  const out = new Map();
  if (!bySize.has('SHA256SUMS.txt')) return out;
  try {
    const text = execFileSync('gh', ['release', 'download', tag, '--repo', repo, '--pattern', 'SHA256SUMS.txt', '--output', '-'], { encoding: 'utf8' });
    for (const line of text.split('\n')) {
      const m = line.trim().match(/^([0-9a-f]{64})\s+\*?(.+)$/i);
      if (m) out.set(m[2].trim(), m[1].toLowerCase());
    }
  } catch (error) {
    console.error(`releases: could not read SHA256SUMS.txt for ${tag}: ${error.message}`);
  }
  return out;
}

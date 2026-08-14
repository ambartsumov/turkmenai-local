import fs from 'node:fs';

const content = fs.readFileSync(new URL('../client/src/i18n.ts', import.meta.url), 'utf8');
const locales = ['ru', 'tk', 'en'];
const blocks = locales.map((locale, index) => {
  const start = content.indexOf(`  ${locale}: {`);
  const end = index === locales.length - 1 ? content.indexOf('\n  },\n};', start) : content.indexOf(`\n  ${locales[index + 1]}: {`, start);
  if (start < 0 || end < 0) throw new Error(`Locale block not found: ${locale}`);
  return content.slice(start, end).slice(content.slice(start, end).indexOf('{') + 1);
});
const keySets = blocks.map((block) => new Set([...block.matchAll(/(?:^|,)\s*([A-Za-z][A-Za-z0-9]*)\s*:/g)].map((match) => match[1])));
const reference = keySets[0];
for (let index = 1; index < keySets.length; index += 1) {
  const missing = [...reference].filter((key) => !keySets[index].has(key));
  const extra = [...keySets[index]].filter((key) => !reference.has(key));
  if (missing.length || extra.length) throw new Error(`i18n mismatch for ${locales[index]}: missing=[${missing}] extra=[${extra}]`);
}
console.log(`i18n completeness: ${reference.size} keys across ${locales.join(', ')}`);

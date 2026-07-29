import fs from 'fs';
import path from 'path';
import { marked } from 'marked';

const langs = ['pt-BR', 'pt-PT', 'en-US', 'es-ES'];
const contentDir = 'public';

function extractTitle(md) {
  const match = md.match(/^#\s+(.+)/m);
  return match ? match[1].trim() : '';
}

function extractContent(md) {
  return md.replace(/```[\s\S]*?```/g, '')
           .replace(/`[^`]+`/g, '')
           .replace(/#{1,6}\s+/g, '')
           .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
           .trim();
}

function walkDir(dir, fileList = []) {
  const files = fs.readdirSync(dir);
  for (const file of files) {
    const fullPath = path.join(dir, file);
    const stat = fs.statSync(fullPath);
    if (stat.isDirectory()) {
      walkDir(fullPath, fileList);
    } else if (file.endsWith('.md')) {
      fileList.push(fullPath);
    }
  }
  return fileList;
}

async function buildSearchIndex() {
  const allDocs = [];

  for (const lang of langs) {
    const langDir = path.join(contentDir, lang);
    if (!fs.existsSync(langDir)) continue;

    const files = walkDir(langDir);
    const langDocs = [];

    for (const filePath of files) {
      const md = fs.readFileSync(filePath, 'utf-8');
      const relPath = path.relative(langDir, filePath);

      const title = extractTitle(md) || pathToTitle(relPath);
      const content = extractContent(md);

      const doc = {
        lang,
        path: relPath,
        title,
        content
      };
      allDocs.push(doc);
      langDocs.push(doc);
    }

    fs.writeFileSync(
      path.join(langDir, 'index.json'),
      JSON.stringify(langDocs, null, 2)
    );
    console.log(`Generated search index for ${lang} with ${langDocs.length} documents`);
  }

  const outputDir = path.join('dist');
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  fs.writeFileSync(
    path.join(outputDir, 'search-index.json'),
    JSON.stringify(allDocs)
  );

  console.log(`Generated global search index with ${allDocs.length} documents`);
}

function pathToTitle(path) {
  return path.replace(/\.md$/, '').split('/').map(p => p.charAt(0).toUpperCase() + p.slice(1)).join(' › ');
}

buildSearchIndex().catch(console.error);
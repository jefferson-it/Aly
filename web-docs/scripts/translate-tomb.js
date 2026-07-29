import fs from 'fs';
import path from 'path';

const srcDir = '/home/jefferson/Desktop/projects/aly/docs/constant_and_tomb.md';
const dstDir = '/home/jefferson/Desktop/projects/aly/web-docs/public/pt-BR/constant_and_tomb.md';

const content = `# Constantes e o Mecanismo de Congelamento \`tomb\` em Aly

Aly suporta constantes tradicionais em tempo de compilação, bem como o congelamento dinâmico de mutabilidade em tempo de execução.

---

## 1. Constantes (\`const\`)

As constantes são declaradas em tempo de compilação e não podem ser reatribuídas ou modificadas.

\`\`\`aly
const MAX_LIMIT = 100
\`\`\`

---

## 2. Congelamento Dinâmico (\`tomb\`)

A função nativa \`tomb(&variavel)\` bloqueia uma variável mutável, convertendo-a em uma constante em tempo de execução. Quaisquer tentativas subsequentes de modificar ou reatribuir a variável falharão.
`;

fs.writeFileSync(dstDir, content, 'utf-8');
console.log('Translated constant_and_tomb.md');

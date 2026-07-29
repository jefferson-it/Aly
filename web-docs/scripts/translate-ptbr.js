import fs from 'fs';
import path from 'path';

const srcDir = '/home/jefferson/Desktop/projects/aly/docs';
const dstDir = '/home/jefferson/Desktop/projects/aly/web-docs/public/pt-BR';

function translateToPT(text) {
  const translations = {
    'Semicolons, Line Continuation, and Comments in Aly': 'Ponto e vírgula, continuação de linha e comentários em Aly',
    'This page documents specific lexical rules regarding statement termination, multi-line statements, and comment delimiters in Aly.': 'Esta página documenta regras léxicas específicas sobre terminação de declarações, declarações multi-linha e delimitadores de comentários em Aly.',
    'Comments': 'Comentários',
    'Aly supports single-line and multi-line (block) comments.': 'Aly suporta comentários de linha única e multi-linha (bloco).',
    'Single-line Comments': 'Comentários de linha única',
    'Start with': 'Começam com',
    'and extend to the end of the line.': 'e se estendem até o final da linha.',
    'Multi-line / Block Comments': 'Comentários multi-linha / de bloco',
    'Enclosed within': 'Delimitados por',
    'tags.': 'tags.',
    'Semicolon Semantics': 'Semântica do ponto e vírgula',
    'Semicolons': 'Ponto e vírgula',
    'are not general-purpose statement terminators in Aly.': 'não são terminadores de declaração de propósito geral em Aly.',
    'Statements are implicitly terminated by line breaks (newlines) or block delimiters.': 'As declarações são terminadas implicitamente por quebras de linha (novas linhas) ou delimitadores de bloco {}.',
    'Semicolons in Loop Headers (Only Valid Use-Case)': 'Ponto e vírgula em cabeçalhos de laço (único caso válido)',
    'Line Continuation': 'Continuação de linha',
    'A backslash': 'Uma barra invertida',
    'at the end of a line allows the statement to continue on the next line.': 'no final de uma linha permite que a declaração continue na próxima linha.',
    'Variables in Aly': 'Variáveis em Aly',
    'Variables are declared using the': 'Variáveis são declaradas usando a palavra-chave',
    'keyword.': '.',
    'Type inference is supported.': 'Inferência de tipo é suportada.',
    'Mutable variables use': 'Variáveis mutáveis usam',
    'immutable (constants) use': 'imutáveis (constantes) usam',
    'Data Types in Aly': 'Tipos de dados em Aly',
    'Aly provides the following built-in types:': 'Aly fornece os seguintes tipos embutidos:',
    'Integers': 'Inteiros',
    'Floating-point numbers': 'Números de ponto flutuante',
    'Strings': 'Strings',
    'Booleans': 'Booleanos',
    'Arrays': 'Arrays',
    'Maps': 'Maps',
    'Functions': 'Funções',
    'Control Flow': 'Fluxo de controle',
    'Conditional Statements': 'Declarações condicionais',
    'Loop Statements': 'Declarações de laço',
    'Match Expressions': 'Expressões match',
    'Operators in Aly': 'Operadores em Aly',
    'Arithmetic Operators': 'Operadores aritméticos',
    'Comparison Operators': 'Operadores de comparação',
    'Logical Operators': 'Operadores lógicos',
    'Bitwise Operators': 'Operadores bitwise',
    'Functions in Aly': 'Funções em Aly',
    'Functions are first-class citizens in Aly.': 'Funções são cidadãs de primeira classe em Aly.',
    'Modules': 'Módulos',
    'Traits': 'Traits',
    'Collections': 'Coleções',
    'Strings': 'Strings',
    'Reflection': 'Reflexão',
    'Metaprogramming': 'Metaprogramação',
    'Compiler Backends': 'Backends do compilador',
    'Compiler Optimization': 'Otimização do compilador',
    'Native Libraries': 'Bibliotecas nativas',
    'AI & Machine Learning': 'IA & Machine Learning',
    'Graphics': 'Gráficos',
    'Tools': 'Ferramentas',
    'Getting Started': 'Início Rápido',
    'Language': 'Linguagem',
    'Modules & Libraries': 'Módulos e Bibliotecas',
    'Native': 'Nativo',
    'Graphics': 'Gráficos',
    'Advanced': 'Avançado',
  };

  let result = text;
  for (const [en, pt] of Object.entries(translations)) {
    const regex = new RegExp(en.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'g');
    result = result.replace(regex, pt);
  }
  return result;
}

function walkDir(dir) {
  const files = fs.readdirSync(dir);
  let result = [];
  for (const file of files) {
    const fullPath = path.join(dir, file);
    const stat = fs.statSync(fullPath);
    if (stat.isDirectory()) {
      result = result.concat(walkDir(fullPath));
    } else if (file.endsWith('.md')) {
      result.push(fullPath);
    }
  }
  return result;
}

function processFile(srcPath) {
  const relPath = path.relative(srcDir, srcPath);
  const dstPath = path.join(dstDir, relPath);
  
  const content = fs.readFileSync(srcPath, 'utf-8');
  const translated = translateToPT(content);
  
  const dstDirPath = path.dirname(dstPath);
  if (!fs.existsSync(dstDirPath)) {
    fs.mkdirSync(dstDirPath, { recursive: true });
  }
  
  fs.writeFileSync(dstPath, translated, 'utf-8');
  console.log(`Translated: ${relPath}`);
}

const files = walkDir(srcDir);
for (const file of files) {
  processFile(file);
}

console.log(`Done. Translated ${files.length} files.`);
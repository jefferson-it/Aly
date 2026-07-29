import { marked } from 'marked';

const BASE = '/';
const LANG_KEY = 'aly-docs-lang';
const THEME_KEY = 'aly-docs-theme';
const AVAILABLE_LANGS = ['pt-BR', 'pt-PT', 'en-US', 'es-ES'];

let currentLang = 'pt-BR';
let currentFile = '';
let allDocsIndex = [];

const navGroups = [
  { label: 'Getting Started', path: 'getting-started', ptBR: 'Início', ptPT: 'Início', enUS: 'Getting Started', esES: 'Inicio' },
  { label: 'Language', path: 'language', ptBR: 'Linguagem', ptPT: 'Linguagem', enUS: 'Language', esES: 'Lenguaje' },
  { label: 'Modules & Libraries', path: 'modules', ptBR: 'Módulos e Bibliotecas', ptPT: 'Módulos e Bibliotecas', enUS: 'Modules & Libraries', esES: 'Módulos y Librerías' },
  { label: 'Native', path: 'native', ptBR: 'Nativo', ptPT: 'Nativo', enUS: 'Native', esES: 'Nativo' },
  { label: 'Graphics', path: 'graphics', ptBR: 'Gráficos', ptPT: 'Gráficos', enUS: 'Graphics', esES: 'Gráficos' },
  { label: 'Tools', path: 'tools', ptBR: 'Ferramentas', ptPT: 'Ferramentas', enUS: 'Tools', esES: 'Herramientas' },
  { label: 'Advanced', path: 'advanced', ptBR: 'Avançado', ptPT: 'Avançado', enUS: 'Advanced', esES: 'Avanzado' },
];

const fileMap = {
  'index.md': 'getting-started',
  'syntax_details.md': 'language',
  'variables.md': 'language',
  'data_types.md': 'language',
  'operators.md': 'language',
  'functions.md': 'language',
  'control_flow/conditional.md': 'language',
  'control_flow/loop.md': 'language',
  'control_flow/match.md': 'language',
  'poo/objects.md': 'language',
  'poo/Schema.md': 'language',
  'collections.md': 'language',
  'strings.md': 'language',
  'traits.md': 'language',
  'modules.md': 'language',
  'reflection.md': 'language',
  'metaprogramming.md': 'language',
  'compiler_backends.md': 'language',
  'compiler_optimization.md': 'language',
  'constant_and_tomb.md': 'language',
  'cpp_abi.md': 'language',
  'ai_ml.md': 'modules',
  'native_libraries.md': 'modules',
  'native_libraries/security.md': 'modules',
  'native_libraries/json.md': 'modules',
  'native_libraries/math_crypto_regex.md': 'modules',
  'native_libraries/filesystem.md': 'modules',
  'native_libraries/system.md': 'modules',
  'native_libraries/datetime.md': 'modules',
  'native_libraries/os_shell.md': 'modules',
  'native_libraries/iot.md': 'modules',
  'native_libraries/database.md': 'modules',
  'native_libraries/net.md': 'modules',
  'native_libraries/web.md': 'modules',
  'native_libraries/data_science.md': 'modules',
  'native_libraries/data_formats.md': 'modules',
  'native_libraries/game.md': 'modules',
  'native_libraries/assertions.md': 'modules',
  'native.md': 'native',
  'bindings.md': 'native',
  'rpc_microservices.md': 'native',
  'schemas_advanced.md': 'native',
  'graphics/geral.md': 'graphics',
  'graphics/cocoa.md': 'graphics',
  'graphics/gtk.md': 'graphics',
  'graphics/fltk.md': 'graphics',
  'tools/index.md': 'tools',
  'render.md': 'advanced',
  'repl.md': 'advanced',
  'jot.md': 'advanced',
  'concurrency.md': 'advanced',
  'android.md': 'advanced',
};

function getContentDir() {
  return BASE + currentLang;
}

const fileTitles = {
  'index.md': { ptBR: 'Início', ptPT: 'Início', enUS: 'Home', esES: 'Inicio' },
  'syntax_details.md': { ptBR: 'Detalhes de Sintaxe', ptPT: 'Detalhes de Sintaxe', enUS: 'Syntax Details', esES: 'Detalles de Sintaxis' },
  'variables.md': { ptBR: 'Variáveis', ptPT: 'Variáveis', enUS: 'Variables', esES: 'Variables' },
  'data_types.md': { ptBR: 'Tipos de Dados', ptPT: 'Tipos de Dados', enUS: 'Data Types', esES: 'Tipos de Datos' },
  'operators.md': { ptBR: 'Operadores', ptPT: 'Operadores', enUS: 'Operators', esES: 'Operadores' },
  'functions.md': { ptBR: 'Funções', ptPT: 'Funções', enUS: 'Functions', esES: 'Funciones' },
  'control_flow/conditional.md': { ptBR: 'Condicional', ptPT: 'Condicional', enUS: 'Conditional', esES: 'Condicional' },
  'control_flow/loop.md': { ptBR: 'Laço', ptPT: 'Ciclo', enUS: 'Loop', esES: 'Bucle' },
  'control_flow/match.md': { ptBR: 'Match', ptPT: 'Match', enUS: 'Match', esES: 'Match' },
  'poo/objects.md': { ptBR: 'Objetos', ptPT: 'Objetos', enUS: 'Objects', esES: 'Objetos' },
  'poo/Schema.md': { ptBR: 'Schema', ptPT: 'Schema', enUS: 'Schema', esES: 'Schema' },
  'collections.md': { ptBR: 'Coleções', ptPT: 'Coleções', enUS: 'Collections', esES: 'Colecciones' },
  'strings.md': { ptBR: 'Strings', ptPT: 'Strings', enUS: 'Strings', esES: 'Strings' },
  'traits.md': { ptBR: 'Traits', ptPT: 'Traits', enUS: 'Traits', esES: 'Traits' },
  'modules.md': { ptBR: 'Módulos', ptPT: 'Módulos', enUS: 'Modules', esES: 'Módulos' },
  'reflection.md': { ptBR: 'Reflexão', ptPT: 'Reflexão', enUS: 'Reflection', esES: 'Reflexión' },
  'metaprogramming.md': { ptBR: 'Metaprogramação', ptPT: 'Metaprogramação', enUS: 'Metaprogramming', esES: 'Metaprogramación' },
  'compiler_backends.md': { ptBR: 'Backends do Compilador', ptPT: 'Backends do Compilador', enUS: 'Compiler Backends', esES: 'Backends del Compilador' },
  'compiler_optimization.md': { ptBR: 'Otimização do Compilador', ptPT: 'Otimização do Compilador', enUS: 'Compiler Optimization', esES: 'Optimización del Compilador' },
  'constant_and_tomb.md': { ptBR: 'Constante e Tomb', ptPT: 'Constante e Tomb', enUS: 'Constant and Tomb', esES: 'Constante y Tomb' },
  'cpp_abi.md': { ptBR: 'ABI C++', ptPT: 'ABI C++', enUS: 'C++ ABI', esES: 'ABI C++' },
  'ai_ml.md': { ptBR: 'IA e Machine Learning', ptPT: 'IA e Aprendizagem Automática', enUS: 'AI & Machine Learning', esES: 'IA y Machine Learning' },
  'native_libraries.md': { ptBR: 'Bibliotecas Nativas', ptPT: 'Bibliotecas Nativas', enUS: 'Native Libraries', esES: 'Librerías Nativas' },
  'native_libraries/security.md': { ptBR: 'Segurança', ptPT: 'Segurança', enUS: 'Security', esES: 'Seguridad' },
  'native_libraries/json.md': { ptBR: 'JSON', ptPT: 'JSON', enUS: 'JSON', esES: 'JSON' },
  'native_libraries/math_crypto_regex.md': { ptBR: 'Matemática, Cripto e Regex', ptPT: 'Matemática, Cripto e Regex', enUS: 'Math, Crypto & Regex', esES: 'Matemáticas, Cripto y Regex' },
  'native_libraries/filesystem.md': { ptBR: 'Sistema de Arquivos', ptPT: 'Sistema de Ficheiros', enUS: 'File System', esES: 'Sistema de Archivos' },
  'native_libraries/system.md': { ptBR: 'Sistema', ptPT: 'Sistema', enUS: 'System', esES: 'Sistema' },
  'native_libraries/datetime.md': { ptBR: 'Data e Hora', ptPT: 'Data e Hora', enUS: 'Date & Time', esES: 'Fecha y Hora' },
  'native_libraries/os_shell.md': { ptBR: 'Shell do SO', ptPT: 'Shell do SO', enUS: 'OS Shell', esES: 'Shell del SO' },
  'native_libraries/iot.md': { ptBR: 'IoT', ptPT: 'IoT', enUS: 'IoT', esES: 'IoT' },
  'native_libraries/database.md': { ptBR: 'Banco de Dados', ptPT: 'Base de Dados', enUS: 'Database', esES: 'Base de Datos' },
  'native_libraries/net.md': { ptBR: 'Rede', ptPT: 'Rede', enUS: 'Network', esES: 'Red' },
  'native_libraries/web.md': { ptBR: 'Web', ptPT: 'Web', enUS: 'Web', esES: 'Web' },
  'native_libraries/data_science.md': { ptBR: 'Ciência de Dados', ptPT: 'Ciência de Dados', enUS: 'Data Science', esES: 'Ciencia de Datos' },
  'native_libraries/data_formats.md': { ptBR: 'Formatos de Dados', ptPT: 'Formatos de Dados', enUS: 'Data Formats', esES: 'Formatos de Datos' },
  'native_libraries/game.md': { ptBR: 'Jogos', ptPT: 'Jogos', enUS: 'Game', esES: 'Juegos' },
  'native_libraries/assertions.md': { ptBR: 'Asserções', ptPT: 'Asserções', enUS: 'Assertions', esES: 'Asertos' },
  'native.md': { ptBR: 'Programação Nativa', ptPT: 'Programação Nativa', enUS: 'Native Programming', esES: 'Programación Nativa' },
  'bindings.md': { ptBR: 'Bindings', ptPT: 'Bindings', enUS: 'Bindings', esES: 'Bindings' },
  'rpc_microservices.md': { ptBR: 'RPC e Microsserviços', ptPT: 'RPC e Microsserviços', enUS: 'RPC & Microservices', esES: 'RPC y Microservicios' },
  'schemas_advanced.md': { ptBR: 'Esquemas Avançados', ptPT: 'Schemas Avançados', enUS: 'Advanced Schemas', esES: 'Esquemas Avanzados' },
  'graphics/geral.md': { ptBR: 'Visão Geral', ptPT: 'Visão Geral', enUS: 'Overview', esES: 'Visión General' },
  'graphics/cocoa.md': { ptBR: 'Cocoa', ptPT: 'Cocoa', enUS: 'Cocoa', esES: 'Cocoa' },
  'graphics/gtk.md': { ptBR: 'GTK', ptPT: 'GTK', enUS: 'GTK', esES: 'GTK' },
  'graphics/fltk.md': { ptBR: 'FLTK', ptPT: 'FLTK', enUS: 'FLTK', esES: 'FLTK' },
  'tools/index.md': { ptBR: 'Visão Geral', ptPT: 'Visão Geral', enUS: 'Overview', esES: 'Visión General' },
  'render.md': { ptBR: 'Render', ptPT: 'Render', enUS: 'Render', esES: 'Render' },
  'repl.md': { ptBR: 'REPL', ptPT: 'REPL', enUS: 'REPL', esES: 'REPL' },
  'jot.md': { ptBR: 'Jot', ptPT: 'Jot', enUS: 'Jot', esES: 'Jot' },
  'concurrency.md': { ptBR: 'Concorrência', ptPT: 'Concorrência', enUS: 'Concurrency', esES: 'Concurrencia' },
  'android.md': { ptBR: 'Android', ptPT: 'Android', enUS: 'Android', esES: 'Android' },
};

function pathToTitle(path) {
  const title = fileTitles[path];
  if (!title) {
    const parts = path.replace(/\.md$/, '').split('/');
    return parts.map(p => p.charAt(0).toUpperCase() + p.slice(1)).join(' › ');
  }
  const langKey = currentLang === 'pt-BR' ? 'ptBR' : currentLang === 'pt-PT' ? 'ptPT' : currentLang === 'es-ES' ? 'esES' : 'enUS';
  return title[langKey] || title.enUS;
}

function getGroupLabel(group) {
  const key = currentLang === 'pt-BR' ? 'ptBR' : currentLang === 'pt-PT' ? 'ptPT' : currentLang === 'es-ES' ? 'esES' : 'enUS';
  return group[key] || group.label;
}

function buildSidebar() {
  const nav = document.getElementById('sidebar-nav');
  nav.innerHTML = '';

  const grouped = {};
  navGroups.forEach(g => { grouped[g.path] = []; });

  Object.entries(fileMap).forEach(([path, groupPath]) => {
    const title = pathToTitle(path);
    grouped[groupPath].push({ path, title });
  });

  navGroups.forEach(group => {
    if (grouped[group.path].length === 0) return;

    const groupLabel = document.createElement('div');
    groupLabel.className = 'nav-group';
    groupLabel.textContent = getGroupLabel(group);
    nav.appendChild(groupLabel);

    grouped[group.path].forEach(file => {
      const link = document.createElement('a');
      link.href = '#' + currentLang + '/' + file.path;
      link.textContent = file.title;
      link.dataset.path = file.path;
      if (file.path === currentFile) {
        link.classList.add('active');
      }
      link.addEventListener('click', (e) => {
        e.preventDefault();
        loadDoc(file.path);
      });
      nav.appendChild(link);
    });
  });
}

function updateActiveNav() {
  const links = document.querySelectorAll('#sidebar-nav a');
  links.forEach(link => {
    link.classList.toggle('active', link.dataset.path === currentFile);
  });
}

function getCurrentFilePath() {
  const hash = window.location.hash.slice(1);
  if (!hash) return { lang: currentLang, path: 'index.md' };
  const parts = hash.split('/');
  if (parts.length >= 2 && AVAILABLE_LANGS.includes(parts[0])) {
    return { lang: parts[0], path: parts.slice(1).join('/') };
  }
  return { lang: currentLang, path: parts[0] || 'index.md' };
}

async function loadDoc(filePath) {
  currentFile = filePath;
  const url = getContentDir() + '/' + filePath;

  try {
    const res = await fetch(url);
    if (!res.ok) throw new Error('Not found');
    const md = await res.text();

    const content = document.getElementById('markdown-content');
    content.innerHTML = marked.parse(md);

    const titleMatch = md.match(/^#\s+(.+)/m);
    const title = titleMatch ? titleMatch[1] : pathToTitle(filePath);
    document.getElementById('page-title').textContent = title;

    updateActiveNav();

    window.location.hash = currentLang + '/' + filePath;

    closeSidebar();
  } catch (err) {
    document.getElementById('markdown-content').innerHTML =
      '<p style="color:var(--color-accent);">Document not found: ' + filePath + '</p>';
  }
}

async function buildSearchIndex() {
  const res = await fetch(BASE + currentLang + '/index.json');
  if (res.ok) {
    allDocsIndex = await res.json();
  }
}

function initLang() {
  const saved = localStorage.getItem(LANG_KEY);
  const urlLang = getCurrentFilePath().lang;
  if (saved && AVAILABLE_LANGS.includes(saved)) {
    currentLang = saved;
  } else if (urlLang && AVAILABLE_LANGS.includes(urlLang)) {
    currentLang = urlLang;
  } else {
    const browserLang = navigator.language;
    if (AVAILABLE_LANGS.includes(browserLang)) {
      currentLang = browserLang;
    } else if (browserLang.startsWith('pt')) {
      currentLang = 'pt-BR';
    } else if (browserLang.startsWith('es')) {
      currentLang = 'es-ES';
    } else {
      currentLang = 'en-US';
    }
  }
  document.getElementById('lang-select').value = currentLang;
}

function switchLang(lang) {
  if (!AVAILABLE_LANGS.includes(lang)) return;
  currentLang = lang;
  localStorage.setItem(LANG_KEY, lang);
  document.getElementById('lang-select').value = lang;

  const hash = window.location.hash.slice(1);
  const parts = hash.split('/');
  let newPath;
  if (parts.length >= 2 && AVAILABLE_LANGS.includes(parts[0])) {
    newPath = parts.slice(1).join('/');
  } else {
    newPath = parts[0] || 'index.md';
  }

  window.location.hash = lang + '/' + newPath;
  buildSidebar();
  buildSearchIndex();
  loadDoc(newPath);
}

function initTheme() {
  const saved = localStorage.getItem(THEME_KEY);
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  const isDark = saved === 'dark' || (!saved && prefersDark);

  if (isDark) {
    document.documentElement.classList.add('dark');
    document.documentElement.classList.remove('light');
    document.getElementById('theme-toggle').textContent = '☀️';
  } else {
    document.documentElement.classList.add('light');
    document.documentElement.classList.remove('dark');
    document.getElementById('theme-toggle').textContent = '🌙';
  }
}

function toggleTheme() {
  const isDark = document.documentElement.classList.contains('dark');
  if (isDark) {
    document.documentElement.classList.remove('dark');
    document.documentElement.classList.add('light');
    document.getElementById('theme-toggle').textContent = '🌙';
    localStorage.setItem(THEME_KEY, 'light');
  } else {
    document.documentElement.classList.remove('light');
    document.documentElement.classList.add('dark');
    document.getElementById('theme-toggle').textContent = '☀️';
    localStorage.setItem(THEME_KEY, 'dark');
  }
}

function openSidebar() {
  document.getElementById('sidebar').classList.add('open');
  document.getElementById('sidebar-overlay').classList.add('active');
}

function closeSidebar() {
  document.getElementById('sidebar').classList.remove('open');
  document.getElementById('sidebar-overlay').classList.remove('active');
}

function toggleSidebar() {
  const sidebar = document.getElementById('sidebar');
  if (sidebar.classList.contains('open')) {
    closeSidebar();
  } else {
    openSidebar();
  }
}

function openSearch() {
  const modal = document.getElementById('search-modal');
  const input = document.getElementById('search-input');
  modal.classList.add('active');
  input.value = '';
  input.focus();
}

function closeSearch() {
  document.getElementById('search-modal').classList.remove('active');
}

function renderSearchResults(query) {
  const resultsEl = document.getElementById('search-results');
  const lowerQuery = query.toLowerCase();
  if (!lowerQuery) {
    resultsEl.innerHTML = '<p class="search-hint">Type to search...</p>';
    return;
  }

  const results = allDocsIndex
    .filter(doc => doc.title.toLowerCase().includes(lowerQuery) || doc.content.toLowerCase().includes(lowerQuery))
    .slice(0, 10);

  if (results.length === 0) {
    resultsEl.innerHTML = '<p class="search-hint">No results found</p>';
    return;
  }

  resultsEl.innerHTML = results.map(doc => `
    <a href="#${currentLang}/${doc.path}" class="search-result" data-path="${doc.path}">
      <span class="search-title">${doc.title}</span>
      <span class="search-path">${doc.path}</span>
    </a>
  `).join('');

  resultsEl.querySelectorAll('.search-result').forEach(link => {
    link.addEventListener('click', (e) => {
      e.preventDefault();
      loadDoc(link.dataset.path);
      closeSearch();
    });
  });
}

function initSearch() {
  const input = document.getElementById('search-input');
  const modal = document.getElementById('search-modal');
  const closeBtn = document.getElementById('search-close');

  document.addEventListener('keydown', (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
      e.preventDefault();
      openSearch();
    }
    if (e.key === 'Escape') {
      closeSearch();
    }
  });

  input.addEventListener('input', (e) => {
    renderSearchResults(e.target.value);
  });

  closeBtn.addEventListener('click', closeSearch);
  modal.addEventListener('click', (e) => {
    if (e.target === modal) closeSearch();
  });
}

const LANG_LABELS = {
  'pt-BR': 'Português (BR)',
  'pt-PT': 'Português (PT)',
  'en-US': 'English',
  'es-ES': 'Español',
};

export async function initApp() {
  initLang();
  initTheme();
  buildSidebar();
  await buildSearchIndex();
  initSearch();

  document.getElementById('lang-select').addEventListener('change', (e) => {
    switchLang(e.target.value);
  });

  document.getElementById('theme-toggle').addEventListener('click', toggleTheme);
  document.getElementById('menu-btn').addEventListener('click', toggleSidebar);
  document.getElementById('sidebar-toggle').addEventListener('click', toggleSidebar);
  document.getElementById('sidebar-overlay').addEventListener('click', closeSidebar);

  document.getElementById('markdown-content').addEventListener('click', async (e) => {
    const link = e.target.closest('a');
    if (!link) return;

    const href = link.getAttribute('href');
    if (!href) return;

    // Skip absolute URLs, protocol-relative, and purely hash-local links
    if (href.startsWith('http://') || href.startsWith('https://') || href.startsWith('//') || href.startsWith('#')) {
      return;
    }

    const hashIndex = href.indexOf('#');
    const filePath = hashIndex === -1 ? href : href.substring(0, hashIndex);
    const hashAnchor = hashIndex === -1 ? '' : href.substring(hashIndex);

    if (!filePath) return;

    // Check if it is a relative markdown doc or mapped page
    if (filePath.endsWith('.md') || Object.keys(fileMap).some(p => filePath.includes(p))) {
      e.preventDefault();

      let targetPath = filePath;
      if (currentFile.includes('/')) {
        const currentDir = currentFile.substring(0, currentFile.lastIndexOf('/'));
        targetPath = currentDir + '/' + filePath;
      }

      const parts = targetPath.split('/');
      const resolvedParts = [];
      for (const part of parts) {
        if (part === '.' || part === '') continue;
        if (part === '..') {
          resolvedParts.pop();
        } else {
          resolvedParts.push(part);
        }
      }
      targetPath = resolvedParts.join('/');

      await loadDoc(targetPath);

      if (hashAnchor) {
        const decodedAnchor = decodeURIComponent(hashAnchor.slice(1));
        const targetEl = document.getElementById(decodedAnchor) || document.querySelector(`[id="${decodedAnchor}"]`);
        if (targetEl) {
          targetEl.scrollIntoView({ behavior: 'smooth' });
        }
      }
    }
  });

  window.addEventListener('hashchange', () => {
    const { lang, path } = getCurrentFilePath();
    if (lang && lang !== currentLang) {
      currentLang = lang;
      document.getElementById('lang-select').value = lang;
      buildSidebar();
    }
    if (path && path !== currentFile) {
      loadDoc(path);
    }
  });

  const { path: initialPath } = getCurrentFilePath();
  loadDoc(initialPath);
}
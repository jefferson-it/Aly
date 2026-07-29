import { marked } from 'marked';

const BASE = '/aly/';
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

function pathToTitle(path) {
  const parts = path.replace(/\.md$/, '').split('/');
  return parts.map(p => p.charAt(0).toUpperCase() + p.slice(1)).join(' › ');
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
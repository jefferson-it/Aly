#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# Aly Language - Editor Extension Installer
# ═══════════════════════════════════════════════════════════════════════════════
set +e  # Não sair em erros

EDITOR_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EDITOR_NAME="aly"
DISPLAY_NAME="Aly Programming Language"

# ── Colors ────────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
DIM='\033[2m'
RESET='\033[0m'

# ── Helpers ───────────────────────────────────────────────────────────────────
info()    { echo -e "${CYAN}[INFO]${RESET}  $*"; }
success() { echo -e "${GREEN}[OK]${RESET}    $*"; }
warn()    { echo -e "${YELLOW}[WARN]${RESET}  $*"; }
error()   { echo -e "${RED}[ERROR]${RESET} $*"; }
line()    { echo -e "${DIM}$(printf '─%.0s' {1..60})${RESET}"; }

confirm() {
    local msg="${1:-Continue?}"
    echo -en "${BOLD}${msg} [Y/n]: ${RESET}"
    read -r reply
    [[ -z "$reply" || "$reply" =~ ^[Yy]$ ]]
}

# ── Banner ────────────────────────────────────────────────────────────────────
banner() {
    clear
    echo -e "${CYAN}${BOLD}"
    cat << 'EOF'
     _                 _
    / \   __ _  __ _(_) ___  ___
   / _ \ / _` |/ _` | |/ _ \/ __|
  / ___ \ (_| | (_| | | (_) \__ \
 /_/   \_\__, |\__, |_|\___/|___/
         |___/   Editor Installer
EOF
    echo -e "${RESET}"
    line
    echo -e "  ${DIM}Instale extensões de sintaxe e suporte para Aly${RESET}"
    line
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════════
# VS Code
# ═══════════════════════════════════════════════════════════════════════════════
install_vscode() {
    info "Verificando VS Code..."

    local vscode_cmd=""

    # Check common VS Code binary names
    for cmd in code code-insiders code-oss; do
        if command -v "$cmd" &>/dev/null; then
            vscode_cmd="$cmd"
            break
        fi
    done

    if [[ -z "$vscode_cmd" ]]; then
        error "VS Code não encontrado no PATH."
        echo ""
        echo -e "  Instale o VS Code em: ${BOLD}https://code.visualstudio.com/${RESET}"
        echo ""
        echo -e "  ${DIM}Ou instale via terminal (Ubuntu/Debian):${RESET}"
        echo -e "  ${CYAN}sudo snap install code --classic${RESET}"
        echo ""
        if confirm "Tentar instalar via snap agora?"; then
            if command -v snap &>/dev/null; then
                sudo snap install code --classic
                vscode_cmd="code"
            else
                error "snap não disponível. Instale manualmente."
                return 1
            fi
        else
            return 1
        fi
    fi

    success "VS Code encontrado: $vscode_cmd"

    # Use the build script to create .vsix and install
    if [[ -f "$EDITOR_DIR/build_extension.sh" ]]; then
        info "Empacotando e instalando extensão..."
        bash "$EDITOR_DIR/build_extension.sh"
    else
        error "build_extension.sh não encontrado em $EDITOR_DIR"
        return 1
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# Cursor
# ═══════════════════════════════════════════════════════════════════════════════
install_cursor() {
    info "Verificando Cursor..."

    local cursor_cmd=""

    for cmd in cursor; do
        if command -v "$cmd" &>/dev/null; then
            cursor_cmd="$cmd"
            break
        fi
    done

    if [[ -z "$cursor_cmd" ]]; then
        # Check common Cursor paths
        local paths=(
            "/usr/bin/cursor"
            "/opt/cursor/cursor"
            "$HOME/.local/share/cursor/cursor"
        )
        for p in "${paths[@]}"; do
            if [[ -x "$p" ]]; then
                cursor_cmd="$p"
                break
            fi
        done
    fi

    if [[ -z "$cursor_cmd" ]]; then
        error "Cursor não encontrado."
        echo ""
        echo -e "  Instale o Cursor em: ${BOLD}https://cursor.sh/${RESET}"
        return 1
    fi

    success "Cursor encontrado: $cursor_cmd"

    local ext_dir="$HOME/.cursor/extensions/${EDITOR_NAME}-lang-0.1.0"
    info "Copiando extensão para: $ext_dir"

    rm -rf "$ext_dir" 2>/dev/null
    mkdir -p "$ext_dir"
    cp -r "$EDITOR_DIR/package.json" "$ext_dir/"
    cp -r "$EDITOR_DIR/extension.js" "$ext_dir/"
    cp -r "$EDITOR_DIR/language-configuration.json" "$ext_dir/"
    cp -r "$EDITOR_DIR/syntaxes" "$ext_dir/"
    cp -r "$EDITOR_DIR/snippets" "$ext_dir/"

    success "Extensão instalada no Cursor!"
    echo ""
    echo -e "  ${DIM}Reinicie o Cursor para ativar a extensão.${RESET}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Neovim / Vim (via nvim-lspconfig or vim-lsp)
# ═══════════════════════════════════════════════════════════════════════════════
install_neovim() {
    info "Configurando suporte para Neovim..."

    local nvim_dir="${XDG_DATA_HOME:-$HOME/.local/share}/nvim/site"
    local syntax_dir="$nvim_dir/syntax"
    local ftdetect_dir="$nvim_dir/ftdetect"

    mkdir -p "$syntax_dir" "$ftdetect_dir"

    # Create ftdetect for .aly files
    cat > "$ftdetect_dir/aly.vim" << 'VIMSCRIPT'
autocmd BufRead,BufNewFile *.aly setfiletype aly
VIMSCRIPT

    success "ftdetect criado: $ftdetect_dir/aly.vim"

    # Create syntax file (basic keywords from Aly)
    cat > "$syntax_dir/aly.vim" << 'VIMSCRIPT'
if exists("b:current_syntax")
    finish
endif

" Keywords
syn keyword alyKeyword let const fun return if elif else loop do match
syn keyword alyKeyword try catch finally throw break async await
syn keyword alyKeyword struct model plugin import

" Operators
syn keyword alyOperator and or xor not
syn match alyOperator "\v(eq|neq|lt|lte|gt|gte)"
syn match alyOperator "[+\-*/%|]"
syn match alyOperator "\v(\=|\+\=|\-\=|\*\=|/\=|\%\=)"
syn match alyOperator "\v(\?|\:)"

" Values
syn keyword alyBoolean true false
syn keyword alyNone None

" Strings
syn region alyString start='"' end='"' contains=alyInterpolation
syn region alyString start="'" end="'"
syn match alyInterpolation "\v\&[a-zA-Z_][a-zA-Z0-9_.]*"

" Numbers
syn match alyFloat "\v\d+\.\d+"
syn match alyInteger "\v\d+"
syn match alyPercent "\v\d+\%"

" Comments
syn match alyComment "\v#.*$" contains=@Spell
syn region alyCommentBlock start="##" end="##" contains=@Spell

" Functions
syn match alyFunction "\v[a-zA-Z_][a-zA-Z0-9_]*\s*\ze\("

" Namespaces
syn match alyNamespace "\v(fs|str|sys|shell|os|console|crypto|gui|curl|http_api)\ze\."

" Highlighting
hi def link alyKeyword     Keyword
hi def link alyOperator    Operator
hi def link alyBoolean     Boolean
hi def link alyNone        Constant
hi def link alyString      String
hi def link alyInterpolation Special
hi def link alyFloat       Float
hi def link alyInteger     Number
hi def link alyPercent     Number
hi def link alyComment     Comment
hi def link alyCommentBlock Comment
hi def link alyFunction    Function
hi def link alyNamespace   Type

let b:current_syntax = "aly"
VIMSCRIPT

    success "Syntax file criado: $syntax_dir/aly.vim"
    success "Suporte Neovim configurado!"
    echo ""
    echo -e "  ${DIM}Para LSP completo, adicione ao seu init.vim/init.lua:${RESET}"
    echo -e "  ${CYAN}require'lspconfig'.als.setup{}${RESET}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Sublime Text
# ═══════════════════════════════════════════════════════════════════════════════
install_sublime() {
    info "Configurando suporte para Sublime Text..."

    local sublime_dir
    local os_type
    os_type="$(uname -s)"

    case "$os_type" in
        Linux)
            sublime_dir="$HOME/.config/sublime-text/Packages/User"
            ;;
        Darwin)
            sublime_dir="$HOME/Library/Application Support/Sublime Text/Packages/User"
            ;;
        *)
            error "Sistema operacional não suportado: $os_type"
            return 1
            ;;
    esac

    local syntax_dir="$sublime_dir/Aly"
    mkdir -p "$syntax_dir"

    # Create syntax definition (TextMate format adapted)
    cat > "$syntax_dir/Aly.sublime-syntax" << 'YAML'
%YAML 1.2
---
name: Aly
file_extensions:
  - aly
scope: source.aly

contexts:
  main:
    - include: comments
    - include: strings
    - include: keywords
    - include: operators
    - include: constants
    - include: numbers
    - include: functions

  comments:
    - match: '#'
      scope: punctuation.definition.comment.aly
      push:
        - meta_scope: comment.line.number-sign.aly
        - match: $
          pop: true

  strings:
    - match: '"'
      scope: punctuation.definition.string.begin.aly
      push:
        - meta_scope: string.quoted.double.aly
        - match: '\\.'
          scope: constant.character.escape.aly
        - match: '"'
          scope: punctuation.definition.string.end.aly
          pop: true

  keywords:
    - match: '\b(let|const|fun|return|if|elif|else|loop|do|match|try|catch|finally|throw|break|async|await|struct|model|plugin|import)\b'
      scope: keyword.control.aly

  operators:
    - match: '\b(and|or|xor|not|eq|neq|lt|lte|gt|gte)\b'
      scope: keyword.operator.aly

  constants:
    - match: '\b(true|false)\b'
      scope: constant.language.boolean.aly
    - match: '\bNone\b'
      scope: constant.language.none.aly

  numbers:
    - match: '\b\d+\.\d+\b'
      scope: constant.numeric.float.aly
    - match: '\b\d+\b'
      scope: constant.numeric.integer.aly

  functions:
    - match: '\b[a-zA-Z_][a-zA-Z0-9_]*\s*(?=\()'
      scope: entity.name.function.aly
YAML

    success "Syntax criada: $syntax_dir/Aly.sublime-syntax"
    success "Suporte Sublime Text configurado!"
}

# ═══════════════════════════════════════════════════════════════════════════════
# JetBrains (IntelliJ / CLion / RustRover)
# ═══════════════════════════════════════════════════════════════════════════════
install_jetbrains() {
    info "Configurando suporte para JetBrains IDEs..."

    local os_type
    os_type="$(uname -s)"

    local config_dir
    case "$os_type" in
        Linux)
            config_dir="$HOME/.config/JetBrains"
            ;;
        Darwin)
            config_dir="$HOME/Library/Application Support/JetBrains"
            ;;
        *)
            error "Sistema operacional não suportado: $os_type"
            return 1
            ;;
    esac

    # Find latest JetBrains IDE directory
    local jetbrains_dir=""
    if [[ -d "$config_dir" ]]; then
        jetbrains_dir=$(find "$config_dir" -maxdepth 1 -type d -name "*.*" | sort -V | tail -1)
    fi

    if [[ -z "$jetbrains_dir" ]]; then
        warn "Nenhum diretório JetBrains encontrado em: $config_dir"
        echo -e "  ${DIM}Crie manualmente:${RESET}"
        echo -e "  ${CYAN}~/.config/JetBrains/<version>/filetypes/AlyFileType.xml${RESET}"
        return 1
    fi

    local filetypes_dir="$jetbrains_dir/filetypes"
    mkdir -p "$filetypes_dir"

    cat > "$filetypes_dir/AlyFileType.xml" << 'XML'
<fileTypes>
  <fileType name="Aly" language="Aly">
    <extensions>
      <extension name="aly" />
    </extensions>
    <filePattern>
      <pattern>\.aly$</pattern>
    </filePattern>
  </fileType>
</fileTypes>
XML

    success "FileType criado para JetBrains"
    echo -e "  ${DIM}Ou instale o plugin 'TextMate Bundles' e aponte para o editor/ do Aly${RESET}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Helix
# ═══════════════════════════════════════════════════════════════════════════════
install_helix() {
    info "Configurando suporte para Helix..."

    local hx_dir="${XDG_CONFIG_HOME:-$HOME/.config}/helix"
    local languages_dir="$hx_dir/languages.d"
    mkdir -p "$languages_dir"

    cat > "$languages_dir/aly.toml" << 'TOML'
language-server = "aly-lsp"
auto-format = false
formatter = { command = "aly", args = ["fmt"] }
roots = ["aly.toml"]

[[grammar]]
name = "aly"
source = { path = "" }
TOML

    success "Configuração Helix criada: $languages_dir/aly.toml"
    echo -e "  ${DIM}Helix usa Tree-sitter para syntax. Para suporte completo:${RESET}"
    echo -e "  ${CYAN}git clone https://github.com/user/tree-sitter-aly ~/.config/helix/runtime/grammars/sources/aly${RESET}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Zed
# ═══════════════════════════════════════════════════════════════════════════════
install_zed() {
    info "Configurando suporte para Zed..."

    local zed_dir="${XDG_CONFIG_HOME:-$HOME/.config}/zed"
    mkdir -p "$zed_dir"

    local themes_dir="$zed_dir/themes"
    mkdir -p "$themes_dir"

    # Create languages.json entry
    local languages_file="$zed_dir/languages.json"
    if [[ ! -f "$languages_file" ]]; then
        cat > "$languages_file" << 'JSON'
[
  {
    "name": "Aly",
    "grammar": "aly",
    "path_suffixes": ["aly"],
    "line_comments": ["# "]
  }
]
JSON
    fi

    success "Configuração Zed criada: $languages_file"
    echo -e "  ${DIM}Zed usa Tree-sitter para syntax highlighting${RESET}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# GNOME Builder / gtksourceview
# ═══════════════════════════════════════════════════════════════════════════════
install_gtksourceview() {
    info "Configurando GtkSourceView (GNOME Builder, gedit)..."

    local gs_dir="${XDG_DATA_HOME:-$HOME/.local/share}/gtksourceview-5/language-specs"
    mkdir -p "$gs_dir"

    cat > "$gs_dir/aly.lang" << 'XML'
<?xml version="1.0" encoding="UTF-8"?>
<language id="aly" name="Aly" version="20" _directory="aly">
  <metadata>
    <property name="mimetypes">text/x-aly</property>
    <property name="globs">*.aly</property>
  </metadata>
  <styles>
    <style id="comment" name="Comment" map-text="comment"/>
    <style id="keyword" name="Keyword" map-text="keyword"/>
    <style id="string" name="String" map-text="string"/>
    <style id="number" name="Number" map-text="literal"/>
    <style id="type" name="Type" map-text="type"/>
  </styles>
  <definitions>
    <context id="aly">
      <include>
        <context id="comment">
          <start>##</start>
          <end>##</end>
          <style>comment</style>
        </context>
        <context id="comment">
          <start>#</start>
          <end>$</end>
          <style>comment</style>
        </context>
        <context id="string">
          <start>"</start>
          <end>"</end>
          <style>string</style>
        </context>
        <context id="keyword">
          <match>\b(let|const|fun|return|if|elif|else|loop|do|match|try|catch|finally|throw|break|async|await|struct|model|plugin|import)\b</match>
          <style>keyword</style>
        </context>
        <context id="keyword">
          <match>\b(and|or|xor|not|eq|neq|lt|lte|gt|gte)\b</match>
          <style>keyword</style>
        </context>
        <context id="literal">
          <match>\b(true|false|None)\b</match>
          <style>literal</style>
        </context>
        <context id="literal">
          <match>\b\d+(\.\d+)?\b</match>
          <style>literal</style>
        </context>
      </include>
    </context>
  </definitions>
</language>
XML

    success "GtkSourceView language criado: $gs_dir/aly.lang"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Notepad++ (via Wine)
# ═══════════════════════════════════════════════════════════════════════════════
install_notepadpp() {
    info "Configurando suporte para Notepad++ (Wine)..."

    local nppp_dir="$HOME/.wine/drive_c/users/$USERNAME/AppData/Roaming/notepad++/userDefineLangs"
    if [[ ! -d "$nppp_dir" ]]; then
        nppp_dir="$HOME/.wine/drive_c/users/$(whoami)/AppData/Roaming/notepad++/userDefineLangs"
    fi

    if [[ ! -d "$nppp_dir" ]]; then
        warn "Notepad++ (Wine) não encontrado."
        echo -e "  ${DIM}Instale o Notepad++ via Wine primeiro:${RESET}"
        echo -e "  ${CYAN}wine notepad++-installer.exe${RESET}"
        return 1
    fi

    cat > "$nppp_dir/Aly.xml" << 'XML'
<NotepadPlus>
    <UserLang name="Aly" ext="aly">
        <Settings>
            <Global caseIgnored="no" />
        </Settings>
        <KeywordLists>
            <Keyword name="Funcionality" ignoreCase="no">
                <Word>let const fun return if elif else loop do match try catch finally throw break async await struct model plugin import</Word>
            </Keyword>
            <Keyword name="Operator" ignoreCase="no">
                <Word>and or xor not eq neq lt lte gt gte</Word>
            </Keyword>
        </KeywordLists>
        <Rules>
            <Keywords inGroup="0" styleID="1" name="Funcionality">
                <Words StyleID="1">let const fun return if elif else loop do match try catch finally throw break async await struct model plugin import</Words>
            </Keywords>
            <Keywords inGroup="1" styleID="10" name="Operator">
                <Words StyleID="10">and or xor not eq neq lt lte gt gte</Words>
            </Keywords>
            <Words StyleID="6">true false None</Words>
            <Comments>
                <LinePattern># $</LinePattern>
            </Comments>
        </Rules>
    </UserLang>
</NotepadPlus>
XML

    success "Notepad++ language criado: $nppp_dir/Aly.xml"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Nano syntax
# ═══════════════════════════════════════════════════════════════════════════════
install_nano() {
    info "Configurando syntax para Nano..."

    local nano_dir="${XDG_DATA_HOME:-$HOME/.local/share}/nano"
    mkdir -p "$nano_dir"

    cat > "$nano_dir/aly.nanorc" << 'NANORC'
## Aly language syntax highlighting for Nano

# Keywords
icolor brightgreen "\b(let|const|fun|return|if|elif|else|loop|do|match|try|catch|finally|throw|break|async|await|struct|model|plugin|import)\b"

# Operators
icolor cyan "\b(and|or|xor|not|eq|neq|lt|lte|gt|gte)\b"

# Boolean and None
icolor yellow "\b(true|false|None)\b"

# Strings
icolor magenta "\"[^"]*\""
icolor magenta "'[^']*'"

# Numbers
icolor brightred "\b[0-9]+\b"
icolor brightred "\b[0-9]+\.[0-9]+\b"

# Comments
icolor brightblue "#.*$"

# Math operators
icolor white "[+\-*/%|]"

# Assignment operators
icolor white "(\+\=|\-\=|\*\=|/\=|\%\=|\=)"
NANORC

    success "Nano syntax criado: $nano_dir/aly.nanorc"
    echo -e "  ${DIM}Adicione ao ~/.nanorc:${RESET}"
    echo -e "  ${CYAN}include \"/home/$USER/.local/share/nano/aly.nanorc\"${RESET}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Emacs
# ═══════════════════════════════════════════════════════════════════════════════
install_emacs() {
    info "Configurando suporte para Emacs..."

    local emacs_dir="${XDG_CONFIG_HOME:-$HOME/.emacs.d}"
    mkdir -p "$emacs_dir"

    cat > "$emacs_dir/aly-mode.el" << 'ELISP'
;;; aly-mode.el --- Aly language support for Emacs -*- lexical-binding: t; -*-

(defvar aly-mode-syntax-table
  (let ((table (make-syntax-table)))
    (modify-syntax-entry ?# "< " table)
    (modify-syntax-entry ?\n "> " table)
    table)
  "Syntax table for `aly-mode'.")

(defconst aly-keywords
  '("let" "const" "fun" "return" "if" "elif" "else"
    "loop" "do" "match" "try" "catch" "finally" "throw"
    "break" "async" "await" "struct" "model" "plugin" "import")
  "Aly keywords.")

(defconst aly-operators
  '("and" "or" "xor" "not" "eq" "neq" "lt" "lte" "gt" "gte")
  "Aly operators.")

(defconst aly-keywords-regexp (regexp-opt aly-keywords 'symbols))
(defconst aly-operators-regexp (regexp-opt aly-operators 'symbols))
(defconst aly-constants-regexp "\b\\(true\\|false\\|None\\)\\b")
(defconst aly-number-regexp "\b[0-9]+\\(\\.[0-9]+\\)?\\b")
(defconst aly-string-regexp "\"[^\"]*\"\\|'[^']*'")
(defconst aly-comment-regexp "#.*$")

(defvar aly-font-lock-keywords
  `((,aly-keywords-regexp . font-lock-keyword-face)
    (,aly-operators-regexp . font-lock-builtin-face)
    (,aly-constants-regexp . font-lock-constant-face)
    (,aly-number-regexp . font-lock-constant-face)
    (,aly-string-regexp . font-lock-string-face)
    (,aly-comment-regexp . font-lock-comment-face))
  "Font lock keywords for `aly-mode'.")

;;;###autoload
(define-derived-mode aly-mode prog-mode "Aly"
  "Major mode for editing Aly source code."
  :syntax-table aly-mode-syntax-table
  (setq font-lock-defaults '(aly-font-lock-keywords)))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.aly\\'" . aly-mode))

(provide 'aly-mode)
;;; aly-mode.el ends here
ELISP

    success "Emacs mode criado: $emacs_dir/aly-mode.el"
    echo -e "  ${DIM}Adicione ao seu init.el:${RESET}"
    echo -e "  ${CYAN}(load \"~/.emacs.d/aly-mode\")${RESET}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Menu principal
# ═══════════════════════════════════════════════════════════════════════════════
show_menu() {
    echo -e "  ${BOLD}Escolha o editor/IDE:${RESET}"
    echo ""
    echo -e "  ${GREEN}1)${RESET}  VS Code          ${DIM}(recomendado — extensão completa)${RESET}"
    echo -e "  ${GREEN}2)${RESET}  Cursor           ${DIM}(VS Code fork, mesma base)${RESET}"
    echo -e "  ${GREEN}3)${RESET}  Neovim / Vim     ${DIM}(syntax + ftdetect)${RESET}"
    echo -e "  ${GREEN}4)${RESET}  Sublime Text     ${DIM}(syntax definition)${RESET}"
    echo -e "  ${GREEN}5)${RESET}  JetBrains        ${DIM}(IntelliJ/CLion/RustRover)${RESET}"
    echo -e "  ${GREEN}6)${RESET}  Helix            ${DIM}(tree-sitter config)${RESET}"
    echo -e "  ${GREEN}7)${RESET}  Zed              ${DIM}(tree-sitter config)${RESET}"
    echo -e "  ${GREEN}8)${RESET}  GNOME Builder    ${DIM}(GtkSourceView lang)${RESET}"
    echo -e "  ${GREEN}9)${RESET}  Emacs            ${DIM}(major mode)${RESET}"
    echo -e "  ${GREEN}10)${RESET} Nano             ${DIM}(syntax highlighting)${RESET}"
    echo -e "  ${GREEN}11)${RESET} Notepad++        ${DIM}(via Wine)${RESET}"
    echo ""
    echo -e "  ${GREEN}a)${RESET}  Instalar ${BOLD}TODOS${RESET}"
    echo -e "  ${GREEN}q)${RESET}  Sair"
    echo ""
    echo -en "  ${BOLD}Opção: ${RESET}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Main
# ═══════════════════════════════════════════════════════════════════════════════
main() {
    banner

    while true; do
        show_menu
        read -r choice

        case "$choice" in
            1)  install_vscode ;;
            2)  install_cursor ;;
            3)  install_neovim ;;
            4)  install_sublime ;;
            5)  install_jetbrains ;;
            6)  install_helix ;;
            7)  install_zed ;;
            8)  install_gtksourceview ;;
            9)  install_emacs ;;
            10) install_nano ;;
            11) install_notepadpp ;;
            a|A)
                echo ""
                install_vscode 2>/dev/null || true
                install_neovim 2>/dev/null || true
                install_sublime 2>/dev/null || true
                install_jetbrains 2>/dev/null || true
                install_helix 2>/dev/null || true
                install_zed 2>/dev/null || true
                install_gtksourceview 2>/dev/null || true
                install_emacs 2>/dev/null || true
                install_nano 2>/dev/null || true
                ;;
            q|Q)
                echo ""
                success "Até logo! 🚀"
                exit 0
                ;;
            *)
                error "Opção inválida: $choice"
                ;;
        esac

        echo ""
        line
        echo ""
        echo -en "  ${DIM}Pressione Enter para continuar...${RESET}"
        read -r
        banner
    done
}

main "$@"

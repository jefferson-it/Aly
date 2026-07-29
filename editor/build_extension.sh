#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# Aly Language - VS Code Extension Builder
# Empacula a extensão em .vsix e instala no VS Code
# ═══════════════════════════════════════════════════════════════════════════════
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="/tmp/aly-vscode-ext"
VSIX_NAME="aly-lang-0.1.0.vsix"
VSIX_TEMP="/tmp/aly-lang-build.vsix"
VSIX_PATH="/tmp/$VSIX_NAME"

RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
BOLD='\033[1m'
DIM='\033[2m'
RESET='\033[0m'

info()    { echo -e "${CYAN}[INFO]${RESET}  $*"; }
success() { echo -e "${GREEN}[OK]${RESET}    $*"; }
error()   { echo -e "${RED}[ERROR]${RESET} $*"; }

echo -e "${CYAN}${BOLD}"
cat << 'EOF'
  _                 _
 / \   __ _  __ _(_) ___  ___
/ _ \ / _` |/ _` | |/ _ \/ __|
/ ___ \ (_| | (_| | | (_) \__ \
/_/   \_\__, |\__, |_|\___/|___/
        |___/   VS Code Builder
EOF
echo -e "${RESET}"
echo ""

# ── 1. Limpar build anterior ──────────────────────────────────────────────
info "Limpando build anterior..."
rm -rf "$BUILD_DIR"
rm -f "$VSIX_PATH"
rm -f "$VSIX_TEMP"

# ── 2. Criar estrutura do .vsix ───────────────────────────────────────────
info "Criando estrutura do pacote..."
mkdir -p "$BUILD_DIR/extension"

# Copiar arquivos necessários
cp "$SCRIPT_DIR/package.json"        "$BUILD_DIR/extension/"
cp "$SCRIPT_DIR/extension.js"        "$BUILD_DIR/extension/"
cp "$SCRIPT_DIR/language-configuration.json" "$BUILD_DIR/extension/"
cp -r "$SCRIPT_DIR/syntaxes"         "$BUILD_DIR/extension/"
cp -r "$SCRIPT_DIR/snippets"         "$BUILD_DIR/extension/"

# Criar [Content_Types].xml (obrigatório para .vsix)
cat > "$BUILD_DIR/[Content_Types].xml" << 'XML'
<?xml version="1.0" encoding="utf-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension=".json" ContentType="application/json"/>
  <Default Extension=".js" ContentType="application/javascript"/>
  <Default Extension=".tmLanguage" ContentType="text/plain"/>
  <Default Extension=".xml" ContentType="application/xml"/>
  <Default Extension=".md" ContentType="text/markdown"/>
</Types>
XML

# Criar extension.vsixmanifest
mkdir -p "$BUILD_DIR/META-INF"
cat > "$BUILD_DIR/META-INF/manifest.xml" << 'XML'
<?xml version="1.0" encoding="utf-8"?>
<PackageManifest Version="2.0.0"
  xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011"
  xmlns:d="http://schemas.microsoft.com/developer/vsx-schema-design/2011">
  <Metadata>
    <Identity Language="en-US"
      Id="aly-lang"
      Version="0.1.0"
      Publisher="jefferson-it" />
    <DisplayName>Aly Programming Language</DisplayName>
    <Description>VS Code support for Aly: syntax highlighting, autocompletion, snippets.</Description>
    <Tags>aly,programming language,syntax highlighting</Tags>
    <Categories>Programming Languages,Snippets</Categories>
  </Metadata>
  <Installation>
    <InstallationTarget Id="Microsoft.VisualStudio.Code" />
  </Installation>
  <Dependencies />
  <Assets>
    <Asset Type="Microsoft.VisualStudio.Code.Extension" Path="extension" Addressable="true" />
  </Assets>
</PackageManifest>
XML

# ── 3. Criar .vsix (é um ZIP) ─────────────────────────────────────────────
info "Empacotando .vsix..."
cd "$BUILD_DIR"

# Criar o zip
zip -r "$VSIX_TEMP" \
    "[Content_Types].xml" \
    META-INF/manifest.xml \
    extension/ \
    -q

cd "$SCRIPT_DIR"

# Renomear para o nome final
mv "$VSIX_TEMP" "$VSIX_PATH"

success "Pacote criado: /tmp/$VSIX_NAME"
echo ""

# ── 4. Instalar no VS Code ────────────────────────────────────────────────
# Procurar VS Code
VSCODE_CMD=""
for cmd in code code-insiders code-oss; do
    if command -v "$cmd" &>/dev/null; then
        VSCODE_CMD="$cmd"
        break
    fi
done

if [[ -z "$VSCODE_CMD" ]]; then
    error "VS Code não encontrado no PATH."
    echo ""
    echo -e "  ${DIM}Instale manualmente:${RESET}"
    echo -e "  ${CYAN}code --install-extension /tmp/$VSIX_NAME${RESET}"
    exit 1
fi

success "VS Code encontrado: $VSCODE_CMD"
echo ""

# Verificar se já está instalado (remover versão anterior)
info "Removendo versão anterior (se existir)..."
"$VSCODE_CMD" --uninstall-extension "jefferson-it.aly-lang" 2>/dev/null || true

# Instalar
info "Instalando extensão..."
"$VSCODE_CMD" --install-extension "/tmp/$VSIX_NAME" --force

if [[ $? -eq 0 ]]; then
    echo ""
    success "Extensão instalada com sucesso!"
    echo ""
    echo -e "  ${DIM}Reinicie o VS Code para ativar.${RESET}"
    echo -e "  ${DIM}Abra um arquivo .aly para ver a syntax highlighting.${RESET}"
else
    echo ""
    error "Falha na instalação. Tente manualmente:"
    echo -e "  ${CYAN}code --install-extension /tmp/$VSIX_NAME${RESET}"
fi

# Limpar
rm -rf "$BUILD_DIR"

echo ""

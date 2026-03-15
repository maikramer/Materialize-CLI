#!/bin/bash
# =============================================================================
# Materialize CLI Installer - Linux/macOS
# =============================================================================
#
# Instala o Materialize CLI (gera mapas PBR a partir de texturas).
# Requer: Python 3 (para o instalador), Rust/cargo (para compilar).
#
# Uso:
#   ./install.sh              # Instalação em ~/.local/bin
#   ./install.sh uninstall    # Desinstalação
#   ./install.sh reinstall    # Reinstalação
#
# =============================================================================

set -e

cd "$(dirname "$0")"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}🚀 Materialize CLI Installer${NC}"
echo "================================"

if ! command -v python3 &> /dev/null; then
    echo -e "${RED}✗ Python 3 não encontrado${NC}"
    echo "Instale Python 3 (necessário para rodar o instalador):"
    echo "  Ubuntu/Debian: sudo apt install python3"
    echo "  macOS: brew install python3"
    exit 1
fi

echo -e "${GREEN}✓ Python 3 encontrado${NC}"

python3 installer/installer.py "$@"

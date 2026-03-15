# Materialize CLI

CLI em Rust que gera mapas PBR (Height, Normal, Metallic) a partir de texturas, usando compute shaders GPU (wgpu).

## Instalação rápida

```bash
git clone https://github.com/seu-user/materialize-cli
cd materialize-cli
./install.sh
```

Requisitos: **Python 3** (instalador) e **Rust** (cargo) para compilar. O instalador coloca o binário em `~/.local/bin/materialize`.

- **Linux/macOS:** `./install.sh` | `./install.sh uninstall` | `./install.sh reinstall`
- **Windows:** `.\install.ps1` ou `install.bat`

## Uso

```bash
materialize texture.png -o ./out/ -v
# Gera: texture_height.png, texture_normal.png, texture_metallic.png
```

## Documentação

Veja [docs/README.md](docs/README.md) para visão geral, instalação detalhada e links para arquitetura, algoritmos e roadmap.

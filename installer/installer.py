#!/usr/bin/env python3
"""
Materialize CLI Installer - Instalação multiplataforma
Modelo: mesmo estilo denv-cli / pc-cli / Galaxy (installer Python, binário Rust)
"""

import os
import sys
import shutil
import subprocess
import platform
from pathlib import Path
from typing import Optional

# Nome do binário no Cargo (target/release/materialize-cli)
CARGO_BIN_NAME = "materialize-cli"
# Nome do comando no PATH
CLI_NAME = "materialize"


class Colors:
    """Cores para terminal"""
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'


class Installer:
    """Instalador multiplataforma do Materialize CLI (Rust)"""

    def __init__(self):
        # Diretório do instalador (installer/) e raiz do repo (Materialize-CLI/)
        self.installer_dir = Path(__file__).parent.resolve()
        self.repo_dir = self.installer_dir.parent.resolve()
        self.platform = platform.system().lower()
        self.is_windows = self.platform == 'windows'
        self.is_macos = self.platform == 'darwin'
        self.is_linux = self.platform == 'linux'

        # Caminho do binário após build
        self.release_binary = self.repo_dir / "target" / "release" / CARGO_BIN_NAME
        self.debug_binary = self.repo_dir / "target" / "debug" / CARGO_BIN_NAME

        # Diretório de instalação (binários no PATH)
        if self.is_windows:
            user_profile = os.environ.get('USERPROFILE') or 'C:\\'
            self.bin_dir = Path(user_profile) / 'bin'
        else:
            self.bin_dir = Path.home() / '.local' / 'bin'

        self.installed_binary = self.bin_dir / CLI_NAME

    def print_header(self, text: str):
        """Print header formatado"""
        print(f"\n{Colors.BOLD}{Colors.OKCYAN}{text}{Colors.ENDC}")
        print("=" * len(text))

    def print_success(self, text: str):
        print(f"{Colors.OKGREEN}✓ {text}{Colors.ENDC}")

    def print_error(self, text: str):
        print(f"{Colors.FAIL}✗ {text}{Colors.ENDC}")

    def print_warning(self, text: str):
        print(f"{Colors.WARNING}⚠ {text}{Colors.ENDC}")

    def print_info(self, text: str):
        print(f"{Colors.OKBLUE}ℹ {text}{Colors.ENDC}")

    def check_python(self) -> bool:
        """Verifica Python 3 (para rodar este instalador)"""
        self.print_header("Verificando dependências")
        try:
            result = subprocess.run(
                [sys.executable, "--version"],
                capture_output=True,
                text=True,
                check=True,
                timeout=5,
            )
            self.print_success(f"Python: {result.stdout.strip()}")
            return True
        except Exception as e:
            self.print_error(f"Python não encontrado: {e}")
            return False

    def check_cargo(self) -> bool:
        """Verifica se Rust/cargo está instalado"""
        try:
            result = subprocess.run(
                ["cargo", "--version"],
                capture_output=True,
                text=True,
                check=True,
                timeout=5,
            )
            self.print_success(f"Rust: {result.stdout.strip()}")
            return True
        except (subprocess.CalledProcessError, FileNotFoundError):
            self.print_warning("cargo não encontrado no PATH")
            return False

    def get_existing_binary(self) -> Optional[Path]:
        """Retorna o caminho do binário se já existir (release ou debug)."""
        if self.release_binary.exists():
            return self.release_binary
        if self.debug_binary.exists():
            return self.debug_binary
        return None

    def build_release(self) -> bool:
        """Executa cargo build --release."""
        self.print_header("Compilando Materialize CLI (release)")
        try:
            subprocess.run(
                ["cargo", "build", "--release"],
                cwd=self.repo_dir,
                check=True,
                timeout=600,
            )
            self.print_success("Build concluído: target/release/" + CARGO_BIN_NAME)
            return True
        except subprocess.CalledProcessError as e:
            self.print_error(f"Falha no build: {e}")
            return False
        except FileNotFoundError:
            self.print_error("cargo não encontrado. Instale Rust: https://rustup.rs")
            return False
        except subprocess.TimeoutExpired:
            self.print_error("Build excedeu o tempo limite")
            return False

    def create_bin_dir(self) -> bool:
        """Cria diretório de binários."""
        try:
            self.bin_dir.mkdir(parents=True, exist_ok=True)
            self.print_success(f"Diretório: {self.bin_dir}")
            return True
        except Exception as e:
            self.print_error(f"Não foi possível criar diretório: {e}")
            return False

    def install_binary(self) -> bool:
        """Copia ou linka o binário para o diretório no PATH."""
        self.print_header("Instalando binário")

        src = self.get_existing_binary()
        if not src:
            if not self.check_cargo():
                return False
            if not self.build_release():
                return False
            src = self.release_binary

        if not src.exists():
            self.print_error(f"Binário não encontrado: {src}")
            return False

        try:
            dest = self.installed_binary
            if dest.exists() or dest.is_symlink():
                dest.unlink()
            shutil.copy2(src, dest)
            dest.chmod(0o755)
            self.print_success(f"{CLI_NAME} instalado em {dest}")
            return True
        except Exception as e:
            self.print_error(f"Erro ao instalar: {e}")
            return False

    def update_path(self) -> bool:
        """Informa sobre PATH."""
        self.print_header("Configurando PATH")
        bin_str = str(self.bin_dir)
        path_env = os.environ.get('PATH', '')

        if self.is_windows:
            path_sep = ';'
        else:
            path_sep = ':'

        if bin_str in path_env:
            self.print_success(f"{bin_str} já está no PATH")
            return True

        self.print_warning(f"{bin_str} pode não estar no PATH")
        if not self.is_windows:
            self.print_info("Adicione ao ~/.bashrc ou ~/.zshrc:")
            self.print_info(f'  export PATH="{bin_str}:$PATH"')
        else:
            self.print_info(f"Adicione manualmente ao PATH do sistema: {bin_str}")
        return False

    def test_installation(self) -> bool:
        """Testa o comando instalado."""
        self.print_header("Testando instalação")
        try:
            result = subprocess.run(
                [str(self.installed_binary), "--version"],
                capture_output=True,
                text=True,
                timeout=10,
            )
            if result.returncode == 0:
                self.print_success(f"Versão: {result.stdout.strip()}")
                return True
            self.print_warning("Teste retornou código não zero")
            return True
        except Exception as e:
            self.print_warning(f"Não foi possível testar: {e}")
            return True

    def uninstall(self) -> bool:
        """Remove o binário do diretório de instalação."""
        self.print_header("Desinstalando Materialize CLI")
        try:
            for name in [CLI_NAME]:
                p = self.bin_dir / name
                if p.exists():
                    p.unlink()
                    self.print_success(f"Removido: {p}")
            self.print_success("Materialize CLI desinstalado.")
            return True
        except Exception as e:
            self.print_error(f"Erro ao desinstalar: {e}")
            return False

    def install(self) -> bool:
        """Executa instalação completa."""
        self.print_header("Materialize CLI Installer")
        print(f"Plataforma: {platform.system()}")
        print(f"Repositório: {self.repo_dir}")
        print(f"Binários em: {self.bin_dir}")

        if not self.check_python():
            return False
        if not self.create_bin_dir():
            return False
        if not self.install_binary():
            return False

        self.update_path()
        self.test_installation()

        self.print_header("Instalação concluída")
        print("\nUso:")
        print(f"  {CLI_NAME} texture.png -o ./out/   # Gerar mapas PBR")
        print(f"  {CLI_NAME} --help                   # Ajuda")
        if not self.is_windows:
            print(f"\nSe '{CLI_NAME}' não for encontrado, adicione ao PATH:")
            print(f'  export PATH="{self.bin_dir}:$PATH"')
        return True


def main():
    import argparse

    parser = argparse.ArgumentParser(
        description="Materialize CLI Installer (Rust)",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Exemplos:
  ./install.sh              # Instala (Linux/macOS)
  python3 installer/installer.py install
  python3 installer/installer.py uninstall
  python3 installer/installer.py reinstall
        """,
    )
    parser.add_argument(
        'action',
        nargs='?',
        default='install',
        choices=['install', 'uninstall', 'reinstall'],
        help='Ação (padrão: install)',
    )
    args = parser.parse_args()

    installer = Installer()

    if args.action == 'install':
        success = installer.install()
    elif args.action == 'uninstall':
        success = installer.uninstall()
    elif args.action == 'reinstall':
        installer.uninstall()
        success = installer.install()
    else:
        success = False

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()

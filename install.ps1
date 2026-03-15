# Materialize CLI Installer - Windows PowerShell
#
# Instala o Materialize CLI (gera mapas PBR a partir de texturas).
# Requisitos: PowerShell 5.1+, Python 3 (instalador), Rust/cargo (compilar).
#
# Uso:
#   .\install.ps1              # Instalação
#   .\install.ps1 uninstall   # Desinstalação
#   .\install.ps1 reinstall   # Reinstalação
#

param(
    [Parameter(Position=0)]
    [ValidateSet('install', 'uninstall', 'reinstall')]
    [string]$Action = 'install'
)

$ErrorActionPreference = "Stop"

$Cyan = "`e[36m"
$Green = "`e[32m"
$Red = "`e[31m"
$Reset = "`e[0m"

Write-Host "${Cyan}🚀 Materialize CLI Installer${Reset}"
Write-Host "================================"

$pythonCmd = Get-Command python -ErrorAction SilentlyContinue
if (-not $pythonCmd) {
    $pythonCmd = Get-Command python3 -ErrorAction SilentlyContinue
}
if (-not $pythonCmd) {
    Write-Host "${Red}✗ Python não encontrado${Reset}"
    Write-Host "Instale Python 3 de https://python.org"
    exit 1
}

Write-Host "${Green}✓ Python: $($pythonCmd.Source)${Reset}"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $scriptDir

& $pythonCmd.Source installer/installer.py $Action

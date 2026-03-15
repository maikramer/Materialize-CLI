@echo off
REM Materialize CLI Installer - Windows CMD
REM Instala o Materialize CLI (gera mapas PBR a partir de texturas).

setlocal EnableDelayedExpansion

echo Materialize CLI Installer
echo ================================

python --version >nul 2>&1
if errorlevel 1 (
    python3 --version >nul 2>&1
    if errorlevel 1 (
        echo Python nao encontrado. Instale Python 3 de https://python.org
        exit /b 1
    )
    set PY=python3
) else (
    set PY=python
)

echo OK Python encontrado

cd /d "%~dp0"
%PY% installer\installer.py %*

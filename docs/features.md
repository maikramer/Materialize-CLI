# Funcionalidades

## Geração de Mapas PBR

### Height Map

Converte uma imagem colorida em um mapa de altura em escala de cinza.

**Entrada:** Imagem difusa (RGBA)
**Saída:** Height map (grayscale, R32Float internamente)

**Algoritmo:**
1. Converter para luminância (grayscale)
2. Aplicar Gaussian blur em múltiplos níveis
3. Combinar com pesos ajustáveis
4. Aplicar contraste e spread

**Uso típico:**
- Displacement mapping
- Parallax occlusion mapping
- Base para normal map generation

### Normal Map

Gera vetores de superfície a partir do height map.

**Entrada:** Height map (gerado internamente)
**Saída:** Normal map (RGB, formato DirectX/OpenGL)

**Algoritmo:**
1. Calcular gradientes via operador Sobel
2. Construir vetores normais (x, y, z)
3. Normalizar e codificar para [0, 255]

**Formato de saída:**
- Red channel: X component (-1 to +1) → 0 to 255
- Green channel: Y component (-1 to +1) → 0 to 255  
- Blue channel: Z component (0 to +1) → 128 to 255

**Uso típico:**
- Iluminação em tempo real
- Bump mapping
- Detalhes de superfície sem geometria adicional

### Metallic Map

Detecta áreas metálicas por análise de cor.

**Entrada:** Imagem difusa original (RGBA)
**Saída:** Metallic map (grayscale, 0 = dieletric, 1 = metal)

**Algoritmo:**
1. Converter RGB para espaço HSL
2. Analisar saturação e luminância
3. Aplicar heurística de detecção metálica

**Heurística:**
- Metais puros têm saturação baixa e luminância alta
- Ouro/cobre têm matiz específico
- Thresholds adaptativos baseados na imagem

**Uso típico:**
- Physically based rendering (PBR)
- Diferenciação metal/não-metal
- Reflexos específicos por material

## Formatos Suportados

### Entrada (Leitura)

| Formato | Extensões | Observações |
|---------|-----------|-------------|
| PNG | .png | Recomendado, lossless |
| JPEG | .jpg, .jpeg | Lossy, bom para fotos |
| TGA | .tga | Legacy, games |
| BMP | .bmp | Limitado, suportado |
| EXR | .exr | HDR, linear |

### Saída (Escrita)

| Formato | Extensões | Melhor uso |
|---------|-----------|------------|
| PNG | .png | Geral (lossless) |
| JPEG | .jpg | Compacto (lossy) |
| TGA | .tga | Games engines |
| EXR | .exr | Normal maps (float precision) |

**Configurações de qualidade:**
- PNG: Compression level 6 (padrão)
- JPEG: Qualidade 95% (padrão), configurável 0-100
- EXR: P-tiles, zip compression

## Interface CLI

### Comando Básico

```bash
materialize <INPUT> [OPTIONS]
```

### Opções

| Opção | Curta | Descrição | Padrão |
|-------|-------|-----------|--------|
| `--output` | `-o` | Diretório de saída | Mesmo diretório do input |
| `--format` | `-f` | Formato de saída | png |
| `--quality` | `-q` | Qualidade JPEG (0-100) | 95 |
| `--prefix` | `-p` | Prefixo dos arquivos de saída | nome do input |
| `--verbose` | `-v` | Modo verbose | false |
| `--help` | `-h` | Mostrar ajuda | - |
| `--version` | `-V` | Mostrar versão | - |

### Exemplos de Uso

```bash
# Básico - gera na mesma pasta
materialize texture.png
# Resultado: texture_height.png, texture_normal.png, texture_metallic.png

# Diretório de saída específico
materialize texture.png -o ./materials/
# Resultado: ./materials/texture_height.png, etc.

# Formato diferente
materialize texture.png -f exr
# Resultado: texture_height.exr (melhor para precisão)

# JPEG com qualidade baixa (mais compacto)
materialize texture.jpg -f jpg -q 80

# Prefixo customizado
materialize texture.png -p brick_wall
# Resultado: brick_wall_height.png, brick_wall_normal.png, etc.

# Verbose - mostra progresso
materialize texture.png -v
# Output: Loading texture.png... 2048x2048
#         Processing height map... done (45ms)
#         Processing normal map... done (12ms)
#         Processing metallic map... done (18ms)
#         Saving outputs... done
```

## Features Futuras (Roadmap)

### Versão 1.1 - Smoothness

- **Smoothness Map:** Similar ao metallic, detecta rugosidade da superfície
- **One Roughness:** Parâmetro global de roughness

### Versão 1.2 - Batch Processing

- **Diretório como input:** `materialize ./textures/`
- **Paralelização:** Processa múltiplas imagens simultaneamente
- **Progress bar:** Indicação visual de progresso

### Versão 2.0 - AO (Ambient Occlusion)

- **AO Map:** Oclusão ambiente via ray marching
- **Configurações:** Ray count, max distance, spread

### Versão 2.1 - Configuração Avançada

- **Arquivo de configuração:** `materialize.toml` para parâmetros customizados
- **Override por mapa:** `--height-blur=5.0 --normal-intensity=2.5`

### Versão 3.0 - Preview

- **Janela de preview:** SDL2-based, visualização 3D rápida
- **Rotação/Pan/Zoom:** Navegação básica

## Casos de Uso

### Game Development

```bash
# Pipeline de assets
for texture in assets/textures/raw/*.png; do
    materialize "$texture" -o assets/textures/processed/
done
```

### 3D Art / Blender

```bash
# Preparar textura para importação
materialize photo.jpg -f exr -o ~/blender_project/textures/
# Importar height para displacement, normal para bump, metallic para shader
```

### Web Development (Three.js/Babylon)

```bash
# Otimizar para web (JPEG compacto)
materialize texture.png -f jpg -q 85 -o ./public/textures/
```

### Archviz

```bash
# Materiais de alta qualidade
materialize marble_scan.png -f exr -o ./materials/marble/
# EXR preserva precisão de cor para materiais PBR
```

## Limitações Conhecidas

### MVP (Versão 1.0)

1. **Parâmetros fixos:** Defaults hardcoded, sem ajuste fino
2. **Resolução máxima:** Limitada por GPU memory (tipicamente 8K+)
3. **Um formato por execução:** Todos os outputs no mesmo formato
4. **Sem alpha handling:** Canal alpha ignorado no processamento

### Futuro

Limitações serão endereçadas conforme roadmap (configuração, tiled processing, etc.)

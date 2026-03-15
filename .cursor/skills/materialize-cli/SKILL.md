---
name: materialize-cli
description: Generates PBR maps (height, normal, metallic, smoothness, edge, AO) from diffuse textures via wgpu compute shaders. Use when working with Materialize CLI, PBR texture baking, diffuse-to-material pipeline, WGSL shaders, or when the user mentions materialize-cli, PBR maps, or the six output maps.
---

# Materialize CLI

## When to use this skill

- User asks about the CLI (usage, options, output files)
- Adding or changing a PBR map (shader + pipeline + I/O)
- Debugging or extending the GPU pipeline (wgpu, bind groups, formats)
- Writing or updating docs (README, docs/, roadmap)

## CLI overview

**Command:** `materialize <input> [options]`

**Outputs (6 maps):** `{stem}_height.*`, `_normal.*`, `_metallic.*`, `_smoothness.*`, `_edge.*`, `_ao.*`

**Options:** `-o`/`--output` (dir), `-f`/`--format` (png|jpg|tga|exr), `-q`/`--quality` (0-100, JPEG), `-v`/`--verbose`, `--quiet` (no file list on success).

**Examples:**
```bash
materialize texture.png -o ./out/ -v
materialize diffuse.png --format png --quiet
materialize skill install   # install this skill into current project's .cursor/skills
```

## Code layout

| Area | Path | Role |
|------|------|------|
| Shaders | `src/shaders/*.wgsl` | One compute shader per map; 8×8 workgroup |
| Pipeline | `src/pipeline.rs` | Order: height → normal → metallic → smoothness → edge → AO; bind groups and readback |
| GPU | `src/gpu.rs` | `create_compute_pipeline` (1 input), `create_compute_pipeline_2_inputs` (2 inputs), `create_bind_group` / `create_bind_group_2_inputs` |
| I/O | `src/io.rs` | `get_output_paths`, `*_to_image`, `save_image`, `load_image` |
| CLI | `src/cli.rs` | clap args; `OutputFormat` enum |
| Main | `src/main.rs` | Load image → `pipeline.process()` → convert maps → `get_output_paths` → save all 6 |

## Map dependencies

- **Height:** from diffuse (Rgba8Unorm → R32Float).
- **Normal:** from height (R32Float, not filterable).
- **Metallic:** from diffuse.
- **Smoothness:** from diffuse + metallic (2-input pipeline).
- **Edge:** from normal.
- **AO:** from height.

## Adding or changing a map

1. **Shader:** Add `src/shaders/<name>.wgsl` with `@group(0) @binding(0)` input and `@binding(1)` storage output (or bindings 0,1,2 for 2 inputs). Workgroup size 8×8. Guard with `coords >= dims` return.
2. **Pipeline:** In `pipeline.rs`, add constant `include_str!("shaders/<name>.wgsl")`, create pipeline (1-input or 2-input), create output texture and bind group, dispatch after dependencies, read back and push into `PbrMaps`.
3. **I/O:** In `io.rs`, add `*_path` to `OutputPaths`, extend `get_output_paths`, add `*_to_image` and call `save_image` in `main.rs` with the new path.
4. **Formats:** Height/normal use R32Float for height; normal/metallic/smoothness/edge/ao use Rgba8Unorm (grayscale maps use R channel only when saving).

## Testing

```bash
cargo build
cargo test
# Optional: run on a real image
materialize /path/to/diffuse.png -o ./out/ -v
```

Integration tests: `tests/integration_test.rs` (help, version, file not found). Unit tests in `io::tests`.

## Documentation

- **User-facing:** `README.md`, `docs/README.md`, `docs/features.md`, `docs/cli-api.md`
- **Technical:** `docs/architecture.md`, `docs/algorithms.md`, `docs/shaders.md`
- **Planning:** `docs/roadmap.md`, `docs/plans/*.md`

Keep docs in sync when adding maps or CLI options (e.g. list all 6 outputs and `--quiet`).

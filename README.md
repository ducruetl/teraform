# Teraform

A procedural terrain generator written in Rust. Teraform builds heightmaps from fractal Brownian motion (fBm) over value noise and exports them as PGM (grayscale heightmap) or PPM (colored terrain) images.

## Features

- Value noise with a custom integer hash (deterministic, seed-based)
- fBm with configurable octaves, persistence and lacunarity
- Two terrain types: `flat` and `island`
- No image library needed: files are written as plain-text

## Installation

**Requirements:** Rust (stable), installed via [rustup](https://rustup.rs/)

```bash
git clone https://github.com/ducruetl/teraform
cd teraform
cargo build --release
```

The binary is then available at `target/release/teraform`.

## Usage

```bash
teraform [OPTIONS]
```

Example:

```bash
teraform \
    --width 512 \
    --height 512 \
    --seed 42 \
    --octaves 6 \
    --scale 80 \
    --persistence 0.5 \
    --lacunarity 2.0 \
    --type island \
    --output terrain.ppm
```

Every option has a default, so `teraform` alone also works.

### Options

| Option          | Default       | Description                                                                 |
| --------------- | ------------- | --------------------------------------------------------------------------- |
| `--width`       | `512`         | Image width in pixels (≥ 1)                                                 |
| `--height`      | `512`         | Image height in pixels (≥ 1)                                                |
| `--seed`        | `0`           | Random seed. The same seed always gives the same terrain                    |
| `--octaves`     | `6`           | Number of noise layers (1 to 16). More layers add finer detail              |
| `--scale`       | `80.0`        | Size in pixels of the largest features. Bigger values mean bigger landforms |
| `--persistence` | `0.5`         | Amplitude multiplier per octave. Lower is smoother, higher is rougher       |
| `--lacunarity`  | `2.0`         | Frequency multiplier per octave                                             |
| `--type`        | `flat`        | Terrain shape: `flat` or `island`                                           |
| `--output`      | `terrain.ppm` | Output file. Supported extensions: `.pgm` and `.ppm`                        |

Run `teraform --help` for the full list with descriptions.

### Output formats

| Extension | Content                                                              |
| --------- | -------------------------------------------------------------------- |
| `.ppm`    | Colored terrain: water, sand, grass and rock, based on height        |
| `.pgm`    | Grayscale heightmap: black is the lowest point, white is the highest |

### Terrain types

- **`flat`**: plain terrain with no edge falloff.
- **`island`**: heights are pulled down toward the edges, so the land is surrounded by ocean.

## Project structure

```
teraform/
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── src/
    ├── main.rs      # CLI parsing and function calls
    ├── noise.rs     # hash, fade, lerp, value noise, fBm
    ├── terrain.rs   # island mask, color palette for terrain
    ├── config.rs    # arguments definition and parsing
    └── export.rs    # PGM / PPM writers
```

## Dependencies

- [`clap`](https://crates.io/crates/clap): command-line argument parsing

## Author

Léo DUCRUET, ducruet.leo1@gmail.com

<div align="center">

# [LXMFace](https://ratspeak.org/)

**Deterministic visual identities for [LXMF](https://github.com/markqvist/lxmf) / [Reticulum](https://reticulum.network/) addresses**

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85+-DEA584.svg)](https://www.rust-lang.org/)
[![JavaScript](https://img.shields.io/badge/javascript-ES5+-F7DF1E.svg)](js/lxmface.js)
[![Status: Stable](https://img.shields.io/badge/status-stable-brightgreen.svg)](https://github.com/ratspeak/lxmface)

<br>

<img src="assets/identicons.svg" alt="LXMFace identicons generated from different LXMF addresses" width="800">

<br>
<br>

</div>

LXMFace gives every LXMF address a unique, recognizable face. The same address always produces the same avatar — across any app that uses LXMFace.

It works like the original [blockies](https://github.com/download13/blockies): feed in an address hash, get back a deterministic symmetrical identicon. No server, no image files, no lookups. The output is a self-contained SVG string.

Available as a **Rust crate** and a **JavaScript library**. Both produce identical output, validated by shared test vectors.

## Usage

### Rust

```toml
[dependencies]
lxmface = "0.1"
```

```rust
use lxmface::lxmface_svg;

// Generate an SVG string from an LXMF address hash
let svg = lxmface_svg("a7b3c9d1e5f20681943ab2de77fc8e01", 64);
```

Zero runtime dependencies. `no_std` compatible (requires `alloc`).

If you need the raw colors and grid for custom rendering (native canvas, terminal, pixel buffer):

```rust
let data = lxmface::lxmface_data("a7b3c9d1e5f20681943ab2de77fc8e01");
// data.color, data.bgcolor, data.spotcolor — HslColor { h, s, l }
// data.grid — [[u8; 8]; 8], where 0=bg, 1=primary, 2=spot
```

### JavaScript

```html
<script src="lxmface.js"></script>
<script>
  document.getElementById('avatar').innerHTML = lxmface('a7b3c9d1e5f20681943ab2de77fc8e01', 64);
</script>
```

Also works as an ES module or CommonJS require. Single file, no build step, no dependencies.

## How It Works

The input hash seeds a Xorshift128 PRNG, which generates three HSL colors and an 8x8 symmetric grid. Each cell maps to one of the three colors. The grid is rendered as SVG rectangles inside a circular clip path.

Same hash, same PRNG sequence, same colors, same grid, same avatar. Every time.

## License

MIT

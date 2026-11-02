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

LXMFace generates a repeatable avatar from an LXMF address string. Its Rust and JavaScript implementations produce the same image for the same canonical ASCII address seed. Avatars can collide and are only a visual aid; compare addresses or keys when verifying a contact.

It works like the original [blockies](https://github.com/download13/blockies): feed in an address hash, get back a deterministic symmetrical identicon. No server, no image files, no lookups. The output is a self-contained SVG string.

Available as a **Rust crate** and a **JavaScript library**. Shared test vectors check identical output for address seeds. Arbitrary Unicode seed parity is not guaranteed: Rust iterates Unicode scalar values, while JavaScript uses UTF-16 code units.

## Usage

### Rust

This source tree declares version **0.1.0** and requires Rust **1.85+**. To use a sibling checkout:

```toml
[dependencies]
lxmface = { path = "../lxmface" }
```

For a reproducible dependency, keep that checkout pinned to the revision you tested.

```rust
use lxmface::lxmface_svg;

// Generate an SVG string from an LXMF address hash
let svg = lxmface_svg("0123456789abcdef0123456789abcdef", 64);
```

The default Rust build uses `std` and has no runtime dependencies. This revision declares a `no_std` mode, but it does not compile with `default-features = false`; keep the default features enabled.

If you need the raw colors and grid for custom rendering (native canvas, terminal, pixel buffer):

```rust
let data = lxmface::lxmface_data("0123456789abcdef0123456789abcdef");
// data.color, data.bgcolor, data.spotcolor — HslColor { h, s, l }
// data.grid — [[u8; 8]; 8], where 0=bg, 1=primary, 2=spot
```

### JavaScript

Place `js/lxmface.js` beside the HTML file for this example:

```html
<div id="avatar"></div>
<script src="lxmface.js"></script>
<script>
  document.getElementById('avatar').innerHTML = lxmface('0123456789abcdef0123456789abcdef', 64);
</script>
```

The single-file UMD library exposes the browser global `lxmface` and supports CommonJS and AMD. It does not declare native ES module exports, despite the `module` field in its package metadata. No build step or runtime dependency is needed for the script example.

## How It Works

The input hash seeds a Xorshift128 PRNG, which generates three HSL colors and an 8x8 symmetric grid. Each cell maps to one of the three colors. The grid is rendered as SVG rectangles inside a circular clip path.

The library treats the input as a seed; it does not validate an LXMF address. Use the same canonical representation in every caller, including letter case. Shared fixtures compare the PRNG, colors, grid and exact SVG output. Ratspeak 1.0.37 still contains an inline Blockies implementation; using this package is a separate application integration.

## License

MIT

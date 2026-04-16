//! # LXMFace
//!
//! Deterministic identicon generator for LXMF/Reticulum addresses.
//!
//! Produces symmetric, circular SVG avatars from hash strings.
//! The same input always generates the same output, across both
//! the Rust and JavaScript implementations.
//!
//! ```
//! let svg = lxmface::lxmface_svg("a7b3c9d1e5f20681943ab2de77fc8e01", 64);
//! assert!(svg.starts_with("<svg"));
//!
//! let data = lxmface::lxmface_data("a7b3c9d1e5f20681943ab2de77fc8e01");
//! assert!(data.color.h < 360);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod color;
mod grid;
mod prng;
mod svg;

pub use color::HslColor;
pub use grid::GRID_SIZE;

#[doc(hidden)]
pub mod test_internals {
    pub use crate::prng::Xorshift128;
}

/// Raw identicon data for custom rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct LxmfaceData {
    /// Primary color
    pub color: HslColor,
    /// Background color
    pub bgcolor: HslColor,
    /// Spot/accent color
    pub spotcolor: HslColor,
    /// 8x8 grid: 0 = bgcolor, 1 = color, 2 = spotcolor
    pub grid: [[u8; GRID_SIZE]; GRID_SIZE],
}

/// Generate an SVG identicon string from a seed hash and pixel size.
///
/// The seed is typically an LXMF hex address. The returned string is a
/// self-contained SVG document that can be embedded directly in HTML.
pub fn lxmface_svg(seed: &str, size: u32) -> alloc::string::String {
    let mut rng = prng::Xorshift128::new(seed);
    let color = color::create_color(&mut rng);
    let bgcolor = color::create_color(&mut rng);
    let spotcolor = color::create_color(&mut rng);
    let grid = grid::create_grid(&mut rng);
    svg::render_svg(&color, &bgcolor, &spotcolor, &grid, size)
}

/// Generate raw identicon data from a seed hash.
///
/// Use this when you need the grid and colors for non-SVG rendering
/// (terminal, native canvas, pixel buffer, etc.).
pub fn lxmface_data(seed: &str) -> LxmfaceData {
    let mut rng = prng::Xorshift128::new(seed);
    let color = color::create_color(&mut rng);
    let bgcolor = color::create_color(&mut rng);
    let spotcolor = color::create_color(&mut rng);
    let grid = grid::create_grid(&mut rng);
    LxmfaceData {
        color,
        bgcolor,
        spotcolor,
        grid,
    }
}

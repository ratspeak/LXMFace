use crate::color::HslColor;
use crate::grid::GRID_SIZE;

/// Render an SVG string from grid data and colors.
///
/// Matches the JS SVG output: 8x8 rects inside a circular clipPath,
/// with `shape-rendering="crispEdges"` and a fixed clip ID.
pub fn render_svg(
    color: &HslColor,
    bgcolor: &HslColor,
    spotcolor: &HslColor,
    grid: &[[u8; GRID_SIZE]; GRID_SIZE],
    size: u32,
) -> String {
    let color_css = color.to_css();
    let bgcolor_css = bgcolor.to_css();
    let spotcolor_css = spotcolor.to_css();

    // Pre-allocate: ~3KB is typical for an 8x8 SVG
    let mut svg = String::with_capacity(3072);

    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&size.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&size.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&GRID_SIZE.to_string());
    svg.push(' ');
    svg.push_str(&GRID_SIZE.to_string());
    svg.push_str("\" shape-rendering=\"crispEdges\">");

    // Circular clip path
    let r = GRID_SIZE / 2;
    svg.push_str("<clipPath id=\"lxmface-clip\"><circle cx=\"");
    svg.push_str(&r.to_string());
    svg.push_str("\" cy=\"");
    svg.push_str(&r.to_string());
    svg.push_str("\" r=\"");
    svg.push_str(&r.to_string());
    svg.push_str("\"/></clipPath>");

    svg.push_str("<g clip-path=\"url(#lxmface-clip)\">");

    // 64 rect elements
    for (y, row) in grid.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let fill = match val {
                0 => &bgcolor_css,
                1 => &color_css,
                _ => &spotcolor_css,
            };
            svg.push_str("<rect x=\"");
            svg.push_str(&x.to_string());
            svg.push_str("\" y=\"");
            svg.push_str(&y.to_string());
            svg.push_str("\" width=\"1\" height=\"1\" fill=\"");
            svg.push_str(fill);
            svg.push_str("\"/>");
        }
    }

    svg.push_str("</g></svg>");
    svg
}

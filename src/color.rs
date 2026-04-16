use crate::prng::Xorshift128;

/// An HSL color as produced by the blockies algorithm.
#[derive(Debug, Clone, PartialEq)]
pub struct HslColor {
    /// Hue, 0-359
    pub h: u16,
    /// Saturation, 40.0-100.0
    pub s: f64,
    /// Lightness, 0.0-100.0
    pub l: f64,
}

impl HslColor {
    /// Format as CSS HSL string matching JS output: `hsl(H,S%,L%)`
    pub fn to_css(&self) -> String {
        format!(
            "hsl({},{}%,{}%)",
            self.h,
            js_float(self.s),
            js_float(self.l)
        )
    }
}

/// Create an HSL color from the PRNG, matching JS `createColor()`.
///
/// Consumes 6 PRNG values: 1 for hue, 1 for saturation, 4 for lightness.
pub fn create_color(rng: &mut Xorshift128) -> HslColor {
    let h = (rng.next_f64() * 360.0).floor() as u16;
    let s = rng.next_f64() * 60.0 + 40.0;
    let l = (rng.next_f64() + rng.next_f64() + rng.next_f64() + rng.next_f64()) * 25.0;
    HslColor { h, s, l }
}

/// Format an f64 the same way JavaScript's Number.prototype.toString() does.
///
/// JS uses shortest-representation (Grisu/Dragon4). Rust's Display for f64
/// also uses shortest-representation (Ryu). They should match, but we handle
/// the special case where the value is an exact integer (JS omits the decimal).
fn js_float(v: f64) -> String {
    if v == v.floor() && v.is_finite() {
        // JS: (40.0).toString() === "40", not "40.0"
        format!("{}", v as i64)
    } else {
        format!("{}", v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_formatting() {
        assert_eq!(js_float(40.0), "40");
        assert_eq!(js_float(0.0), "0");
        assert_eq!(js_float(360.0), "360");
    }

    #[test]
    fn fractional_formatting() {
        // Non-integer values should retain their decimal representation
        let v = 40.11573363095522;
        let s = js_float(v);
        assert!(s.contains('.'), "expected decimal point in {}", s);
    }
}

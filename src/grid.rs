use crate::prng::Xorshift128;

/// Grid size is always 8x8.
pub const GRID_SIZE: usize = 8;

/// Generate an 8x8 symmetric grid, matching JS `createImageData()`.
///
/// Each cell is 0 (background), 1 (primary), or 2 (spot color).
/// The left half is generated randomly, then mirrored to the right.
pub fn create_grid(rng: &mut Xorshift128) -> [[u8; GRID_SIZE]; GRID_SIZE] {
    let half_w = GRID_SIZE.div_ceil(2); // 4
    let mirror_start = GRID_SIZE / 2; // 4
    let mut grid = [[0u8; GRID_SIZE]; GRID_SIZE];

    for row in &mut grid {
        // Generate left half
        for cell in row.iter_mut().take(half_w) {
            *cell = (rng.next_f64() * 2.3).floor() as u8;
        }
        // Mirror: fill right half from center outward
        // JS: for (var x = Math.floor(w / 2) - 1; x >= 0; x--) fullRow.push(row[x])
        for x in (0..mirror_start).rev() {
            row[half_w + (mirror_start - 1 - x)] = row[x];
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_is_symmetric() {
        let mut rng = Xorshift128::new("test_symmetry");
        let grid = create_grid(&mut rng);
        for (y, row) in grid.iter().enumerate() {
            for x in 0..GRID_SIZE {
                assert_eq!(
                    row[x],
                    row[GRID_SIZE - 1 - x],
                    "row {} not symmetric at x={}",
                    y,
                    x
                );
            }
        }
    }

    #[test]
    fn grid_values_in_range() {
        let mut rng = Xorshift128::new("test_range");
        let grid = create_grid(&mut rng);
        for row in &grid {
            for &val in row {
                assert!(val <= 2, "grid value {} out of range", val);
            }
        }
    }
}

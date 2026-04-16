use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct VectorFile {
    vectors: Vec<TestVector>,
}

#[derive(Deserialize)]
struct TestVector {
    seed: String,
    size: u32,
    seeded_state: [i32; 4],
    color: String,
    bgcolor: String,
    spotcolor: String,
    grid: Vec<Vec<u8>>,
    svg: String,
}

fn load_vectors() -> Vec<TestVector> {
    let data = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/vectors.json"))
        .expect("failed to read vectors.json");
    let file: VectorFile = serde_json::from_str(&data).expect("failed to parse vectors.json");
    file.vectors
}

#[test]
fn prng_seeded_state_matches_js() {
    for vec in &load_vectors() {
        let rng = lxmface::test_internals::Xorshift128::new(&vec.seed);
        let state = rng.seeded_state();
        assert_eq!(
            state, vec.seeded_state,
            "seeded state mismatch for seed {:?}",
            vec.seed
        );
    }
}

#[test]
fn colors_match_js() {
    for vec in &load_vectors() {
        let data = lxmface::lxmface_data(&vec.seed);
        assert_eq!(
            data.color.to_css(),
            vec.color,
            "primary color mismatch for seed {:?}",
            vec.seed
        );
        assert_eq!(
            data.bgcolor.to_css(),
            vec.bgcolor,
            "bgcolor mismatch for seed {:?}",
            vec.seed
        );
        assert_eq!(
            data.spotcolor.to_css(),
            vec.spotcolor,
            "spotcolor mismatch for seed {:?}",
            vec.seed
        );
    }
}

#[test]
fn grid_matches_js() {
    for vec in &load_vectors() {
        let data = lxmface::lxmface_data(&vec.seed);
        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(
                    data.grid[y][x], vec.grid[y][x],
                    "grid mismatch at ({},{}) for seed {:?}",
                    x, y, vec.seed
                );
            }
        }
    }
}

#[test]
fn svg_matches_js() {
    for vec in &load_vectors() {
        let svg = lxmface::lxmface_svg(&vec.seed, vec.size);
        assert_eq!(svg, vec.svg, "SVG mismatch for seed {:?}", vec.seed);
    }
}

#[test]
fn grids_are_symmetric() {
    for vec in &load_vectors() {
        let data = lxmface::lxmface_data(&vec.seed);
        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(
                    data.grid[y][x],
                    data.grid[y][7 - x],
                    "grid not symmetric at ({},{}) for seed {:?}",
                    x,
                    y,
                    vec.seed
                );
            }
        }
    }
}

#[test]
fn grid_values_in_range() {
    for vec in &load_vectors() {
        let data = lxmface::lxmface_data(&vec.seed);
        for y in 0..8 {
            for x in 0..8 {
                assert!(
                    data.grid[y][x] <= 2,
                    "grid value {} at ({},{}) for seed {:?}",
                    data.grid[y][x],
                    x,
                    y,
                    vec.seed
                );
            }
        }
    }
}

#[test]
fn deterministic_across_calls() {
    for vec in &load_vectors() {
        let svg1 = lxmface::lxmface_svg(&vec.seed, vec.size);
        let svg2 = lxmface::lxmface_svg(&vec.seed, vec.size);
        assert_eq!(svg1, svg2, "non-deterministic for seed {:?}", vec.seed);
    }
}

#[test]
fn different_seeds_produce_different_output() {
    let vectors = load_vectors();
    // Skip the first two (all-zeros and all-f's could theoretically collide,
    // but in practice all 24 seeds produce unique avatars)
    for i in 0..vectors.len() {
        for j in (i + 1)..vectors.len() {
            let svg_i = lxmface::lxmface_svg(&vectors[i].seed, 64);
            let svg_j = lxmface::lxmface_svg(&vectors[j].seed, 64);
            assert_ne!(
                svg_i, svg_j,
                "collision between seeds {:?} and {:?}",
                vectors[i].seed, vectors[j].seed
            );
        }
    }
}

#[test]
fn different_sizes_same_colors_and_grid() {
    for vec in &load_vectors() {
        let data_small = lxmface::lxmface_data(&vec.seed);
        // lxmface_data doesn't take size — same seed always same data
        let data_again = lxmface::lxmface_data(&vec.seed);
        assert_eq!(data_small, data_again);
    }
}

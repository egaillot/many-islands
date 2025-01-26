use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

const ISLAND_ID: u64 = 2;
const SIZE_ORDER: u32 = 6;
const SIZE: usize = 2_u32.pow(SIZE_ORDER) as usize;

fn generate_submap_delimiters(submap_delimiters: &mut Vec<[usize; 4]>, top: usize, left: usize, bottom: usize, right: usize) {
    submap_delimiters.push([top, left, bottom, right]);

    let middle = (left + right) / 2;
    let center = (top + bottom) / 2;
    if middle - left <= 1 { return; }
    generate_submap_delimiters(submap_delimiters, top, left, center, middle);
    generate_submap_delimiters(submap_delimiters, top, middle, center, right);
    generate_submap_delimiters(submap_delimiters, center, left, bottom, middle);
    generate_submap_delimiters(submap_delimiters, center, middle, bottom, right);
}

fn submap_delimiters() -> Vec<[usize; 4]> {
    let mut submap_delimiters: Vec<[usize; 4]> = Vec::new();
    generate_submap_delimiters(&mut submap_delimiters, 0, 0, SIZE, SIZE);

    submap_delimiters
}

fn epsilon(rng: &mut ChaCha8Rng) -> f32 {
    0.3*(rng.gen::<f32>() - 0.5)
}

fn mutate_intermediary_cell(rng: &mut ChaCha8Rng, map: &mut [[f32; SIZE + 1]; SIZE + 1], row1: usize, col1: usize, row2: usize, col2: usize) {
    let row = (row1 + row2) / 2;
    let col = (col1 + col2) / 2;

    if map[row][col] == 0.0 {
        map[row][col] = (map[row1][col1] + map[row2][col2]) / 2.0 + epsilon(rng);
    }
}

fn mutate_map(rng: &mut ChaCha8Rng, map: &mut [[f32; SIZE + 1]; SIZE + 1], delimiter: [usize; 4]) {
    let [top, left, bottom, right] = delimiter;

    mutate_intermediary_cell(rng, map, top, left, top, right);
    mutate_intermediary_cell(rng, map, bottom, left, bottom, right);
    mutate_intermediary_cell(rng, map, top, left, bottom, left);
    mutate_intermediary_cell(rng, map, top, right, bottom, right);
    mutate_intermediary_cell(rng, map, top, left, bottom, right);
}

fn init_map(submap_delimiters: Vec<[usize; 4]>) -> [[f32; SIZE + 1]; SIZE + 1] {
    let mut map = [[0.0; SIZE + 1]; SIZE + 1];
    let mut rng = ChaCha8Rng::seed_from_u64(ISLAND_ID);

    map[SIZE / 2][SIZE / 2] = 1.0;
    for delimiter in submap_delimiters {
        mutate_map(&mut rng, &mut map, delimiter)
    }

    map
}

fn main() {
    let submap_delimiters = submap_delimiters();
    let map = init_map(submap_delimiters);

    for row in map {
        for cell in row {
            let printable_cell = if cell > 0.8 { "• " }
                                 else if cell > 0.5 { "+ " }
                                 else if cell > 0.3 { ". " }
                                 else { "  " };
            print!("{printable_cell}");
        }
        println!();
    }
}

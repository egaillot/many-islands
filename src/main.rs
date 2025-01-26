use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

const ISLAND_ID: u64 = 12345;
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
    0.25*(rng.gen::<f32>() - 0.5)
}

fn mutate_intermediary_cell(rng: &mut ChaCha8Rng, map: &mut [[f32; SIZE + 1]; SIZE + 1], points: Vec<[usize; 2]>) {
    let mut row = 0;
    let mut col = 0;
    let mut elevation = 0.0;

    for [p_row, p_col] in &points {
        row = row + p_row;
        col = col + p_col;
        elevation = elevation + map[*p_row][*p_col];
    }

    row = row / points.len();
    col = col / points.len();
    elevation = elevation / points.len() as f32 + epsilon(rng);

    if map[row][col] == 0.0 { map[row][col] = elevation; }
}

fn mutate_map(rng: &mut ChaCha8Rng, map: &mut [[f32; SIZE + 1]; SIZE + 1], delimiter: [usize; 4]) {
    let [top, left, bottom, right] = delimiter;

    mutate_intermediary_cell(rng, map, vec![[top, left], [top, right]]);
    mutate_intermediary_cell(rng, map, vec![[bottom, left], [bottom, right]]);
    mutate_intermediary_cell(rng, map, vec![[top, left], [bottom, left]]);
    mutate_intermediary_cell(rng, map, vec![[top, right], [bottom, right]]);
    mutate_intermediary_cell(rng, map, vec![[top, left], [top, right], [bottom, left], [bottom, right]]);
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

fn draw(map: [[f32; SIZE + 1]; SIZE + 1]) {
    for row in map {
        for cell in row {
            let printable_cell = if cell > 0.7 { "• " }
                                 else if cell > 0.45 { "+ " }
                                 else if cell > 0.35 { ". " }
                                 else { "  " };
            print!("{printable_cell}");
        }
        println!();
    }
}

fn main() {
    let submap_delimiters = submap_delimiters();
    let map = init_map(submap_delimiters);
    draw(map);
}

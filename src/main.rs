const SIZE_ORDER: u32 = 4;
const SIZE: usize = 2_u32.pow(SIZE_ORDER) as usize;

fn generate_submap_delimiters(submap_delimiters: &mut Vec<[usize; 4]>, top: usize, left: usize, bottom: usize, right: usize) {
    submap_delimiters.push([top, left, bottom, right]);

    let middle = (left + right) / 2;
    let center = (top + bottom) / 2;
    if middle - left <= 1 { return; }
    generate_submap_delimiters(submap_delimiters, top, left, center, middle);
}

fn epsilon() -> f32 {
    0.2
}

fn mutate_intermediary_cell(map: &mut [[f32; SIZE + 1]; SIZE + 1], row1: usize, col1: usize, row2: usize, col2: usize) {
    let row = (row1 + row2) / 2;
    let col = (col1 + col2) / 2;

    if map[row][col] == 0.0 {
        map[row][col] = (map[row1][col1] + map[row2][col2]) / 2.0 + epsilon();
    }
}

fn mutate_map(map: &mut [[f32; SIZE + 1]; SIZE + 1], delimiter: [usize; 4]) {
    let [top, left, bottom, right] = delimiter;

    mutate_intermediary_cell(map, top, left, top, right);
    mutate_intermediary_cell(map, bottom, left, bottom, right);
    mutate_intermediary_cell(map, top, left, bottom, left);
    mutate_intermediary_cell(map, top, right, bottom, right);
    mutate_intermediary_cell(map, top, left, bottom, right);
}

fn main() {
    let mut submap_delimiters: Vec<[usize; 4]> = Vec::new();
    generate_submap_delimiters(&mut submap_delimiters, 0, 0, SIZE, SIZE);

    let mut map = [[0.0; SIZE + 1]; SIZE + 1];
    map[SIZE / 2][SIZE / 2] = 1.0;

    for delimiter in submap_delimiters {
        mutate_map(&mut map, delimiter)
    }

    for row in map {
        for cell in row {
            let printable_cell = if cell > 0.5 { "• " }
                                 else if cell > 0.1 { "+ " }
                                 else { "· " };
            print!("{printable_cell}");
        }
        println!();
    }
}

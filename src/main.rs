const SIZE_ORDER: u32 = 4;
const SIZE: usize = 2_u32.pow(SIZE_ORDER) as usize;

fn generate_submap_delimiters(submap_delimiters: &mut Vec<[usize; 4]>, top: usize, left: usize, bottom: usize, right: usize) {
    submap_delimiters.push([top, left, bottom, right]);
}

fn epsilon() -> f32 {
    0.2
}

fn mutate_map(map: &mut [[f32; SIZE + 1]; SIZE + 1], delimiter: [usize; 4]) {
    let [top, left, _bottom, right] = delimiter;
    let center = (left + right) / 2;

    if map[top][center] == 0.0 { map[top][center] = (map[top][left] + map[top][right]) / 2.0 + epsilon(); }
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

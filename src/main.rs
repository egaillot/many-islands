const SIZE_ORDER: u32 = 4;

fn main() {
    const SIZE: usize = 2_u32.pow(SIZE_ORDER) as usize;

    let mut map = [[0.0; SIZE + 1]; SIZE + 1];
    map[SIZE / 2][SIZE / 2] = 1.0;

    for row in map {
        for cell in row {
            let printable_cell = if cell > 0.5 { "• " }
                                 else { "· " };
            print!("{printable_cell}");
        }
        println!();
    }
}

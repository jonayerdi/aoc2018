const GRID_SERIAL: usize = 8141;
const GRID_SIZE: (usize, usize) = (300, 300);
const SQUARE_SIZE: (usize, usize) = (3, 3);

fn main() {
    let grid = (1..=GRID_SIZE.1)
        .map(|y| {
            (1..=GRID_SIZE.0)
                .map(|x| ((((((x + 10) * y) + GRID_SERIAL) * (x + 10)) / 100) % 10) as i32 - 5)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut max_square = (0, 0, i32::min_value());
    for y in 0..=GRID_SIZE.1 - SQUARE_SIZE.1 {
        for x in 0..=GRID_SIZE.0 - SQUARE_SIZE.0 {
            let power = grid[y..y + SQUARE_SIZE.1]
                .iter()
                .map(|row| row[x..x + SQUARE_SIZE.0].iter().sum::<i32>())
                .sum::<i32>();
            if power > max_square.2 {
                max_square = (x + 1, y + 1, power);
            }
        }
    }
    println!("{},{}", max_square.0, max_square.1);
}

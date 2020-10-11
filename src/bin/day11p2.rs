use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

const GRID_SIZE: (usize, usize) = (300, 300);

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day11.txt"))?.read_to_string(&mut input)?;
    let grid_serial = input.parse::<usize>().unwrap();
    let grid = (1..=GRID_SIZE.1)
        .map(|y| {
            (1..=GRID_SIZE.0)
                .map(|x| ((((((x + 10) * y) + grid_serial) * (x + 10)) / 100) % 10) as i32 - 5)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut max_square = (0, 0, 0, i32::min_value());
    for square_size in 1..=300 {
        for y in 0..=GRID_SIZE.1 - square_size {
            for x in 0..=GRID_SIZE.0 - square_size {
                let power = grid[y..y + square_size]
                    .iter()
                    .map(|row| row[x..x + square_size].iter().sum::<i32>())
                    .sum::<i32>();
                if power > max_square.3 {
                    max_square = (x + 1, y + 1, square_size, power);
                }
            }
        }
    }
    println!("{},{},{}", max_square.0, max_square.1, max_square.2);
    Ok(())
}

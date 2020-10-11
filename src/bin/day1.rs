use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day1.txt"))?.read_to_string(&mut input)?;
    let sum: i64 = input
        .lines()
        .map(|e| e.trim().parse::<i64>().unwrap())
        .sum();
    println!("{}", sum);
    Ok(())
}

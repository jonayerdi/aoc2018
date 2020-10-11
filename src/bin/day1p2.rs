use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day1.txt"))?.read_to_string(&mut input)?;
    let mut result = None;
    let mut sum: i64 = 0;
    let mut seen = HashSet::with_capacity(64);
    seen.insert(0);
    while result == None {
        for e in input.lines() {
            sum += e.trim().parse::<i64>().unwrap();
            if !seen.insert(sum) {
                result = Some(sum);
                break;
            }
        }
    }
    println!("{}", result.unwrap());
    Ok(())
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day2.txt"))?.read_to_string(&mut input)?;
    let mut sums = (0u64, 0u64);
    let mut counts = HashMap::with_capacity(32);
    for id in input.lines() {
        counts.clear();
        for c in id.chars() {
            let appearances = counts.entry(c).or_insert(0);
            *appearances += 1;
        }
        let mut increments = (0, 0);
        for &value in counts.values() {
            if increments.0 == 0 && value == 2 {
                increments.0 = 1;
                if increments.1 == 1 {
                    break;
                }
            } else if increments.1 == 0 && value == 3 {
                increments.1 = 1;
                if increments.0 == 1 {
                    break;
                }
            }
        }
        sums.0 += increments.0;
        sums.1 += increments.1;
    }
    //println!("{} * {} = {}", sums.0, sums.1, sums.0 * sums.1);
    println!("{}", sums.0 * sums.1);
    Ok(())
}

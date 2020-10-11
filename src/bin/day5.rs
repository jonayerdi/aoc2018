use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day5.txt"))?.read_to_string(&mut input)?;
    while let Some((index, (_, _))) =
        input
            .chars()
            .zip(input.chars().skip(1))
            .enumerate()
            .find(|(_, (current, next))| {
                let lower = current.to_ascii_lowercase();
                let upper = current.to_ascii_uppercase();
                (*current != lower && lower == *next) || (*current != upper && upper == *next)
            })
    {
        input.replace_range(index..=index + 1, "");
    }
    println!("{}", input.len());
    Ok(())
}

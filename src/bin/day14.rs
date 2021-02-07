use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn digit(ascii: u8) -> u8 {
    ascii - ('0' as u8)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day14.txt"))?.read_to_string(&mut input)?;
    let skip = input.parse::<usize>().unwrap();
    let count = 10;
    let mut recipes = String::from("37");
    let mut elves = (0usize, 1usize);
    while recipes.len() < skip + count {
        let r_old = (
            digit(recipes.as_bytes()[elves.0]),
            digit(recipes.as_bytes()[elves.1]),
        );
        let r_new = format!("{}", r_old.0 + r_old.1);
        recipes.extend(r_new.chars());
        elves = (
            (elves.0 + r_old.0 as usize + 1) % recipes.len(),
            (elves.1 + r_old.1 as usize + 1) % recipes.len(),
        );
    }

    println!("{}", &recipes[skip..skip + count]);
    Ok(())
}

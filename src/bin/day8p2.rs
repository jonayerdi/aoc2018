use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day8.txt"))?.read_to_string(&mut input)?;
    let mut tree = input.split(' ').map(|n| n.parse::<usize>().unwrap());
    println!("{}", value_of_entry(&mut tree));
    Ok(())
}

fn value_of_entry<T>(tree: &mut T) -> usize
where
    T: Iterator<Item = usize>,
{
    let child_count = tree.next().unwrap();
    let entry_count = tree.next().unwrap();
    if child_count > 0 {
        let child_values: Vec<_> = (0..child_count).map(|_| value_of_entry(tree)).collect();
        (0..entry_count)
            .map(|_| tree.next().unwrap())
            .map(|index| {
                if index > 0 {
                    *child_values.get(index - 1).unwrap_or(&0)
                } else {
                    0
                }
            })
            .sum::<usize>()
    } else {
        (0..entry_count)
            .map(|_| tree.next().unwrap())
            .sum::<usize>()
    }
}

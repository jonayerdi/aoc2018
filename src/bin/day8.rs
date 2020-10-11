use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day8.txt"))?.read_to_string(&mut input)?;
    let mut tree = input.split(' ').map(|n| n.parse::<usize>().unwrap());
    println!("{}", sum_of_entries(&mut tree, 1));
    Ok(())
}

fn sum_of_entries<T>(tree: &mut T, node_count: usize) -> usize
where
    T: Iterator<Item = usize>,
{
    let mut sum = 0;
    for _ in 0..node_count {
        let child_count = tree.next().unwrap();
        let entry_count = tree.next().unwrap();
        sum += sum_of_entries(tree, child_count);
        sum += (0..entry_count)
            .map(|_| tree.next().unwrap())
            .sum::<usize>();
    }
    sum
}

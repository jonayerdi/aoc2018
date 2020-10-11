use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day7.txt"))?.read_to_string(&mut input)?;
    let mut nodes = BTreeMap::new();
    let mut result = Vec::new();
    input
        .lines()
        .map(|line| {
            let mut words = line.split(' ');
            (words.nth(1).unwrap(), words.nth(5).unwrap())
        })
        .for_each(|(requirement, step)| {
            nodes.entry(requirement).or_insert_with(BTreeSet::new);
            nodes
                .entry(step)
                .or_insert_with(BTreeSet::new)
                .insert(requirement);
        });
    while let Some(current) = nodes.iter().find(|node| node.1.is_empty()) {
        let current = current.0.clone();
        result.push(current);
        nodes.remove(current);
        nodes.iter_mut().for_each(|node| {
            node.1.remove(current);
        });
    }
    println!(
        "{}",
        result
            .iter()
            .map(|s| s.chars())
            .flatten()
            .collect::<String>()
    );
    Ok(())
}

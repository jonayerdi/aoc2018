#[allow(non_upper_case_globals)]
const teststr: &str = include_str!("day7.txt");

fn main() {
    use std::collections::btree_map::BTreeMap;
    use std::collections::btree_set::BTreeSet;
    let mut nodes = BTreeMap::new();
    let mut result = Vec::new();
    teststr
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
}

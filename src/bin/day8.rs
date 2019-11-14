#[allow(non_upper_case_globals)]
const teststr: &str = include_str!("day8.txt");

fn main() {
    let mut tree = teststr.split(' ').map(|n| n.parse::<usize>().unwrap());
    println!("{}", sum_of_entries(&mut tree, 1));
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

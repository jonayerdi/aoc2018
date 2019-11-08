#[allow(non_upper_case_globals)]
const teststr: &'static str = include_str!("day1.txt");

fn main() {
    let sum: i64 = teststr
        .lines()
        .map(|e| e.trim().parse::<i64>().unwrap())
        .sum();
    println!("{}", sum);
}

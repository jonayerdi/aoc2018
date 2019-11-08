#[allow(non_upper_case_globals)]
const teststr: &'static str = include_str!("day1.txt");

fn main() {
    use std::collections::HashSet;
    let mut result = None;
    let mut sum: i64 = 0;
    let mut seen = HashSet::with_capacity(64);
    seen.insert(0);
    while result == None {
        for e in teststr.lines() {
            sum += e.trim().parse::<i64>().unwrap();
            if !seen.insert(sum) {
                result = Some(sum);
                break;
            }
        }
    }
    println!("{}", result.unwrap());
}

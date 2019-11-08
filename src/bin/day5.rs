fn main() {
    let mut teststr = String::from(include_str!("day5.txt"));
    while let Some((index, (_, _))) = teststr
        .chars()
        .zip(teststr.chars().skip(1))
        .enumerate()
        .find(|(_, (current, next))| {
            let lower = current.to_ascii_lowercase();
            let upper = current.to_ascii_uppercase();
            (*current != lower && lower == *next) || (*current != upper && upper == *next)
        })
    {
        teststr.replace_range(index..=index + 1, "");
    }
    println!("{}", teststr.len());
}

fn main() {
    let mut minlen = None;
    for remove in "abcdefghijklmnopqrstuvwxyz".chars() {
        let mut teststr = String::from(include_str!("day5.txt"));
        teststr.retain(|c| c != remove && c != remove.to_ascii_uppercase());
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
        if let Some(len) = minlen {
            if teststr.len() < len {
                minlen = Some(teststr.len());
            }
        } else {
            minlen = Some(teststr.len());
        }
    }
    if let Some(len) = minlen {
        println!("{}", len);
    }
}

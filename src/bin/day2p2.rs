use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn find_diff1(id_list: &str) -> Option<(&str, &str, usize)> {
    for (id_index, id1) in id_list.lines().enumerate() {
        for id2 in id_list.lines().skip(id_index + 1) {
            let mut diff_index = None;
            for (char_index, (c1, c2)) in id1.chars().zip(id2.chars()).enumerate() {
                if c1 != c2 {
                    if diff_index.is_some() {
                        diff_index = None;
                        break;
                    } else {
                        diff_index = Some(char_index);
                    }
                }
            }
            if let Some(diff_index) = diff_index {
                return Some((id1, id2, diff_index));
            }
        }
    }
    None
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day2.txt"))?.read_to_string(&mut input)?;
    if let Some((id1, _id2, diff_index)) = find_diff1(&input) {
        println!("{}{}", &id1[..diff_index], &id1[diff_index + 1..]);
    } else {
        println!("No id pair with one character difference found in list");
    }
    Ok(())
}

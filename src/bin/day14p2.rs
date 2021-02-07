use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::collections::HashSet;

fn digit(ascii: u8) -> u8 {
    ascii - ('0' as u8)
}

struct Matcher<'a, T>
where
    T: std::cmp::PartialEq,
{
    needle: &'a [T],
    count: usize,
    matches: Vec<usize>,
    ongoing: HashSet<usize>,
    ongoing_remove: HashSet<usize>,
}

impl<'a, T> Matcher<'a, T>
where
    T: std::cmp::PartialEq,
{
    pub fn new(needle: &'a [T]) -> Self {
        Self {
            needle,
            count: 0,
            matches: vec![],
            ongoing: HashSet::with_capacity(needle.len()),
            ongoing_remove: HashSet::with_capacity(needle.len()),
        }
    }
    pub fn consume(&mut self, item: &T) {
        for &start in self.ongoing.iter() {
            if item != &self.needle[self.count - start] {
                self.ongoing_remove.insert(start);
            }
        }
        for start in self.ongoing_remove.iter() {
            self.ongoing.remove(start);
        }
        self.ongoing_remove.clear();
        if item == &self.needle[0] {
            self.ongoing.insert(self.count);
        }
        self.count += 1;
        for &start in self.ongoing.iter() {
            if self.count - start == self.needle.len() {
                self.matches.push(start);
                self.ongoing_remove.insert(start);
            }
        }
        for start in self.ongoing_remove.iter() {
            self.ongoing.remove(start);
        }
        self.ongoing_remove.clear();
    }
    pub fn consume_all(&mut self, iter: impl Iterator<Item = T>) {
        iter.for_each(|item| self.consume(&item));
    }
    pub fn get_matches(&self) -> &[usize] {
        &self.matches
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day14.txt"))?.read_to_string(&mut input)?;
    let needle = &input;
    let mut recipes = String::from("37");
    let mut elves = (0usize, 1usize);
    let mut matcher = Matcher::new(needle.as_bytes());
    matcher.consume_all(recipes.bytes());
    while matcher.get_matches().is_empty() {
        let r_old = (
            digit(recipes.as_bytes()[elves.0]),
            digit(recipes.as_bytes()[elves.1]),
        );
        let r_new = format!("{}", r_old.0 + r_old.1);
        recipes.extend(r_new.chars());
        matcher.consume_all(r_new.bytes());
        elves = (
            (elves.0 + r_old.0 as usize + 1) % recipes.len(),
            (elves.1 + r_old.1 as usize + 1) % recipes.len(),
        );
    }
    println!("{}", matcher.get_matches().first().unwrap());
    Ok(())
}

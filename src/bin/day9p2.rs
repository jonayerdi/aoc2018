use linked_list::{LinkedList, LinkedListIndexMut};

use std::fs::File;
use std::io::{self, Read};
use std::iter;
use std::path::PathBuf;

struct Game<'a> {
    pub player: usize,
    pub scores: Vec<usize>,
    pub next_value: usize,
    position: LinkedListIndexMut<'a, usize>,
}

impl<'a> Game<'a> {
    fn new(players: usize, marbles: &mut LinkedList<usize>) -> Game {
        *marbles = iter::once(0).collect::<LinkedList<_>>();
        let index = marbles.first_mut().unwrap();
        Game {
            player: 0,
            scores: (0..players).map(|_| 0).collect(),
            next_value: 1,
            position: index,
        }
    }
    fn next(&mut self) {
        if self.next_value % 23 != 0 {
            self.position.next().unwrap();
            self.position.insert_after(self.next_value)
        } else {
            self.position.nth_back(6).unwrap();
            self.scores[self.player] += self.next_value + self.position.remove_advance();
        }
        self.player = (self.player + 1) % self.scores.len();
        self.next_value += 1;
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day9.txt"))?.read_to_string(&mut input)?;
    let mut words = input.split(" ");
    let players = words.nth(0).unwrap().parse::<usize>().unwrap();
    let turns = words.nth(5).unwrap().parse::<usize>().unwrap() * 100;
    let mut marbles = LinkedList::new();
    let mut game = Game::new(players, &mut marbles);
    (0..turns).for_each(|_| game.next());
    let high_score = game.scores.iter().fold(&0, |hi, s| hi.max(s));
    println!("{}", high_score);
    Ok(())
}

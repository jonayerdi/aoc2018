mod linked_list;
use linked_list::{LinkedList, LinkedListIndex};

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

struct Game {
    pub player: usize,
    pub scores: Vec<usize>,
    pub position: LinkedListIndex<usize>,
    pub next_value: usize,
    pub marbles: LinkedList<usize>,
}

impl Game {
    fn new(players: usize) -> Game {
        let mut ll = LinkedList::new();
        let mut index = ll.first();
        index.insert_after(0);
        Game {
            player: 0,
            scores: (0..players).map(|_| 0).collect(),
            position: index,
            next_value: 1,
            marbles: ll,
        }
    }
    fn next(&mut self) {
        if self.next_value % 23 != 0 {
            self.position.next().unwrap();
            self.position.insert_after(self.next_value)
        } else {
            self.position.nth_back(6).unwrap();
            self.scores[self.player] += self.next_value + self.position.remove();
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
    let mut game = Game::new(players);
    (0..turns).for_each(|_| game.next());
    let high_score = game.scores.iter().fold(&0, |hi, s| hi.max(s));
    println!("{}", high_score);
    Ok(())
}

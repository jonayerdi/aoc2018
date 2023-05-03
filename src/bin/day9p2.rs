use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Clone, Copy)]
struct Node {
    pub next: usize,
    pub previous: usize,
    pub value: usize,
}

struct Game {
    pub player: usize,
    pub scores: Vec<usize>,
    pub position: usize,
    pub next_value: usize,
    pub marbles: Vec<Node>,
}

impl Game {
    fn new(players: usize) -> Game {
        Game {
            player: 0,
            scores: (0..players).map(|_| 0).collect(),
            position: 0,
            next_value: 1,
            marbles: vec![Node {
                next: 0,
                previous: 0,
                value: 0,
            }],
        }
    }
    fn insert(&mut self, value: usize) {
        let node = self.marbles[self.position];
        self.marbles.push(Node {
            next: node.next,
            previous: self.position,
            value,
        });
        let new_position = self.marbles.len() - 1;
        self.marbles[node.next].previous = new_position;
        self.marbles[self.position].next = new_position;
        self.position = new_position;
    }
    fn remove(&mut self) -> usize {
        let node = self.marbles[self.position];
        let previous = node.previous;
        let next = node.next;
        self.marbles[previous].next = next;
        self.marbles[next].previous = previous;
        self.position = next;
        node.value
    }
    fn move_forward(&mut self, count: usize) {
        for _ in 0..count {
            self.position = self.marbles[self.position].next;
        }
    }
    fn move_back(&mut self, count: usize) {
        for _ in 0..count {
            self.position = self.marbles[self.position].previous;
        }
    }
    fn next(&mut self) {
        if self.next_value % 23 != 0 {
            self.move_forward(1);
            self.insert(self.next_value);
        } else {
            self.move_back(7);
            self.scores[self.player] += self.next_value + self.remove();
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

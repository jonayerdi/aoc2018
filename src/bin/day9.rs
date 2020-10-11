use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

struct Game {
    pub player: usize,
    pub scores: Vec<usize>,
    pub position: usize,
    pub next_value: usize,
    pub marbles: Vec<usize>,
}

impl Game {
    fn new(players: usize) -> Game {
        Game {
            player: 0,
            scores: (0..players).map(|_| 0).collect(),
            position: 0,
            next_value: 1,
            marbles: vec![0],
        }
    }
    fn next(&mut self) {
        if self.next_value % 23 != 0 {
            let insert_after = (self.position + 1) % self.marbles.len();
            self.marbles.insert(insert_after + 1, self.next_value);
            self.position = insert_after + 1;
        } else {
            let take_at = (self.position + self.marbles.len() - 7) % self.marbles.len();
            self.scores[self.player] += self.next_value + self.marbles.remove(take_at);
            self.position = take_at;
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
    let turns = words.nth(5).unwrap().parse::<usize>().unwrap();
    let mut game = Game::new(players);
    (0..turns).for_each(|_| game.next());
    let high_score = game.scores.iter().fold(&0, |hi, s| hi.max(s));
    println!("{}", high_score);
    Ok(())
}

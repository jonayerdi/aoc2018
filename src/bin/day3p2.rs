use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

enum Cell {
    Empty,
    Claimed(usize),
    Overlap(Vec<usize>),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Claimed(id) => write!(f, "{}", id),
            Cell::Overlap(_) => write!(f, "X"),
        }
    }
}

struct Map {
    pub width: usize,
    pub cells: Vec<Cell>,
}

impl Map {
    fn with_size(size: (usize, usize)) -> Map {
        Map {
            width: size.0,
            cells: (0..size.0 * size.1).map(|_| Cell::Empty).collect(),
        }
    }
    fn index2position(width: usize, index: usize) -> (usize, usize) {
        (index % width, index / width)
    }
    fn cell_range_mut(
        &mut self,
        top_left: (usize, usize),
        bottom_right: (usize, usize),
    ) -> impl Iterator<Item = &mut Cell> {
        let width = self.width;
        self.cells
            .iter_mut()
            .enumerate()
            .filter(move |(index, _)| {
                let position = Map::index2position(width, *index);
                position.0 >= top_left.0
                    && position.1 >= top_left.1
                    && position.0 <= bottom_right.0
                    && position.1 <= bottom_right.1
            })
            .map(|(_, cell)| cell)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (index, cell) in self.cells.iter().enumerate() {
            write!(f, "{}", cell)?;
            if (index + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct Claim {
    pub id: usize,
    pub top_left: (usize, usize),
    pub bottom_right: (usize, usize),
}

impl Claim {
    fn new(id: usize, offset: (usize, usize), size: (usize, usize)) -> Claim {
        Claim {
            id,
            top_left: offset,
            bottom_right: (offset.0 + size.0 - 1, offset.1 + size.1 - 1),
        }
    }
    fn parse(line: &str) -> Claim {
        let mut chars = line.chars();
        assert_eq!(chars.next(), Some('#'));
        let id = parse_usize(&mut chars, ' ');
        assert_eq!(chars.next(), Some('@'));
        assert_eq!(chars.next(), Some(' '));
        let offset = (parse_usize(&mut chars, ','), parse_usize(&mut chars, ':'));
        assert_eq!(chars.next(), Some(' '));
        let size = (parse_usize(&mut chars, 'x'), parse_usize(&mut chars, '\n'));
        Claim::new(id, offset, size)
    }
}

fn parse_usize<T>(iter: &mut T, ending: char) -> usize
where
    T: Iterator<Item = char>,
{
    let mut chars = String::with_capacity(32);
    iter.find(|e| {
        if e.is_digit(10) {
            chars.push(*e);
            false
        } else {
            assert_eq!(*e, ending);
            true
        }
    });
    chars.parse::<usize>().unwrap()
}

fn main() -> io::Result<()> {
    use std::collections::HashSet;
    let mut map_size = (0, 0);
    let mut claims = Vec::with_capacity(1234);
    let mut no_overlaps = HashSet::with_capacity(1234);
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day3.txt"))?.read_to_string(&mut input)?;
    input.lines().for_each(|line| {
        let claim = Claim::parse(line);
        if claim.bottom_right.0 >= map_size.0 {
            map_size.0 = claim.bottom_right.0 + 1;
        }
        if claim.bottom_right.1 >= map_size.1 {
            map_size.1 = claim.bottom_right.1 + 1;
        }
        no_overlaps.insert(claim.id);
        claims.push(claim);
    });
    let mut map = Map::with_size(map_size);
    claims.iter().for_each(|claim| {
        map.cell_range_mut(claim.top_left, claim.bottom_right)
            .for_each(|cell| {
                match cell {
                    Cell::Empty => *cell = Cell::Claimed(claim.id),
                    Cell::Claimed(id) => {
                        no_overlaps.remove(&claim.id);
                        no_overlaps.remove(id);
                        *cell = Cell::Overlap(vec![*id, claim.id]);
                    }
                    Cell::Overlap(ids) => {
                        no_overlaps.remove(&claim.id);
                        ids.push(claim.id)
                    }
                };
            });
    });
    //println!("{}", map);
    for id in no_overlaps {
        println!("{}", id);
    }
    Ok(())
}

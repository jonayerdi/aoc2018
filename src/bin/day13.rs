use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

type Position = (usize, usize);

type Tracks = HashMap<Position, Connections>;

#[repr(u8)]
#[derive(Clone, Copy)]
enum Direction {
    Up = 1 << 0,
    Down = 1 << 1,
    Left = 1 << 2,
    Right = 1 << 3,
}

impl Direction {
    fn rotate(self, clockwise: bool) -> Direction {
        if clockwise {
            match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
        } else {
            match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            }
        }
    }
    fn draw(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Debug)]
struct Connections(u8);

macro_rules! connections {
    ( $( $x:expr ),* ) => {
        {
            let mut c = Connections::new();
            $(
                c.set_direction($x, true);
            )*
            c
        }
    };
}

impl PartialEq for Connections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[allow(dead_code)]
impl Connections {
    fn new() -> Self {
        Connections(0)
    }
    fn set_direction(&mut self, direction: Direction, value: bool) {
        if value {
            self.0 |= direction as u8;
        } else {
            self.0 &= !(direction as u8);
        }
    }
    fn has_direction(&self, direction: Direction) -> bool {
        self.0 & direction as u8 != 0
    }
    fn is_crossroad(&self) -> bool {
        *self
            == connections![
                Direction::Up,
                Direction::Down,
                Direction::Right,
                Direction::Left
            ]
    }
    fn draw(&self) -> char {
        if self.is_crossroad() {
            '+'
        } else if *self == connections![Direction::Up, Direction::Down] {
            '|'
        } else if *self == connections![Direction::Left, Direction::Right] {
            '-'
        } else if *self == connections![Direction::Up, Direction::Right]
            || *self == connections![Direction::Down, Direction::Left]
        {
            '\\'
        } else if *self == connections![Direction::Up, Direction::Left]
            || *self == connections![Direction::Down, Direction::Right]
        {
            '/'
        } else {
            panic!("Invalid Connections: {:?}", self)
        }
    }
}

#[derive(Clone, Copy)]
enum Crossroad {
    Left,
    Straight,
    Right,
}

#[derive(Clone, Copy)]
struct Cart {
    direction: Direction,
    position: Position,
    next_crossroad: Crossroad,
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.position.1.cmp(&self.position.1) {
            Ordering::Equal => other.position.0.cmp(&self.position.0),
            other => other,
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Cart {}

impl Cart {
    fn new(position: Position, direction: Direction) -> Self {
        Cart {
            direction,
            position,
            next_crossroad: Crossroad::Right,
        }
    }
    fn next_crossroad(&mut self) -> Crossroad {
        self.next_crossroad = match self.next_crossroad {
            Crossroad::Left => Crossroad::Straight,
            Crossroad::Straight => Crossroad::Right,
            Crossroad::Right => Crossroad::Left,
        };
        self.next_crossroad
    }
    fn next_step(&mut self, map: &HashMap<Position, Connections>) {
        self.position = match self.direction {
            Direction::Up => (self.position.0, self.position.1 - 1),
            Direction::Down => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
            Direction::Right => (self.position.0 + 1, self.position.1),
        };
        let track = map
            .get(&self.position)
            .expect(&format!("Car out of tracks: {:?}", self.position));
        self.direction = if track.is_crossroad() {
            match self.next_crossroad() {
                Crossroad::Straight => self.direction,
                Crossroad::Left => self.direction.rotate(false),
                Crossroad::Right => self.direction.rotate(true),
            }
        } else {
            let mut direction = None;
            for &d in &[
                self.direction,
                self.direction.rotate(false),
                self.direction.rotate(true),
            ] {
                if track.has_direction(d) {
                    direction = Some(d);
                    break;
                }
            }
            if let Some(d) = direction {
                d
            } else {
                panic!("Invalid track, position and/or direction");
            }
        }
    }
}

fn parse_map(map: &str) -> (HashMap<Position, Connections>, BinaryHeap<Cart>) {
    let mut tracks: Tracks = HashMap::new();
    let mut carts = BinaryHeap::new();
    // Utility functions
    let is_left_connected = |(x, y): (usize, usize), tracks: &Tracks| {
        if let Some(left) = tracks.get(&(x.wrapping_sub(1), y)) {
            left.has_direction(Direction::Right)
        } else {
            false
        }
    };
    let is_right_connected = |(x, y): (usize, usize), tracks: &Tracks| {
        if let Some(left) = tracks.get(&(x + 1, y)) {
            left.has_direction(Direction::Left)
        } else {
            false
        }
    };
    let is_up_connected = |(x, y): (usize, usize), tracks: &Tracks| {
        if let Some(left) = tracks.get(&(x, y.wrapping_sub(1))) {
            left.has_direction(Direction::Down)
        } else {
            false
        }
    };
    let is_down_connected = |(x, y): (usize, usize), tracks: &Tracks| {
        if let Some(left) = tracks.get(&(x, y + 1)) {
            left.has_direction(Direction::Up)
        } else {
            false
        }
    };
    // First pass
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut connections = None;
            let mut cart = None;
            match c {
                '-' => {
                    connections = Some(connections![Direction::Left, Direction::Right]);
                }
                '|' => {
                    connections = Some(connections![Direction::Up, Direction::Down]);
                }
                '/' => {
                    connections = if x == 0
                        || match tracks.get(&(x - 1, y)) {
                            Some(track) => !track.has_direction(Direction::Right),
                            None => true,
                        } {
                        Some(connections![Direction::Down, Direction::Right])
                    } else {
                        Some(connections![Direction::Up, Direction::Left])
                    };
                }
                '\\' => {
                    connections = if x == 0
                        || match tracks.get(&(x - 1, y)) {
                            Some(track) => !track.has_direction(Direction::Right),
                            None => true,
                        } {
                        Some(connections![Direction::Up, Direction::Right])
                    } else {
                        Some(connections![Direction::Down, Direction::Left])
                    };
                }
                '+' => {
                    connections = Some(connections![
                        Direction::Up,
                        Direction::Down,
                        Direction::Right,
                        Direction::Left
                    ]);
                }
                '^' => {
                    // Figure out straight, crossroad or curve later
                    connections = Some(connections![Direction::Up]);
                    cart = Some(Direction::Up);
                }
                'v' => {
                    // Figure out straight, crossroad or curve later
                    connections = Some(connections![Direction::Down]);
                    cart = Some(Direction::Down);
                }
                '<' => {
                    // Figure out straight, crossroad or curve later
                    connections = Some(connections![Direction::Left]);
                    cart = Some(Direction::Left);
                }
                '>' => {
                    // Figure out straight, crossroad or curve later
                    connections = Some(connections![Direction::Right]);
                    cart = Some(Direction::Right);
                }
                _ if c.is_whitespace() => {}
                _ => panic!("Invalid character in map: {}", c),
            }
            if let Some(connections) = connections {
                tracks.insert((x, y), connections);
            }
            if let Some(direction) = cart {
                carts.push(Cart::new((x, y), direction));
            }
        }
    }
    // Second pass, figure out straight, crossroad or curve for positions with carts
    for cart in carts.iter() {
        let connections = match cart.direction {
            Direction::Up => {
                match (
                    is_left_connected(cart.position, &tracks),
                    is_right_connected(cart.position, &tracks),
                ) {
                    (true, true) => connections![
                        Direction::Up,
                        Direction::Down,
                        Direction::Right,
                        Direction::Left
                    ],
                    (true, false) => connections![Direction::Up, Direction::Left],
                    (false, true) => connections![Direction::Up, Direction::Right],
                    _ => connections![Direction::Up, Direction::Down],
                }
            }
            Direction::Down => {
                match (
                    is_left_connected(cart.position, &tracks),
                    is_right_connected(cart.position, &tracks),
                ) {
                    (true, true) => connections![
                        Direction::Up,
                        Direction::Down,
                        Direction::Right,
                        Direction::Left
                    ],
                    (true, false) => connections![Direction::Down, Direction::Left],
                    (false, true) => connections![Direction::Down, Direction::Right],
                    _ => connections![Direction::Down, Direction::Up],
                }
            }
            Direction::Left => {
                match (
                    is_up_connected(cart.position, &tracks),
                    is_down_connected(cart.position, &tracks),
                ) {
                    (true, true) => connections![
                        Direction::Up,
                        Direction::Down,
                        Direction::Right,
                        Direction::Left
                    ],
                    (true, false) => connections![Direction::Left, Direction::Up],
                    (false, true) => connections![Direction::Left, Direction::Down],
                    _ => connections![Direction::Left, Direction::Right],
                }
            }
            Direction::Right => {
                match (
                    is_up_connected(cart.position, &tracks),
                    is_down_connected(cart.position, &tracks),
                ) {
                    (true, true) => connections![
                        Direction::Up,
                        Direction::Down,
                        Direction::Right,
                        Direction::Left
                    ],
                    (true, false) => connections![Direction::Right, Direction::Up],
                    (false, true) => connections![Direction::Right, Direction::Down],
                    _ => connections![Direction::Right, Direction::Left],
                }
            }
        };
        tracks.insert(cart.position, connections);
    }
    (tracks, carts)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day13.txt"))?.read_to_string(&mut input)?;
    let (tracks, mut carts) = parse_map(&input);
    let mut carts_next = BinaryHeap::with_capacity(carts.len());
    loop {
        while let Some(mut cart) = carts.pop() {
            cart.next_step(&tracks);
            for other in carts.iter().chain(carts_next.iter()) {
                if &cart == other {
                    println!("{},{}", cart.position.0, cart.position.1);
                    return Ok(());
                }
            }
            carts_next.push(cart);
        }
        std::mem::swap(&mut carts, &mut carts_next);
    }
}

#[allow(dead_code)]
fn print_state(map: &HashMap<Position, Connections>, carts: &[Cart]) {
    use std::iter::FromIterator;
    let cart_directions: HashMap<Position, Direction> =
        HashMap::from_iter(carts.iter().map(|cart| (cart.position, cart.direction)));
    let mut positions = map.keys().collect::<Vec<_>>();
    positions.sort_unstable_by(|p1, p2| match p1.1.cmp(&p2.1) {
        Ordering::Equal => p1.0.cmp(&p2.0),
        x => x,
    });
    let mut current = (0, 0);
    for position in positions {
        if position.1 != current.1 {
            for _ in 0..position.1 - current.1 {
                println!();
            }
            current.1 = position.1;
            current.0 = 0;
        }
        if position.0 != current.0 {
            for _ in 0..position.0 - current.0 {
                print!(" ");
            }
            current.0 = position.0;
        }
        print!(
            "{}",
            if let Some(direction) = cart_directions.get(&position) {
                direction.draw()
            } else {
                map.get(&position).unwrap().draw()
            }
        );
        current.0 += 1;
    }
    println!();
}

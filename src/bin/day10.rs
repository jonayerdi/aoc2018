use std::ops::{AddAssign, SubAssign};

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

fn parse_i32<T>(iter: &mut T, ending: char) -> i32
where
    T: Iterator<Item = char>,
{
    let mut chars = String::with_capacity(12);
    iter.find(|e| {
        if e.is_digit(10) || e == &'-' {
            chars.push(*e);
            false
        } else if e.is_whitespace() {
            false
        } else {
            assert_eq!(*e, ending);
            true
        }
    });
    chars.parse::<i32>().unwrap()
}

fn height(positions: &[Point]) -> i32 {
    let limits = positions
        .iter()
        .fold((i32::max_value(), i32::min_value()), |(min, max), p| {
            (min.min(p.y), max.max(p.y))
        });
    limits.1 - limits.0
}

fn print_message(positions: &[Point]) {
    let mut positions = Vec::from(positions);
    positions.sort_by(|p1, p2| p1.x.cmp(&p2.x));
    let min_x = positions.first().unwrap().x;
    let max_x = positions.last().unwrap().x;
    positions.sort_by(|p1, p2| p1.y.cmp(&p2.y));
    let min_y = positions.first().unwrap().y;
    let max_y = positions.last().unwrap().y;
    let min = Point { x: min_x, y: min_y };
    positions.iter_mut().for_each(|p| *p -= min);
    let max_x = max_x - min_x;
    let max_y = max_y - min_y;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if positions
                .binary_search_by(|p| {
                    use std::cmp::Ordering;
                    let ycmp = p.y.cmp(&y);
                    if ycmp == Ordering::Equal {
                        p.x.cmp(&x)
                    } else {
                        ycmp
                    }
                })
                .is_ok()
            {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[allow(non_upper_case_globals)]
const teststr: &str = include_str!("day10.txt");

const MAX_ITERATIONS: usize = 11000;

fn main() {
    let mut positions = Vec::with_capacity(32);
    let mut velocities = Vec::with_capacity(32);
    teststr
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            chars.nth("position=".len()).unwrap();
            let position = Point {
                x: parse_i32(&mut chars, ','),
                y: parse_i32(&mut chars, '>'),
            };
            chars.nth(" velocity=".len()).unwrap();
            let velocity = Point {
                x: parse_i32(&mut chars, ','),
                y: parse_i32(&mut chars, '>'),
            };
            (position, velocity)
        })
        .for_each(|(position, velocity)| {
            positions.push(position);
            velocities.push(velocity);
        });
    let mut prev_positions = positions.clone();
    let mut prev_height = height(&positions);
    for _ in 0..MAX_ITERATIONS {
        let height = height(&positions);
        if height > prev_height {
            print_message(&prev_positions);
            break;
        }
        prev_positions = positions.clone();
        prev_height = height;
        positions
            .iter_mut()
            .zip(velocities.iter())
            .for_each(|(position, velocity)| {
                *position += *velocity;
            });
    }
}

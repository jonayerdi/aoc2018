use std::fs::File;
use std::io::{self, Read};
use std::ops::AddAssign;
use std::path::PathBuf;

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

const MAX_ITERATIONS: usize = 11000;

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day10.txt"))?.read_to_string(&mut input)?;
    let mut positions = Vec::with_capacity(32);
    let mut velocities = Vec::with_capacity(32);
    input
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
    let mut prev_height = height(&positions);
    for iteration in 0..MAX_ITERATIONS {
        let height = height(&positions);
        if height > prev_height {
            println!("{}", iteration - 1);
            break;
        }
        prev_height = height;
        positions
            .iter_mut()
            .zip(velocities.iter())
            .for_each(|(position, velocity)| {
                *position += *velocity;
            });
    }
    Ok(())
}

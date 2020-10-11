use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

type Range = std::ops::Range<usize>;
type Position = (usize, usize);

fn manhatan(p1: Position, p2: Position) -> usize {
    p1.0.max(p2.0) - p1.0.min(p2.0) + p1.1.max(p2.1) - p1.1.min(p2.1)
}

fn find_closest_danger<'a, I>(dangers: I, position: Position) -> Option<usize>
where
    I: Iterator<Item = &'a Position>,
{
    let mut dangers = dangers;
    let first = *dangers.next()?;
    Some(
        dangers
            .enumerate()
            .fold(
                (Some((0, first)), manhatan(position, first)),
                |closest, (index, &danger)| {
                    let index = index + 1;
                    let distance = manhatan(position, danger);
                    if distance < closest.1 {
                        (Some((index, danger)), distance)
                    } else if distance == closest.1 {
                        (None, distance)
                    } else {
                        closest
                    }
                },
            )
            .0?
            .0,
    )
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day6.txt"))?.read_to_string(&mut input)?;
    let dangers = input
        .lines()
        .map(|line| {
            let mut nums = line.split(',').map(|n| n.trim().parse::<usize>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect::<Vec<_>>();

    let bbox = dangers
        .iter()
        .fold(None, |bounds: Option<(Range, Range)>, danger| {
            if let Some(bounds) = bounds {
                Some((
                    bounds.0.start.min(danger.0)..bounds.0.end.max(danger.0),
                    bounds.1.start.min(danger.1)..bounds.1.end.max(danger.1),
                ))
            } else {
                Some((danger.0..danger.0, danger.1..danger.1))
            }
        })
        .unwrap();

    let mut relevant_dangers = HashMap::with_capacity(dangers.len());
    (0..dangers.len()).for_each(|e| {
        relevant_dangers.insert(e, 0);
    });

    let mut filter_dangers = |position| {
        if let Some(index) = find_closest_danger(dangers.iter(), position) {
            relevant_dangers.remove(&index);
        }
    };

    for x in bbox.0.clone() {
        filter_dangers((x, bbox.1.start));
        filter_dangers((x, bbox.1.end));
    }
    for y in bbox.1.clone() {
        filter_dangers((bbox.0.start, y));
        filter_dangers((bbox.0.end, y));
    }

    for x in bbox.0.clone().skip(1) {
        for y in bbox.1.clone().skip(1) {
            if let Some(closest) = find_closest_danger(dangers.iter(), (x, y)) {
                if let Some(danger) = relevant_dangers.get_mut(&closest) {
                    *danger += 1;
                }
            }
        }
    }

    let largest = relevant_dangers
        .iter()
        .max_by(|(_, count1), (_, count2)| count1.cmp(count2));

    if let Some(largest) = largest {
        println!("{}", largest.1);
    }
    Ok(())
}

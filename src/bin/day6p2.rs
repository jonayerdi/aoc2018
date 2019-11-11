type Range = std::ops::Range<usize>;
type Position = (usize, usize);

fn manhatan(p1: Position, p2: Position) -> usize {
    p1.0.max(p2.0) - p1.0.min(p2.0) + p1.1.max(p2.1) - p1.1.min(p2.1)
}

#[allow(non_upper_case_globals)]
const teststr: &str = include_str!("day6.txt");

fn main() {
    let dangers = teststr
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

    let mut areas = 0;

    for x in bbox.0.clone() {
        for y in bbox.1.clone() {
            if dangers
                .iter()
                .map(|&danger| manhatan((x, y), danger))
                .fold(0, |sum, e| sum + e)
                < 10000
            {
                areas += 1;
            }
        }
    }

    println!("{}", areas);
}

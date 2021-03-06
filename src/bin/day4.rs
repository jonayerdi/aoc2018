use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

enum RecordType {
    Shift(usize),
    Sleep,
    Wake,
}

impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordType::Shift(id) => write!(f, "Guard #{} begins shift", id),
            RecordType::Sleep => write!(f, "falls asleep"),
            RecordType::Wake => write!(f, "wakes up"),
        }
    }
}

struct Record {
    pub date: (usize, usize, usize),
    pub time: (usize, usize),
    pub info: RecordType,
}

impl Record {
    fn parse(line: &str) -> Record {
        let mut chars = line.chars();
        assert_eq!(chars.next(), Some('['));
        let date = (
            parse_usize(&mut chars, '-'),
            parse_usize(&mut chars, '-'),
            parse_usize(&mut chars, ' '),
        );
        let time = (parse_usize(&mut chars, ':'), parse_usize(&mut chars, ']'));
        assert_eq!(chars.next(), Some(' '));
        let info = match chars.next() {
            Some('G') => {
                assert_eq!(chars.find(|c| *c == '#'), Some('#'));
                RecordType::Shift(parse_usize(&mut chars, ' '))
            }
            Some('f') => RecordType::Sleep,
            Some('w') => RecordType::Wake,
            _ => panic!("Invalid message in line: \"{}\"", line),
        };
        Record { date, time, info }
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        for (n1, n2) in [
            self.date.0,
            self.date.1,
            self.date.2,
            self.time.0,
            self.time.1,
        ]
        .iter()
        .zip(
            [
                other.date.0,
                other.date.1,
                other.date.2,
                other.time.0,
                other.time.1,
            ]
            .iter(),
        ) {
            let order = n1.cmp(n2);
            if order != Ordering::Equal {
                return order;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Record {}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:04}-{:02}-{:02} {:02}:{:02}] {}",
            self.date.0, self.date.1, self.date.2, self.time.0, self.time.1, self.info
        )
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
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day4.txt"))?.read_to_string(&mut input)?;
    let mut records: Vec<_> = input.lines().map(|line| Record::parse(line)).collect();
    records.sort_unstable();
    let mut events = HashMap::with_capacity(32);
    let mut guard = 0;
    let mut sleep_minute = 0;
    for record in records.iter() {
        match record.info {
            RecordType::Shift(id) => guard = id,
            RecordType::Sleep => sleep_minute = record.time.1,
            RecordType::Wake => events
                .entry(guard)
                .or_insert(Vec::with_capacity(32))
                .push(sleep_minute..record.time.1),
        }
    }
    let total_minutes: HashMap<usize, usize> = events
        .iter()
        .map(|(guard, days)| {
            (
                *guard,
                days.iter().map(|asleep| asleep.end - asleep.start).sum(),
            )
        })
        .collect();
    let guard = total_minutes
        .iter()
        .fold(
            (&0, &0),
            |max, current| if current.1 > max.1 { current } else { max },
        )
        .0;
    let minutes: Vec<_> = (0..60)
        .map(|hour| {
            if let Some(days) = events.get(guard) {
                days.iter().filter(|asleep| asleep.contains(&hour)).count()
            } else {
                panic!("Guard #{} has no entry in events list", guard)
            }
        })
        .collect();
    let max_minute = minutes
        .iter()
        .enumerate()
        .fold(
            (0, &0),
            |max, current| if current.1 > max.1 { current } else { max },
        )
        .0;
    //println!("{} * {} = {}", guard, max_minute, guard * max_minute);
    println!("{}", guard * max_minute);
    Ok(())
}

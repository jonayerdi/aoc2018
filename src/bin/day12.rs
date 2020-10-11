use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

const RULE_LEFT: usize = 2;
const RULE_RIGHT: usize = 2;
const RULE_LENGTH: usize = RULE_LEFT + 1 + RULE_RIGHT;

const GENERATIONS: usize = 20;

const MAX_LEFT_EXPANSION: usize = GENERATIONS * RULE_RIGHT;
const MAX_RIGHT_EXPANSION: usize = GENERATIONS * RULE_LEFT;

#[allow(dead_code)]
fn print_state<'a, I>(state: I)
where
    I: Iterator<Item = &'a (i32, bool)>,
{
    for pot in state {
        print!("{}", if pot.1 { "#" } else { "." });
    }
    println!();
}

fn decode(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        c => panic!("invalid character decoded: \'{}\'", c),
    }
}

fn matches_rule(state: &[(i32, bool)], rules: &[[bool; RULE_LENGTH]]) -> bool {
    assert_eq!(state.len(), RULE_LENGTH);
    rules
        .iter()
        .find(|rule| {
            rule.iter()
                .zip(state)
                .find(|(v1, (_, v2))| **v1 != *v2)
                .is_none()
        })
        .is_some()
}

fn next_generation(state: &mut [(i32, bool)], rules: &[[bool; RULE_LENGTH]]) {
    const WINDOW_SIZE: usize = RULE_LEFT + 1;
    let mut next_values = [false; WINDOW_SIZE];
    for index in RULE_LEFT..RULE_LEFT + WINDOW_SIZE {
        let next_value = &mut next_values[index % WINDOW_SIZE];
        *next_value = matches_rule(&state[index - RULE_LEFT..=index + RULE_RIGHT], rules);
    }
    for index in RULE_LEFT + WINDOW_SIZE..state.len() - 1 - RULE_RIGHT {
        let next_value = &mut next_values[index % WINDOW_SIZE];
        state[index - WINDOW_SIZE].1 = *next_value;
        *next_value = matches_rule(&state[index - RULE_LEFT..=index + RULE_RIGHT], rules);
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open(PathBuf::from("data").join("day12.txt"))?.read_to_string(&mut input)?;
    let mut lines = input.lines();
    // Read initial state
    let state = lines
        .next()
        .unwrap()
        .chars()
        .skip("initial state: ".len())
        .map(|c| decode(c))
        .enumerate()
        .map(|(index, value)| (index as i32, value))
        .collect::<Vec<_>>();
    let state_len = state.len();
    // Expand to maximum final size
    let left =
        (0..MAX_LEFT_EXPANSION).map(|index| (index as i32 - MAX_LEFT_EXPANSION as i32, false));
    let right = (0..MAX_RIGHT_EXPANSION).map(|index| (index as i32 + state_len as i32, false));
    let mut state = left.chain(state).chain(right).collect::<Vec<_>>();
    // Skip empty line
    lines.next().unwrap();
    // Read rules, store only the rules that result in a plant
    let rules = lines
        .map(|line| {
            let mut chars = line.chars();
            let mut rule = [false; RULE_LENGTH];
            for e in rule.iter_mut() {
                *e = decode(chars.next().unwrap());
            }
            chars.nth(" =>".len()).unwrap();
            let result = decode(chars.next().unwrap());
            (rule, result)
        })
        .filter(|(_, result)| *result)
        .map(|(rule, _)| rule)
        .collect::<Vec<_>>();
    // Print initial state
    //print_state(state.iter());
    // Iterate over generations
    for _ in 0..GENERATIONS {
        next_generation(&mut state, &rules);
        //print_state(state.iter());
    }
    // Calculate the sum of pot numbers with plants
    let result = state
        .iter()
        .filter(|(_, plant)| *plant)
        .fold(0, |sum, (num, _)| sum + num);
    println!("{}", result);
    Ok(())
}

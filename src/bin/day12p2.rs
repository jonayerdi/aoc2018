#[allow(non_upper_case_globals)]
const teststr: &str = include_str!("day12.txt");

const RULE_LEFT: usize = 2;
const RULE_RIGHT: usize = 2;
const RULE_LENGTH: usize = RULE_LEFT + 1 + RULE_RIGHT;

const MAX_ITER_GENERATIONS: usize = 500;
const GENERATIONS: usize = 50000000000;

const MAX_LEFT_EXPANSION: usize = MAX_ITER_GENERATIONS * RULE_RIGHT;
const MAX_RIGHT_EXPANSION: usize = MAX_ITER_GENERATIONS * RULE_LEFT;

fn decode(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        c => panic!("invalid character decoded: \'{}\'", c),
    }
}

fn matches_rule(state: &[(i64, bool)], rules: &[[bool; RULE_LENGTH]]) -> bool {
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

fn next_generation(state: &mut [(i64, bool)], rules: &[[bool; RULE_LENGTH]]) {
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

fn get_offset(previous_state: &[(i64, bool)], state: &[(i64, bool)]) -> Option<i64> {
    let first_plant_state = state.iter().enumerate().find(|(_, (_, plant))| *plant);
    let first_plant_prev = previous_state
        .iter()
        .enumerate()
        .find(|(_, (_, plant))| *plant);
    if let (Some((index, _)), Some((index_prev, _))) = (first_plant_state, first_plant_prev) {
        let offset = index as i64 - index_prev as i64;
        let mut states = if offset >= 0 {
            previous_state
                .iter()
                .zip(state.iter().skip(offset as usize))
        } else {
            state
                .iter()
                .zip(previous_state.iter().skip(-offset as usize))
        };
        if states.find(|((_, p1), (_, p2))| p1 != p2).is_none() {
            Some(offset)
        } else {
            None
        }
    } else {
        if first_plant_state.is_none() && first_plant_prev.is_none() {
            Some(0)
        } else {
            None
        }
    }
}

fn main() {
    let mut lines = teststr.lines();
    // Read initial state
    let state = lines
        .next()
        .unwrap()
        .chars()
        .skip("initial state: ".len())
        .map(|c| decode(c))
        .enumerate()
        .map(|(index, value)| (index as i64, value))
        .collect::<Vec<_>>();
    let state_len = state.len();
    // Expand to maximum final size
    let left =
        (0..MAX_LEFT_EXPANSION).map(|index| (index as i64 - MAX_LEFT_EXPANSION as i64, false));
    let right = (0..MAX_RIGHT_EXPANSION).map(|index| (index as i64 + state_len as i64, false));
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
    // Store previous state
    let mut previous_state;
    let mut generations = 0;
    // Iterate over generations
    let offset = loop {
        previous_state = state.clone();
        next_generation(&mut state, &rules);
        generations += 1;
        if generations == GENERATIONS {
            break 0;
        }
        if let Some(offset) = get_offset(&previous_state, &state) {
            break offset;
        }
        if generations == MAX_ITER_GENERATIONS {
            panic!("State did not converge after MAX_ITER_GENERATIONS");
        }
    };
    // Calculate shift value
    let shift = offset * (GENERATIONS - generations) as i64;
    // Calculate the sum of pot numbers with plants
    let result = state
        .iter()
        .filter(|(_, plant)| *plant)
        .fold(0, |sum, (num, _)| sum + num + shift);
    println!("Result: {}", result);
}

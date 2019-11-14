use std::collections::btree_map::BTreeMap;
use std::collections::btree_set::BTreeSet;
use std::iter::FromIterator;

macro_rules! print_state {
    ($nodes:expr, $workers:expr, $result:expr) => {
        for worker in $workers.iter() {
            print!("|");
            if let Some(node) = worker {
                if let Node::Ongoing(remain) = $nodes.get(node).unwrap() {
                    print!("{}{:02}", node, remain);
                }
            } else {
                print!("---");
            }
        }
        println!("| {:04}", $result);
    };
}

fn node2time(node: char) -> usize {
    61 + (node as usize - 'A' as usize)
}

fn try_find_work(nodes: &mut BTreeMap<char, Node>, worker: &mut Option<char>) {
    if worker.is_none() {
        let node = nodes.iter_mut().find(|(_, state)| match state {
            Node::Available(_) => true,
            _ => false,
        });
        *worker = match node {
            Some(node) => {
                if let Node::Available(remain) = node.1 {
                    *node.1 = Node::Ongoing(*remain);
                }
                Some(*node.0)
            }
            None => None,
        };
    }
}

fn try_handle_work(nodes: &mut BTreeMap<char, Node>, worker: &mut Option<char>) {
    if let Some(node) = worker {
        let state = nodes.get_mut(node).unwrap();
        match state {
            Node::Ongoing(remain) => {
                if *remain > 1 {
                    *remain -= 1;
                } else {
                    nodes.remove(node);
                    nodes.iter_mut().for_each(|(candidate, state)| {
                        if let Node::Unavailable(requirements) = state {
                            if requirements.remove(node) && requirements.is_empty() {
                                *state = Node::Available(node2time(*candidate));
                            }
                        }
                    });
                    *worker = None;
                }
            }
            _ => panic!("invalid node state"),
        }
    }
}

#[derive(Debug)]
enum Node {
    Unavailable(BTreeSet<char>),
    Available(usize),
    Ongoing(usize),
}

#[allow(non_upper_case_globals)]
const teststr: &str = include_str!("day7.txt");

const WORKERS: usize = 5;

fn main() {
    let mut nodes = BTreeMap::new();
    let mut workers: Vec<_> = (0..WORKERS).map(|_| None).collect();
    let mut result = 0;

    teststr
        .lines()
        .map(|line| {
            let mut words = line.split(' ');
            (
                words.nth(1).unwrap().chars().next().unwrap(),
                words.nth(5).unwrap().chars().next().unwrap(),
            )
        })
        .for_each(|(requirement, step)| {
            nodes
                .entry(requirement)
                .or_insert(Node::Available(node2time(requirement)));
            let node = nodes
                .entry(step)
                .or_insert(Node::Unavailable(BTreeSet::new()));
            match node {
                Node::Unavailable(requirements) => {
                    requirements.insert(requirement);
                }
                _ => {
                    *node = Node::Unavailable(BTreeSet::from_iter(
                        [requirement].into_iter().map(|&c| c),
                    ));
                }
            }
        });

    while !nodes.is_empty() {
        for worker in workers.iter_mut() {
            try_handle_work(&mut nodes, worker);
        }
        for worker in workers.iter_mut() {
            try_find_work(&mut nodes, worker);
        }
        print_state!(nodes, workers, result);
        result += 1;
    }

    result -= 1;
    println!("{}", result);
}

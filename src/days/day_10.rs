use std::collections::HashSet;

use itertools::Itertools;
use regex::bytes::Regex;

use crate::utils::input::read_input;

fn parse_input(input: &str) -> Vec<(u16, Vec<u16>)> {
    input
        .lines()
        .map(|line| {
            let re = Regex::new(r"\[(.*)\] (.*) \{").unwrap();
            let cap = re.captures(line.as_bytes()).unwrap();
            let lights = cap.get(1).unwrap();
            let buttons = cap.get(2).unwrap();
            let lights: u16 = lights
                .as_bytes()
                .into_iter()
                .rev()
                .fold(0, |acc, l| acc * 2 + (*l == b'#') as u16);
            let buttons = buttons.as_bytes().split(|b| *b == b' ');
            let buttons: Vec<u16> = buttons
                .map(|b| {
                    let bits = b[1..b.len() - 1].split(|c| *c == b',');
                    bits.fold(0, |acc, b| acc + (1 << (b[0] - b'0')))
                })
                .collect();

            (lights, buttons)
        })
        .collect()
}

fn get_min_presses(lights: u16, buttons: &Vec<u16>) -> usize {
    for n in 1..buttons.len() {
        if buttons
            .iter()
            .combinations(n)
            .any(|buttons| buttons.iter().fold(0, |acc, b| acc ^ *b) == lights)
        {
            return n;
        }
    }

    return 0;
}

fn solve_part_1(input: &str) {
    let configs = parse_input(input);
    let min_presses: usize = configs
        .iter()
        .map(|(lights, buttons)| get_min_presses(*lights, buttons))
        .sum();

    println!("Fewest presses: {}", min_presses);
}

fn parse_input_v2(input: &str) -> Vec<(Vec<Vec<u16>>, Vec<u16>)> {
    input
        .lines()
        .map(|line| {
            let re = Regex::new(r"\[(.*)\] (.*) \{(.*)\}").unwrap();
            let cap = re.captures(line.as_bytes()).unwrap();
            let buttons = cap.get(2).unwrap();
            let jolts = cap.get(3).unwrap();
            let buttons = buttons.as_bytes().split(|b| *b == b' ');
            let buttons: Vec<Vec<u16>> = buttons
                .map(|b| {
                    let bids = b[1..b.len() - 1].split(|c| *c == b',');
                    bids.map(|bid| (bid[0] - b'0') as u16).collect()
                })
                .collect();
            let jolts = jolts.as_bytes().split(|b| *b == b',');
            let jolts = jolts
                .map(|j| str::from_utf8(j).unwrap().parse::<u16>().unwrap())
                .collect();

            (buttons, jolts)
        })
        .collect()
}

fn fits_jolts(buttons: &Vec<Vec<u16>>, jolts: &Vec<u16>, branch: &Vec<u16>) -> bool {
    let mut calculated_jolts: Vec<u16> = vec![0; jolts.len()];
    for (bid, presses) in branch.iter().enumerate() {
        for jid in buttons[bid].iter() {
            calculated_jolts[*jid as usize] += presses;
        }
    }
    for i in 0..jolts.len() {
        if jolts[i] != calculated_jolts[i] {
            return false;
        }
    }
    true
}

fn exceeds_jolts(buttons: &Vec<Vec<u16>>, jolts: &Vec<u16>, branch: &Vec<u16>) -> bool {
    let mut calculated_jolts: Vec<u16> = vec![0; jolts.len()];
    for (bid, presses) in branch.iter().enumerate() {
        for jid in buttons[bid].iter() {
            let jid = *jid as usize;
            calculated_jolts[jid] += presses;
            if calculated_jolts[jid] > jolts[jid] {
                return true;
            }
        }
    }
    false
}

fn get_min_presses_v2(buttons: &Vec<Vec<u16>>, jolts: &Vec<u16>) -> usize {
    let mut branches: HashSet<Vec<u16>> = HashSet::new();
    branches.insert(vec![0; buttons.len()]);
    let mut presses = 0;
    loop {
        presses += 1;
        println!("Turn: {}, branches: {}", presses, branches.len());
        let mut new_branches: HashSet<Vec<u16>> = HashSet::new();
        for b in branches.iter() {
            for n in 0..buttons.len() {
                let mut new = b.clone();
                new[n] += 1;
                if fits_jolts(buttons, jolts, &new) {
                    return presses;
                }
                if !exceeds_jolts(buttons, jolts, &new) {
                    new_branches.insert(new);
                }
            }
        }
        branches = new_branches;
    }
}

fn get_highest_score_button_id(buttons: &Vec<Vec<u16>>, jolts: &Vec<u16>) -> usize {
    buttons
        .iter()
        .enumerate()
        .filter_map(|(bid, butt)| {
            if butt.iter().any(|bid| jolts[*bid as usize] == 0) {
                None
            } else {
                Some((
                    bid,
                    butt.iter()
                        .map(|jid| {
                            let j = jolts[*jid as usize] as u64;
                            j * j
                        })
                        .sum::<u64>(),
                ))
            }
        })
        .inspect(|(bid, score)| println!("Score {} for {:?}", *score, buttons[*bid]))
        .max_by_key(|(_, score)| *score)
        .unwrap()
        .0
}

fn push_button(jolts: &mut Vec<u16>, button: &Vec<u16>) {
    for b in button.iter() {
        jolts[*b as usize] -= 1;
    }
}

fn get_min_presses_v3(buttons: &Vec<Vec<u16>>, jolts: &Vec<u16>) -> usize {
    let mut jolts = jolts.clone();
    let mut turns = 0;
    while jolts.iter().any(|&j| j != 0) {
        turns += 1;
        println!("Turn {}: {:?}", turns, jolts);
        let bid = get_highest_score_button_id(buttons, &jolts);
        push_button(&mut jolts, &buttons[bid]);
    }

    println!("{turns} turns");
    turns
}

fn solve_part_2(input: &str) {
    let configs = parse_input_v2(input);
    let min_presses: usize = configs
        .iter()
        .enumerate()
        .map(|(n, bj)| {
            print!("Line {}: ", n);
            bj
        })
        .map(|(buttons, jolts)| get_min_presses_v3(buttons, jolts))
        .sum();

    println!("Fewest presses: {}", min_presses);
}

pub fn part_1() {
    let input = read_input(module_path!());
    solve_part_1(input.as_str());
}

pub fn part_2() {
    let input = read_input(module_path!());
    solve_part_2(input.as_str());
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = EXAMPLE_1;

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}

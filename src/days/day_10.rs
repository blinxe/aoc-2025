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

fn parse_input_v2(input: &str) -> Vec<(Vec<Vec<u32>>, Vec<i32>)> {
    input
        .lines()
        .map(|line| {
            let re = Regex::new(r"\[(.*)\] (.*) \{(.*)\}").unwrap();
            let cap = re.captures(line.as_bytes()).unwrap();
            let buttons = cap.get(2).unwrap();
            let jolts = cap.get(3).unwrap();
            let buttons = buttons.as_bytes().split(|b| *b == b' ');
            let buttons: Vec<Vec<u32>> = buttons
                .map(|b| {
                    let bids = b[1..b.len() - 1].split(|c| *c == b',');
                    bids.map(|bid| (bid[0] - b'0') as u32).collect()
                })
                .collect();
            let jolts = jolts.as_bytes().split(|b| *b == b',');
            let jolts = jolts
                .map(|j| str::from_utf8(j).unwrap().parse::<i32>().unwrap())
                .collect();

            (buttons, jolts)
        })
        .collect()
}

fn get_binary_buttons(buttons: &[Vec<u32>]) -> Vec<u32> {
    buttons
        .iter()
        .map(|b| b.iter().map(|n| 1u32 << n).sum())
        .collect()
}

fn get_combinations<T: Copy>(set: &[T], count: usize) -> Vec<Vec<T>> {
    if count == 0 {
        vec![Vec::new()]
    } else {
        set[..set.len() - count + 1]
            .iter()
            .enumerate()
            .flat_map(|(i, &t)| {
                get_combinations(&set[i + 1..], count - 1)
                    .iter()
                    .map(|c| {
                        let mut c1 = c.clone();
                        c1.push(t);
                        c1
                    })
                    .collect::<Vec<Vec<T>>>()
            })
            .collect()
    }
}

fn subsets<T: Copy>(set: &[T]) -> Vec<Vec<T>> {
    let mut subsets: Vec<Vec<T>> = Vec::new();
    for count in 0..=set.len() {
        subsets.extend(get_combinations(set, count));
    }
    subsets
}

// I gave up, went to Reddit and found this hint:
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
// > find all possible sets of buttons you can push so that the remaining voltages are even, and divide by 2 and recurse.
fn fewest_joltage_presses(buttons: &Vec<Vec<u32>>, jolts: &Vec<i32>) -> usize {
    let binary_buttons = get_binary_buttons(buttons);
    let subset_xors: Vec<_> = subsets(&binary_buttons)
        .iter()
        .map(|subset| (subset.to_owned(), subset.iter().fold(0, |a, &b| a ^ b)))
        .collect();
    fewest_joltage_presses_recur(&subset_xors, jolts).unwrap()
}

fn fewest_joltage_presses_recur(
    subset_xors: &[(Vec<u32>, u32)],
    joltages: &[i32],
) -> Option<usize> {
    if joltages.iter().all(|&j| j == 0) {
        return Some(0);
    }
    let binary_joltages = get_binary_joltages(joltages);
    let mut best = None;
    for (subset, xor) in subset_xors {
        if *xor == binary_joltages {
            let new_joltages = get_new_joltages(joltages, &subset);
            if new_joltages.iter().all(|&j| j >= 0) {
                let press_count = fewest_joltage_presses_recur(subset_xors, &new_joltages)
                    .map(|c| subset.len() + 2 * c);
                best = best.min(press_count).or(best).or(press_count);
            }
        }
    }
    best
}

fn get_new_joltages(joltages: &[i32], subset: &[u32]) -> Vec<i32> {
    let mut new_joltages = Vec::new();
    let mut mask = 1;
    for &joltage in joltages {
        new_joltages.push((joltage - subset.iter().filter(|&b| b & mask != 0).count() as i32) / 2);
        mask <<= 1;
    }
    new_joltages
}

fn get_binary_joltages(joltages: &[i32]) -> u32 {
    joltages
        .iter()
        .enumerate()
        .map(|(i, j)| ((1 << i) * (j % 2)) as u32)
        .sum()
}

fn solve_part_2(input: &str) {
    let configs = parse_input_v2(input);
    let min_presses: usize = configs
        .iter()
        // .enumerate()
        // .map(|(n, bj)| {
        //     print!("Line {}: ", n);
        //     bj
        // })
        .map(|(buttons, jolts)| fewest_joltage_presses(buttons, jolts))
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

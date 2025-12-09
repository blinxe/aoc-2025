use std::ops::Range;

use crate::utils::input::read_input;

type Operand = i64;

fn parse_input(input: &str) -> Vec<(Vec<Operand>, char)> {
    let nrows = input.lines().count() - 1;
    let ncols = input.lines().next().unwrap().split_whitespace().count();

    let mut cols: Vec<Vec<Operand>> = vec![vec![0; nrows]; ncols];

    for (ln, line) in input.lines().take(nrows).enumerate() {
        for (wn, word) in line.split_whitespace().enumerate() {
            cols[wn][ln] = word.parse::<Operand>().unwrap();
        }
    }

    let ops: Vec<char> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    cols.into_iter().zip(ops).collect()
}

fn solve_part_1(input: &str) {
    let ops = parse_input(input);
    // println!("{:?}", bla);

    let sum: Operand = ops
        .iter()
        .map(|(operands, operator)| {
            if *operator == '+' {
                operands.iter().sum::<Operand>()
            } else {
                operands.iter().product::<Operand>()
            }
        })
        .sum();

    println!("Grand total: {}", sum);
}

fn parse_input_v2(input: &str) -> (Vec<String>, Vec<(Range<usize>, char)>) {
    let nrows = input.lines().count() - 1;
    let ncols = input.lines().next().unwrap().split_whitespace().count();

    let mut positions: Vec<(usize, char)> = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .collect();
    positions.push((input.lines().next().unwrap().len() + 1, ' '));

    let mut cols: Vec<(Range<usize>, char)> = Vec::new();
    for nc in 0..ncols {
        cols.push((positions[nc].0..positions[nc + 1].0 - 1, positions[nc].1));
    }

    let lines: Vec<String> = input
        .lines()
        .take(nrows)
        .map(|line| line.to_owned())
        .collect();

    (lines, cols)
}

fn solve_part_2(input: &str) {
    let (lines, cols) = parse_input_v2(input);
    // println!("{:?}", cols);

    let total: Operand = cols
        .into_iter()
        .map(|(ran, op)| {
            let numbers = ran.map(|pos| {
                let mut acc: Operand = 0;
                for ln in 0..lines.len() {
                    match lines[ln].chars().nth(pos) {
                        Some(' ') => (),
                        Some(c) => acc = 10 * acc + (c as Operand - '0' as Operand),
                        _ => (),
                    }
                }
                acc
            });
            if op == '+' {
                numbers.sum::<Operand>()
            } else {
                numbers.product::<Operand>()
            }
        })
        .sum();

    println!("Grand total: {}", total);
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
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   +  
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

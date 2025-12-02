use crate::utils::input::read_input;

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|range| {
            let mut min_max = range.split('-');
            let min_str = min_max.next().unwrap();
            let max_str = min_max.next().unwrap();
            (min_str.parse().unwrap(), max_str.parse().unwrap())
        })
        .collect()
}

fn is_invalid_v1(n: u64) -> bool {
    let s = format!("{}", n);
    let half = &s[..s.len() / 2];
    &s[s.len() / 2..] == half
}

fn is_invalid_v2(n: u64) -> bool {
    let s = format!("{}", n);

    for l in 1..=s.len() / 2 {
        if s.len() % l != 0 {
            continue;
        }
        let reps = s.len() / l;
        let sub = &s[..l];
        let mut repeats = true;
        for i in 1..reps {
            if &s[i * l..(i + 1) * l] != sub {
                repeats = false;
                break;
            }
        }
        if repeats {
            return true;
        }
    }

    false
}

fn solve_part_1(input: &str) {
    let parsed_input = parse_input(input);
    let mut result = 0;

    for (min, max) in parsed_input {
        for n in min..=max {
            if is_invalid_v1(n) {
                result += n;
            }
        }
    }

    println!("Part 1 result: {}", result);
}

fn solve_part_2(input: &str) {
    let parsed_input = parse_input(input);
    let mut result = 0;

    for (min, max) in parsed_input {
        for n in min..=max {
            if is_invalid_v2(n) {
                result += n;
            }
        }
    }

    println!("Part 2 result: {}", result);
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
    const EXAMPLE_1: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

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

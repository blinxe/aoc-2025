use crate::utils::input::read_input;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_max_digit(row: &[char]) -> (usize, char) {
    let (pos, digit) = row
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();

    (pos, *digit)
}

fn solve_part_1(input: &str) {
    let rows = parse_input(input);
    let total: u32 = rows
        .iter()
        .map(|row| {
            let (pos, d1) = &row[..row.len() - 1]
                .iter()
                .enumerate()
                .rev()
                .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
                .unwrap();
            let d2 = &row[pos + 1..].iter().max().unwrap();
            d1.to_digit(10).unwrap() * 10 + d2.to_digit(10).unwrap()
        })
        .sum();

    println!("{}", total);
}

fn solve_part_2(input: &str) {
    let rows = parse_input(input);
    let total: u64 = rows
        .iter()
        .map(|row| {
            let mut start = 0;
            let mut number: u64 = 0;
            for i in (1..=12).rev() {
                let subrow = &row[start..=row.len() - i];
                let (pos, digit) = find_max_digit(subrow);
                number = number * 10 + digit.to_digit(10).unwrap() as u64;
                start += pos + 1;
            }
            number
        })
        .sum();

    println!("{}", total);
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
    const EXAMPLE_1: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

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

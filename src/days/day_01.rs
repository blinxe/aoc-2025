use crate::utils::input::read_input;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| {
            let num: i32 = l[1..].parse().unwrap();
            match l.chars().next().unwrap() {
                'R' => num,
                'L' => -num,
                _ => panic!("invalid input"),
            }
        })
        .collect()
}

fn solve_part_1(input: &str) {
    let rotations = parse_input(input);
    let mut count = 0;
    let mut value = 50;

    for num in rotations {
        value = (value + num + 100) % 100;
        if value == 0 {
            count += 1;
        }
    }

    println!("Number of 0s: {}", count);
}

fn solve_part_2(input: &str) {
    let rotations = parse_input(input);
    let mut count = 0;
    let mut value = 50;

    for num in rotations {
        count += num.abs() / 100;
        let rem = num % 100;
        if value != 0 && (value + rem <= 0 || value + rem > 99) {
            count += 1;
        }
        value = (value + rem + 100) % 100;
    }

    println!("Number of 0s: {}", count);
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
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
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

use crate::utils::input::read_input;
use std::ops::Range;

fn parse_input(input: &str) -> (Vec<Range<u64>>, Vec<u64>) {
    let mut split = input.split("\n\n");
    let ranges = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut bounds = line.split('-');
            let lower: u64 = bounds.next().unwrap().parse().unwrap();
            let upper: u64 = bounds.next().unwrap().parse().unwrap();
            lower..upper + 1
        })
        .collect();
    let ids: Vec<u64> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ids)
}

fn solve_part_1(input: &str) {
    let (ranges, ids) = parse_input(input);

    let fresh_number = ids
        .iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count();

    println!("Fresh products: {}", fresh_number);
}

fn intersect(r1: &Range<u64>, r2: &Range<u64>) -> bool {
    r1.start < r2.end && r1.end > r2.start
}

fn merge(r1: &Range<u64>, r2: &Range<u64>) -> Range<u64> {
    let lower = r1.start.min(r2.start);
    let upper = r1.end.max(r2.end);

    lower..upper
}

fn solve_part_2(input: &str) {
    let (mut ranges, _) = parse_input(input);

    for i1 in 0..ranges.len() - 1 {
        for i2 in i1 + 1..ranges.len() {
            if intersect(&ranges[i1], &ranges[i2]) {
                ranges[i2] = merge(&ranges[i1], &ranges[i2]);
                ranges[i1] = 0..0; // make this range empty without removing it to preserve the loops
                break;
            }
        }
    }

    let fresh_number: u64 = ranges.iter().map(|r| r.end - r.start).sum();

    println!("Fresh products: {}", fresh_number);
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
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
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

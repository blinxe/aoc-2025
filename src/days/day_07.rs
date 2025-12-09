use std::{
    collections::{HashMap, HashSet},
    usize,
};

use crate::utils::input::read_input;

fn parse_input(input: &str) -> (Vec<HashSet<usize>>, usize) {
    let start_pos = input.find('S').unwrap();
    let splitters = input
        .lines()
        .skip(1)
        .map(|l| {
            l.char_indices()
                .filter_map(|(pos, char)| if char == '^' { Some(pos) } else { None })
                .collect()
        })
        .collect();

    (splitters, start_pos)
}

fn solve_part_1(input: &str) {
    let (splitters, start_pos) = parse_input(input);
    let mut booms: usize = 0;
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_pos);

    for s in splitters {
        let intersect: HashSet<usize> = beams.intersection(&s).cloned().collect();
        booms += intersect.len();
        for i in intersect {
            beams.remove(&i);
            beams.insert(i - 1);
            beams.insert(i + 1);
        }
    }

    println!("Booms: {:?}", booms);
}

fn solve_part_2(input: &str) {
    let (splitters, start_pos) = parse_input(input);
    let mut beams: HashMap<usize, u64> = HashMap::new();
    beams.insert(start_pos, 1);

    for s in splitters {
        let intersect: HashSet<usize> = beams
            .keys()
            .cloned()
            .collect::<HashSet<usize>>()
            .intersection(&s)
            .cloned()
            .collect();

        for i in intersect {
            let n = beams.remove(&i).unwrap();
            beams.entry(i - 1).and_modify(|v| *v += n).or_insert(n);
            beams.entry(i + 1).and_modify(|v| *v += n).or_insert(n);
        }
    }

    let timelines = beams.into_values().sum::<u64>();

    println!("Timelines: {:?}", timelines);
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
        .......S.......
        .......|.......
        ......|^|......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
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

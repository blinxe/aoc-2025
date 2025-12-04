use std::collections::HashSet;

use crate::utils::input::read_input;

fn parse_input(input: &str) -> (HashSet<(isize, isize)>, usize, usize) {
    let xsize = input.lines().next().unwrap().len();
    let ysize = input.lines().count();
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let set: HashSet<(isize, isize)> = line
                .char_indices()
                .filter_map(|(x, c)| match c {
                    '@' => Some((x as isize, y as isize)),
                    _ => None,
                })
                .collect();
            set
        })
        .collect();

    (map, xsize, ysize)
}

fn solve_part_1(input: &str) {
    let mut total = 0;
    let (map, _xsize, _ysize) = parse_input(input);

    for (x, y) in &map {
        let mut neighbours = 0;
        for (dx, dy) in [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ] {
            if map.contains(&(x + dx, y + dy)) {
                neighbours += 1;
            }
        }
        if neighbours < 4 {
            total += 1;
        }
    }

    println!("Accessible rolls: {}", total);
}

fn display_map(map: &HashSet<(isize, isize)>, xsize: usize, ysize: usize) -> () {
    for y in 0..ysize {
        for x in 0..xsize {
            if map.contains(&(x as isize, y as isize)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("")
    }
    println!("")
}

fn solve_part_2(input: &str) {
    let mut removed_cnt = 0;
    let (mut map, xsize, ysize) = parse_input(input);

    display_map(&map, xsize, ysize);

    loop {
        let mut to_remove: Vec<(isize, isize)> = Vec::new();

        for (x, y) in &map {
            let mut neighbours = 0;
            for (dx, dy) in [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ] {
                if map.contains(&(x + dx, y + dy)) {
                    neighbours += 1;
                }
            }
            if neighbours < 4 {
                to_remove.push((*x, *y));
            }
        }
        removed_cnt += to_remove.len();

        if to_remove.len() == 0 {
            // finished
            break;
        }

        for coord in &to_remove {
            map.remove(&coord);
        }
        display_map(&map, xsize, ysize);
    }

    println!("Removed rolls: {}", removed_cnt);
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
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
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

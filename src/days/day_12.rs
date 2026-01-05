use crate::utils::input::read_input;

type PresentCounts = Vec<usize>;
type Config = (usize, usize, PresentCounts);

fn parse_input(input: &str) -> Vec<Config> {
    input
        .lines()
        .filter(|l| l.contains('x'))
        .map(|l| {
            let w = l[..2].parse::<usize>().unwrap();
            let h = l[3..5].parse::<usize>().unwrap();
            let counts: PresentCounts = l[7..]
                .split(' ')
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
            (w, h, counts)
        })
        .collect()
}

fn solve_part_1(input: &str) {
    let configs = parse_input(input);
    let fitting = configs
        .iter()
        .filter_map(|(w, h, pcs)| {
            let max = (w / 3) * (h / 3);
            let count = pcs.iter().sum::<usize>();
            if max >= count {
                Some(())
            } else {
                None
            }
        })
        .count();

    println!("{}", fitting);
}

fn solve_part_2(_input: &str) {}

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
        0:
        ###
        ##.
        ##.

        1:
        ###
        ##.
        .##

        2:
        .##
        ###
        ##.

        3:
        ##.
        ###
        ##.

        4:
        ###
        #..
        ###

        5:
        ###
        .#.
        ###

        4x4: 0 0 0 0 2 0
        12x5: 1 0 1 0 2 2
        12x5: 1 0 1 0 3 2
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

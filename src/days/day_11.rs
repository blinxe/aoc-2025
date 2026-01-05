use crate::utils::input::read_input;
use itertools::Itertools;
use std::collections::HashMap;

type Device = String;

fn parse_input(input: &str) -> HashMap<Device, Vec<Device>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            (
                Device::from(parts.next().unwrap()),
                parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|d| Device::from(d))
                    .collect(),
            )
        })
        .collect()
}

fn count_paths(map: &HashMap<Device, Vec<Device>>, start: Device, end: Device) -> usize {
    let mut paths: Vec<Device> = Vec::new();
    let mut count = 0;

    print!("Counting paths: '{}' -> '{}': ", start, end);
    paths.push(start);
    while paths.len() > 0 {
        let mut new_paths: Vec<Device> = Vec::new();

        for p in paths {
            new_paths.extend(map.get(&p).unwrap().iter().map(|d| d.clone()));
        }
        count += new_paths.iter().filter(|p| **p == end).count();
        paths = new_paths
            .iter()
            .filter_map(|p| {
                if (*p == end) || (*p == "out") {
                    None
                } else {
                    Some(p.clone())
                }
            })
            .collect();
    }
    println!("{}", count);
    count
}

fn solve_part_1(input: &str) {
    let map = parse_input(input);
    println!(
        "Number of differents paths: {}",
        count_paths(&map, Device::from("you"), Device::from("out"))
    );
}

fn count_paths_v2(map: &HashMap<Device, Vec<Device>>, start: Device, end: Device) -> usize {
    let mut paths: HashMap<Device, usize> = HashMap::new();
    let mut count = 0;

    print!("Counting paths: '{}' -> '{}': ", start, end);
    paths.insert(start, 1);
    while paths.len() > 0 {
        let mut new_paths: HashMap<Device, usize> = HashMap::new();
        for (p, count) in paths {
            let new_devices = map.get(&p).unwrap().iter().map(|d| d.clone());
            for n in new_devices {
                if new_paths.keys().contains(&n) {
                    let to_update = new_paths.get_mut(&n).unwrap();
                    *to_update += count;
                } else {
                    new_paths.insert(n, count);
                }
            }
        }
        count += new_paths
            .iter()
            .filter_map(|(p, count)| if *p == end { Some(count) } else { None })
            .sum::<usize>();
        paths = new_paths
            .iter()
            .filter_map(|(p, count)| {
                if (*p == end) || (*p == Device::from("out")) {
                    None
                } else {
                    Some((p.clone(), *count))
                }
            })
            .collect();
    }
    println!("{}", count);
    count
}

fn solve_part_2(input: &str) {
    let map = parse_input(input);

    let svr_fft_dac_out = count_paths_v2(&map, Device::from("svr"), Device::from("fft"))
        * count_paths_v2(&map, Device::from("fft"), Device::from("dac"))
        * count_paths_v2(&map, Device::from("dac"), Device::from("out"));

    let svr_dac_fft_out = count_paths_v2(&map, Device::from("svr"), Device::from("dac"))
        * count_paths_v2(&map, Device::from("dac"), Device::from("fft"))
        * count_paths_v2(&map, Device::from("fft"), Device::from("out"));

    println!(
        "Number of differents paths: {}",
        if svr_fft_dac_out > 0 {
            svr_fft_dac_out
        } else {
            svr_dac_fft_out
        }
    );
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
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = indoc! {"
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
    "};

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}

use crate::utils::input::read_input;

type JBox = (i64, i64, i64);

fn parse_input(input: &str) -> Vec<JBox> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',');
            let x = coords.next().unwrap().parse().unwrap();
            let y = coords.next().unwrap().parse().unwrap();
            let z = coords.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect()
}

fn get_distance_squared(b1: &JBox, b2: &JBox) -> i64 {
    (b2.0 - b1.0).pow(2) + (b2.1 - b1.1).pow(2) + (b2.2 - b1.2).pow(2)
}

fn get_box_circuit(circuits: &Vec<Vec<usize>>, box_id: usize) -> Option<usize> {
    for ci in 0..circuits.len() {
        if circuits[ci].contains(&box_id) {
            return Some(ci);
        }
    }
    None
}

fn solve_part_1(input: &str, connections: usize) {
    let boxes = parse_input(input);
    let mut pairs: Vec<(usize, usize, i64)> = Vec::new();
    for b1 in 0..boxes.len() - 1 {
        for b2 in b1 + 1..boxes.len() {
            pairs.push((b1, b2, get_distance_squared(&boxes[b1], &boxes[b2])));
        }
    }

    pairs.sort_by(|p1, p2| p1.2.cmp(&p2.2));
    // println!("{:?}", &pairs[..15]);

    let mut circuits: Vec<Vec<usize>> = Vec::new();
    let mut connections = connections;
    let mut pair_id = 0;

    while connections > 0 {
        loop {
            let (bid1, bid2, _) = pairs[pair_id];
            pair_id += 1;
            let c1 = get_box_circuit(&circuits, bid1);
            let c2 = get_box_circuit(&circuits, bid2);
            if let Some(cid1) = c1 {
                if let Some(cid2) = c2 {
                    if cid1 == cid2 {
                        break;
                    }

                    let mut new = Vec::new();
                    new.append(&mut circuits[cid1]);
                    new.append(&mut circuits[cid2]);
                    circuits[cid1] = new;
                    circuits.remove(cid2);
                    break;
                } else {
                    circuits[cid1].push(bid2);
                    break;
                }
            } else if let Some(cid2) = c2 {
                circuits[cid2].push(bid1);
                break;
            } else {
                circuits.push(vec![bid1, bid2]);
                break;
            }
        }
        connections -= 1;
    }

    circuits.sort_by(|c1, c2| c2.len().cmp(&c1.len()));

    let product = circuits[0].len() * circuits[1].len() * circuits[2].len();

    println!("Product: {}", product);
}

fn solve_part_2(input: &str) {
    let boxes = parse_input(input);
    let mut pairs: Vec<(usize, usize, i64)> = Vec::new();
    for b1 in 0..boxes.len() - 1 {
        for b2 in b1 + 1..boxes.len() {
            pairs.push((b1, b2, get_distance_squared(&boxes[b1], &boxes[b2])));
        }
    }

    pairs.sort_by(|p1, p2| p1.2.cmp(&p2.2));
    // println!("{:?}", &pairs[..15]);

    let mut circuits: Vec<Vec<usize>> = Vec::new();
    circuits.push(vec![pairs[0].0, pairs[0].1]);
    let mut pair_id = 0;

    while !(circuits.len() == 1 && circuits.iter().map(|c| c.len()).sum::<usize>() == boxes.len()) {
        pair_id += 1;
        loop {
            let (bid1, bid2, _) = pairs[pair_id];
            let c1 = get_box_circuit(&circuits, bid1);
            let c2 = get_box_circuit(&circuits, bid2);
            if let Some(cid1) = c1 {
                if let Some(cid2) = c2 {
                    if cid1 == cid2 {
                        break;
                    }

                    let mut new = Vec::new();
                    new.append(&mut circuits[cid1]);
                    new.append(&mut circuits[cid2]);
                    circuits[cid1] = new;
                    circuits.remove(cid2);
                    break;
                } else {
                    circuits[cid1].push(bid2);
                    break;
                }
            } else if let Some(cid2) = c2 {
                circuits[cid2].push(bid1);
                break;
            } else {
                circuits.push(vec![bid1, bid2]);
                break;
            }
        }
    }

    let (bid1, bid2, _) = pairs[pair_id];

    let product = boxes[bid1].0 * boxes[bid2].0;

    println!("Product: {}", product);
}

pub fn part_1() {
    let input = read_input(module_path!());
    solve_part_1(input.as_str(), 1000);
}

pub fn part_2() {
    let input = read_input(module_path!());
    solve_part_2(input.as_str());
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1, 10);
    }

    const EXAMPLE_2: &str = EXAMPLE_1;

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}

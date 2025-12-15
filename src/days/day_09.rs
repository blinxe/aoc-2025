use crate::utils::input::read_input;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    p1: Point,
    p2: Point,
}

impl Rect {
    fn new(p1: &Point, p2: &Point) -> Self {
        Rect {
            p1: Point {
                x: p1.x.min(p2.x),
                y: p1.y.min(p2.y),
            },
            p2: Point {
                x: p1.x.max(p2.x),
                y: p1.y.max(p2.y),
            },
        }
    }

    fn area(&self) -> u64 {
        (self.p1.x.abs_diff(self.p2.x) + 1) * (self.p1.y.abs_diff(self.p2.y) + 1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    pos: u64,
    start: u64,
    end: u64,
}

impl Segment {
    fn new(p1: &Point, p2: &Point) -> Segment {
        if p1.x == p2.x {
            Segment {
                pos: p1.x,
                start: p1.y.min(p2.y),
                end: p1.y.max(p2.y),
            }
        } else {
            Segment {
                pos: p1.y,
                start: p1.x.min(p2.x),
                end: p1.x.max(p2.x),
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            Point {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn solve_part_1(input: &str) {
    let points = parse_input(input);
    let mut max_area = 0u64;
    for p1 in 0..points.len() - 1 {
        for p2 in p1..points.len() {
            let r = Rect::new(&points[p1], &points[p2]);
            let area = r.area();
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Largest rectangle {}", max_area);
}

fn solve_part_2(input: &str) {
    let mut points = parse_input(input);
    points.push(points[0]); // close the loop
    let mut vseg: Vec<_> = points
        .iter()
        .zip(points.iter().skip(1))
        .filter_map(|(p1, p2)| {
            if p1.x == p2.x {
                Some(Segment::new(p1, p2))
            } else {
                None
            }
        })
        .collect();
    vseg.sort_by_key(|s| s.pos);

    let mut hseg: Vec<_> = points
        .iter()
        .zip(points.iter().skip(1))
        .filter_map(|(p1, p2)| {
            if p1.y == p2.y {
                Some(Segment::new(p1, p2))
            } else {
                None
            }
        })
        .collect();
    hseg.sort_by_key(|s| s.pos);

    // println!("{:?}", vseg);
    // println!("{:?}", hseg);

    let mut max_area = 0u64;
    for p1 in 0..points.len() - 1 {
        'next_rect: for p2 in p1 + 1..points.len() {
            let r = Rect::new(&points[p1], &points[p2]);
            // println!("");
            // println!("{:?} {:?} ", &points[p1], &points[p2]);
            // print!("{:?} ", r);

            // top
            let y = r.p1.y;
            let mut inside = false;
            let mut from = 0i8;
            for v in vseg.iter() {
                if v.pos >= r.p2.x {
                    break;
                }

                if v.start > y || v.end < y {
                    // not aligned
                } else if v.start < y && v.end > y {
                    inside = !inside; // unambiguously intersects
                } else if from == 0 {
                    // getting on the fence, remember where from
                    if v.start == y {
                        from = 1;
                    } else {
                        from = -1;
                    }
                } else {
                    if v.start == y && from == -1 || v.end == y && from == 1 {
                        inside = !inside; // crossing for good
                    }
                    from = 0;
                }

                if !inside && from == 0 && v.pos >= r.p1.x {
                    continue 'next_rect;
                }
            }

            // bottom
            let y = r.p2.y;
            let mut inside = false;
            let mut from = 0i8;
            for v in vseg.iter() {
                if v.pos >= r.p2.x {
                    break;
                }

                if v.start > y || v.end < y {
                    // not aligned
                } else if v.start < y && v.end > y {
                    inside = !inside; // unambiguously intersects
                } else if from == 0 {
                    // getting on the fence, remember where from
                    if v.start == y {
                        from = 1;
                    } else {
                        from = -1;
                    }
                } else {
                    if v.start == y && from == -1 || v.end == y && from == 1 {
                        inside = !inside; // crossing for good
                    }
                    from = 0;
                }

                if !inside && from == 0 && v.pos >= r.p1.x {
                    continue 'next_rect;
                }
            }

            // left
            let x = r.p1.x;
            let mut inside = false;
            let mut from = 0i8;
            for h in hseg.iter() {
                if h.pos >= r.p2.y {
                    break;
                }

                if h.start > x || h.end < x {
                    // not aligned
                } else if h.start < x && h.end > x {
                    inside = !inside; // unambiguously intersects
                } else if from == 0 {
                    // getting on the fence, remember where from
                    if h.start == x {
                        from = 1;
                    } else {
                        from = -1;
                    }
                } else {
                    if h.start == x && from == -1 || h.end == x && from == 1 {
                        inside = !inside; // crossing for good
                    }
                    from = 0;
                }

                if !inside && from == 0 && h.pos >= r.p1.y {
                    continue 'next_rect;
                }
            }

            // right
            let x = r.p2.x;
            let mut inside = false;
            let mut from = 0i8;
            for h in hseg.iter() {
                if h.pos >= r.p2.y {
                    break;
                }

                if h.start > x || h.end < x {
                    // not aligned
                } else if h.start < x && h.end > x {
                    inside = !inside; // unambiguously intersects
                } else if from == 0 {
                    // getting on the fence, remember where from
                    if h.start == x {
                        from = 1;
                    } else {
                        from = -1;
                    }
                } else {
                    if h.start == x && from == -1 || h.end == x && from == 1 {
                        inside = !inside; // crossing for good
                    }
                    from = 0;
                }

                if !inside && from == 0 && h.pos >= r.p1.y {
                    continue 'next_rect;
                }
            }

            // print!(" is inside");

            let area = r.area();
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Largest rectangle {}", max_area);
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
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
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

use std::time::Instant;
use std::collections::BinaryHeap;

use rustc_hash::FxHashSet;


const NEIGHBOR_DIRS: [(i64, i64); 4] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1)
];


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    dist: i64,
    priority: i64,
}

impl State {
    fn new(pos: Pos, dist: i64, priority: i64) -> Self {
        Self { pos, dist, priority }
    }
}


impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.priority).cmp(&other.priority).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}



fn main() {
    let input = include_str!("input");

    let start = Instant::now();
    let _parsed_input = parse_input(input);
    let parse_duration = start.elapsed().as_secs_f32();
    println!("Parsing took {parse_duration} secs");

    println!();

    let start = Instant::now();
    let p1_answer = p1(input);
    let p1_duration = start.elapsed().as_secs_f32();
    println!("P1: {p1_answer}");
    println!("Took {p1_duration} secs");

    println!();

    let start = Instant::now();
    let p2_answer = p2(input);
    let p2_duration = start.elapsed().as_secs_f32();
    println!("P2: {p2_answer}");
    println!("Took {p2_duration} secs");
}


fn find_shortest_path(map_size: i64, bytes: &[Pos]) -> Option<i64> {
    let blocked = FxHashSet::from_iter(bytes.iter().cloned());
    let goal = Pos::new(map_size-1, map_size-1);

    let mut visited = FxHashSet::default();
    let mut frontier = BinaryHeap::new();
    frontier.push(State::new(Pos::new(0, 0), 0, 0));

    while let Some(state) = frontier.pop() {
        if state.pos == goal {
            return Some(state.dist)
        }

        visited.insert(state.pos);

        let new_dist = state.dist + 1;
        for neighbor in get_neighbors(map_size, &blocked, state.pos) {
            if !visited.contains(&neighbor) {
                let priority = heuristic(neighbor, goal) + new_dist;
                frontier.push(State::new(neighbor, new_dist, priority));
            }
        }
    }

    None
}


fn get_neighbors(map_size: i64, blocked: &FxHashSet<Pos>, cur_pos: Pos) -> Vec<Pos> {
    let mut neighbors = vec![];

    for (dx, dy) in NEIGHBOR_DIRS {
        let new_x = cur_pos.x + dx;
        let new_y = cur_pos.y + dy;
        if new_x >= 0 && new_x < map_size && new_y >= 0 && new_y < map_size {
            let new_pos = Pos::new(new_x, new_y);
            if !blocked.contains(&new_pos) {
                neighbors.push(new_pos);
            }
        }
    }

    neighbors
}


fn heuristic(cur_pos: Pos, goal: Pos) -> i64 {
    (cur_pos.x - goal.x).abs() + (cur_pos.y - goal.y).abs()
}


fn p1(input: &str) -> String {
    let bytes = parse_input(input);

    for y in 0..71 {
        for x in 0..71 {
            if bytes[0..1024].contains(&Pos::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
    println!();


    let dist = find_shortest_path(71, &bytes[0..1024]).unwrap();
    format!("{dist}")
}


fn p2(input: &str) -> String {
    let bytes = parse_input(input);

    let mut byte: Option<_> = None;

    for idx in 1024..bytes.len() {
        if find_shortest_path(71, &bytes[0..=idx]).is_none() { byte = Some(bytes[idx]); break }
    }

    format!("{:?}", byte.unwrap())
}


fn parse_input(input: &str) -> Vec<Pos> {
    input.lines()
        .map(|line| {
            let (x_str, y_str) = line.split_once(",").unwrap();
            Pos {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap()
            }
        })
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        let bytes = parse_input(EX);

        for y in 0..7 {
            for x in 0..7 {
                if bytes[0..12].contains(&Pos::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();

        let dist = find_shortest_path(7, &bytes[0..12]).unwrap();

        assert_eq!(dist, 22);
    }

    #[test]
    fn test2() {
        let bytes = parse_input(EX);

        let mut byte: Option<_> = None;

        for idx in 12..bytes.len() {
            if find_shortest_path(7, &bytes[0..=idx]).is_none() { byte = Some(bytes[idx]); break }
        }

        assert_eq!(byte.unwrap(), Pos::new(6, 1))
    }

}
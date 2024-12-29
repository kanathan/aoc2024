use std::time::Instant;
use std::collections::{HashMap, HashSet, VecDeque};

use ndarray::Array2;

#[derive(Debug)]
enum MapObj {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

const NEIGHBOR_DIRS: [(i64, i64); 4] = [
        ((-1, 0)),
        ((1, 0)),
        ((0, -1)),
        ((0, 1))
    ];


fn get_cheats(map: &Array2<MapObj>, start: Pos, stop: Pos, cheat_length: i32) -> HashMap<(Pos, Pos), i32> {
    let mut frontier = VecDeque::new();
    let mut nominal_cost_from = HashMap::new();

    frontier.push_back(stop);
    nominal_cost_from.insert(stop, 0);

    while let Some(cur_pos) = frontier.pop_front() {
        let new_cost = *nominal_cost_from.get(&cur_pos).unwrap() + 1;
        for (neighbor, cheated) in get_neighbors(map, cur_pos) {
            if cheated { continue };
            if !nominal_cost_from.contains_key(&neighbor) {
                nominal_cost_from.insert(neighbor, new_cost);
                frontier.push_back(neighbor);
            }
        }
    }

    println!("No cheat start to stop = {}", nominal_cost_from.get(&start).unwrap());
    let mut cheats = HashMap::new();

    for (&cheat_start, &start_cost) in nominal_cost_from.iter() {
        let mut cheat_frontier = VecDeque::new();
        let mut visited = HashSet::new();
        cheat_frontier.push_back((cheat_start, cheat_length));
        visited.insert(cheat_start);

        while let Some((cur_pos, remaining_cheat_time)) = cheat_frontier.pop_front() {
            if remaining_cheat_time == 0 { continue };

            let new_remaining_cheat_time = remaining_cheat_time - 1;
            for (neighbor, cheated) in get_neighbors(map, cur_pos) {
                if new_remaining_cheat_time == 0 && cheated { continue }
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    cheat_frontier.push_back((neighbor, new_remaining_cheat_time));

                    if let Some(&end_cost) = nominal_cost_from.get(&neighbor) {
                        let cheat_time = cheat_length - new_remaining_cheat_time;
                        let cost_change = start_cost - (end_cost + cheat_time);
                        if cost_change > 0 {
                            if let Some(&existing_change) = cheats.get(&(cheat_start, neighbor)) {
                                if existing_change > cost_change { continue }
                            }
                            cheats.insert((cheat_start, neighbor), cost_change);
                        }
                    }
                }
            }
        }
    }

    cheats
}


fn get_neighbors(map: &Array2<MapObj>, pos: Pos) -> Vec<(Pos, bool)> {
    let mut neighbors = vec![];
    for (drow, dcol) in NEIGHBOR_DIRS {
        let Ok(row) = (pos.row as i64 + drow).try_into() else { continue };
        let Ok(col) = (pos.col as i64 + dcol).try_into() else { continue };
        let Some(obj) = map.get((row, col)) else { continue };
        neighbors.push((Pos {row, col}, matches!(obj, MapObj::Wall)))
    }
    neighbors
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


fn p1(input: &str) -> String {
    let (map, start, stop) = parse_input(input);
    let cheats = get_cheats(&map, start, stop, 2);

    let count = cheats.into_iter()
        .filter(|(_, cost)| *cost >= 100)
        .count();

    format!("{count}")
}


fn p2(input: &str) -> String {
    let (map, start, stop) = parse_input(input);
    let cheats = get_cheats(&map, start, stop, 20);

    let count = cheats.into_iter()
        .filter(|(_, cost)| *cost >= 100)
        .count();

    format!("{count}")
}


fn parse_input(input: &str) -> (Array2<MapObj>, Pos, Pos) {
    let mut start_idx = None;
    let mut end_idx = None;
    let data = input.lines()
        .flat_map(|line| line.chars())
        .enumerate()
        .map(|(idx, c)| match c {
            '.' => MapObj::Empty,
            '#' => MapObj::Wall,
            'S' => {
                if start_idx.is_some() { panic!("Multiple starts found") };
                start_idx = Some(idx);
                MapObj::Empty
            },
            'E' => {
                if end_idx.is_some() { panic!("Multiple ends found") };
                end_idx = Some(idx);
                MapObj::Empty
            },
            _ => unreachable!("Unexpected char: {c}")
        })
        .collect::<Vec<_>>();
    let ncols = input.lines().next().unwrap().len();
    let nrows = data.len()/ncols;

    let map = Array2::from_shape_vec((nrows, ncols), data).unwrap();
    let start = Pos {
        row: start_idx.unwrap()/ncols,
        col: start_idx.unwrap() % ncols,
    };
    let end = Pos {
        row: end_idx.unwrap()/ncols,
        col: end_idx.unwrap() % ncols,
    };
    (map, start, end)
}


#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        println!("2 sec cheat");
        let (map, start, stop) = parse_input(EX);
        let cheats = get_cheats(&map, start, stop, 2);

        let mut cheat_sums = HashMap::new();
        for (_, &cost) in cheats.iter() {
            cheat_sums.entry(cost)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        for (cost, count) in cheat_sums.into_iter().sorted_by(|a,b| a.0.cmp(&b.0)) {
            println!("{count} cheats saved {cost}")
        }

        println!();
        println!("20 sec cheat");

        let cheats = get_cheats(&map, start, stop, 20);

        let mut cheat_sums = HashMap::new();
        for (_, &cost) in cheats.iter() {
            cheat_sums.entry(cost)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        for (cost, count) in cheat_sums.into_iter().sorted_by(|a,b| a.0.cmp(&b.0)) {
            println!("{count} cheats saved {cost}")
        }
    }

}
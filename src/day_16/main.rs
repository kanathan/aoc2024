use std::time::Instant;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum MapObj {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
    dir: Dir,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.row*self.col).cmp(&(other.row*other.col))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Pos,
    route: Vec<Pos>
}

impl State {
    fn new(cost: usize, position: Pos, route: Vec<Pos>) -> Self {
        Self { cost, position, route }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const NEIGHBOR_DIRS: [((i32, i32), Dir); 4] = [
        ((-1, 0), Dir::North),
        ((1, 0), Dir::South),
        ((0, -1), Dir::West),
        ((0, 1), Dir::East)
    ];

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
    let (map, start, end) = parse_input(input);
    let (cost, _) = shortest_path_cost(&map, start, end);
    format!("{cost}")
}


fn p2(input: &str) -> String {
    let (map, start, end) = parse_input(input);
    let (_, best_paths) = shortest_path_cost(&map, start, end);
    format!("{}", best_paths.len())
}


fn shortest_path_cost(map: &Array2<MapObj>, start: Pos, end: Pos) -> (usize, HashSet<(usize, usize)>) {
    let mut frontier = BinaryHeap::new();
    let mut cost_so_far = HashMap::new();
    let mut shortest_cost = usize::MAX;
    let mut best_paths = HashSet::new();
    frontier.push(State::new(0, start, vec![start]));
    cost_so_far.insert(start, 0);


    while let Some(cur_state) = frontier.pop() {
        if cur_state.position.col == end.col && cur_state.position.row == end.row {
            if cur_state.cost <= shortest_cost {
                shortest_cost = cur_state.cost;
                best_paths.extend(cur_state.route.into_iter().map(|p| (p.row, p.col)));
            }
            continue
        }

        // Move
        if let Some(new_pos) = get_move(map, cur_state.position) {
            let new_cost = cur_state.cost + 1;
            if !cost_so_far.contains_key(&new_pos) || *cost_so_far.get(&new_pos).unwrap() >= new_cost {
                cost_so_far.insert(new_pos, new_cost);
                let mut route = cur_state.route.clone();
                route.push(new_pos);
                frontier.push(State::new(new_cost, new_pos, route));
            }
        }

        // Rotate
        let (dir_1, dir_2) = match cur_state.position.dir {
            Dir::North | Dir::South => (Dir::West, Dir::East),
            Dir::East | Dir::West => (Dir::North, Dir::South),
        };

        let new_positions = [
            Pos { row: cur_state.position.row, col: cur_state.position.col, dir: dir_1 },
            Pos { row: cur_state.position.row, col: cur_state.position.col, dir: dir_2 },
        ];

        for new_pos in new_positions {
            let new_cost = cur_state.cost + 1000;
            if !cost_so_far.contains_key(&new_pos) || *cost_so_far.get(&new_pos).unwrap() >= new_cost {
                cost_so_far.insert(new_pos, new_cost);
                let mut route = cur_state.route.clone();
                route.push(new_pos);
                frontier.push(State::new(new_cost, new_pos, route));
            }
        }
    }

    
    // let mut to_visit = VecDeque::new();
    // best_paths.insert(end);
    // to_visit.push_back(end);
    // while let Some(pos) = to_visit.pop_front() {
    //     // Dir doesn't matter
    //     let neighbors = get_neighbors(map, pos, Dir::North);
    //     let min_cost = *neighbors.iter().map(|(pos, _, _)| cost_so_far.get(pos).unwrap()).min().unwrap();
    //     for (pos, _, _) in neighbors {
    //         if *cost_so_far.get(&pos).unwrap() == min_cost {
    //             best_paths.insert(pos);
    //             to_visit.push_back(pos);
    //         }
    //     }
    //     if min_cost == 0 {
    //         break
    //     }
    // }

    (shortest_cost, best_paths)
}


fn get_move(map: &Array2<MapObj>, pos: Pos) -> Option<Pos> {
    let (drow, dcol) = NEIGHBOR_DIRS.iter().filter(|(_, dir)| pos.dir == *dir).next().unwrap().0;
    let new_row = (pos.row as i32 + drow).try_into().ok()?;
    let new_col = (pos.col as i32 + dcol).try_into().ok()?;
    let map_obj = map.get((new_row, new_col))?;

    if matches!(map_obj, MapObj::Empty) {
        Some(Pos { row: new_row, col: new_col, dir: pos.dir })
    } else {
        None
    }
}


// fn get_neighbors(map: &Array2<MapObj>, pos: Pos, dir: Dir) -> Vec<(Pos, usize, Dir)> {
    

//     let mut neighbors = vec![];

//     for ((drow, dcol), neighbor_dir) in NEIGHBOR_DIRS {
//         let Ok(new_row) = (pos.row as i32 + drow).try_into() else { continue };
//         let Ok(new_col) = (pos.col as i32 + dcol).try_into() else { continue };
//         let Some(map_obj) = map.get((new_row, new_col)) else { continue };
//         if matches!(map_obj, MapObj::Wall) { continue };
//         let cost = if dir == neighbor_dir { 1 } else { 1001 };
//         let new_pos = Pos { row: new_row, col: new_col };
//         neighbors.push((new_pos, cost, neighbor_dir));
//     }

//     neighbors
// }


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
        dir: Dir::East,
    };
    let end = Pos {
        row: end_idx.unwrap()/ncols,
        col: end_idx.unwrap() % ncols,
        dir: Dir::North,
    };
    (map, start, end)
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");
    const EX2: &str = include_str!("example2");

    #[test]
    fn test1() {
        let (map, start, end) = parse_input(EX);
        let (cost, best_paths) = shortest_path_cost(&map, start, end);

        // for (irow, row) in map.outer_iter().enumerate() {
        //     for (icol, obj) in row.indexed_iter() {
        //         if best_paths.contains(&(irow, icol)) {
        //             print!("O");
        //         } else if matches!(obj, MapObj::Empty) {
        //             print!(".");
        //         } else {
        //             print!("#");
        //         }
        //     }
        //     println!();
        // }
        // println!();

        assert_eq!(cost, 7036);
        assert_eq!(best_paths.len(), 45);
    }

    #[test]
    fn test2() {
        let (map, start, end) = parse_input(EX2);
        let (cost, best_paths) = shortest_path_cost(&map, start, end);

        assert_eq!(cost, 11048);
        assert_eq!(best_paths.len(), 64);
    }

}
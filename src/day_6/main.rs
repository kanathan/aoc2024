use std::time::Instant;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Obstacle,
    Free,
}

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<Vec<Cell>>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    pos: (isize, isize),
    dir: Direction,
}

impl Grid {
    fn new(data: Vec<Vec<Cell>>) -> Self {
        Self {
            data
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<Cell> {
        if x < 0 || y < 0 {
            return None
        }
        let x = x as usize;
        let y = y as usize;
        let Some(line) = self.data.get(y) else {
            return None
        };
        let Some(&val) = line.get(x) else {
            return None
        };
        return Some(val)
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


fn p1(input: &str) -> String {
    let (grid, guard) = parse_input(input);
    let visited = walk_map(&grid, guard.clone()).unwrap();

    format!("{}", visited.len())
}


fn p2(input: &str) -> String {
    let (grid, guard) = parse_input(input);
    let starting_pos = guard.pos;
    let visited = walk_map(&grid, guard.clone()).unwrap();

    let mut blockers = 0;
    for pos in visited.iter() {
        if starting_pos == *pos { continue }
        let mut mod_grid = grid.clone();
        mod_grid.data[pos.1 as usize][pos.0 as usize] = Cell::Obstacle;
        if walk_map(&mod_grid, guard.clone()).is_none() {
            blockers += 1;
        }
    }

    format!("{}", blockers)
}


fn walk_map(grid: &Grid, mut guard: Guard) -> Option<HashSet<(isize, isize)>> {
    let mut visited_pos = HashSet::new();

    loop {
        if !visited_pos.insert((guard.pos, guard.dir)) {
            return None
        }

        match guard.dir {
            Direction::North => {
                let new_pos = (guard.pos.0, guard.pos.1-1);

                if let Some(Cell::Obstacle) = grid.get(new_pos.0, new_pos.1) {
                    guard.dir = Direction::East;
                } else {
                    guard.pos = new_pos;
                }
            },
            Direction::East => {
                let new_pos = (guard.pos.0+1, guard.pos.1);

                if let Some(Cell::Obstacle) = grid.get(new_pos.0, new_pos.1) {
                    guard.dir = Direction::South;
                } else {
                    guard.pos = new_pos;
                }
            },
            Direction::South => {
                let new_pos = (guard.pos.0, guard.pos.1+1);

                if let Some(Cell::Obstacle) = grid.get(new_pos.0, new_pos.1) {
                    guard.dir = Direction::West;
                } else {
                    guard.pos = new_pos;
                }
            },
            Direction::West => {
                let new_pos = (guard.pos.0-1, guard.pos.1);

                if let Some(Cell::Obstacle) = grid.get(new_pos.0, new_pos.1) {
                    guard.dir = Direction::North;
                } else {
                    guard.pos = new_pos;
                }
            }
        }

        if grid.get(guard.pos.0, guard.pos.1).is_none() {
            return Some(visited_pos.into_iter()
                .map(|(pos, _)| pos)
                .collect()
            )
        }
    }
}


fn parse_input(input: &str) -> (Grid, Guard) {
    let mut data = Vec::new();
    let mut guard = Guard {
        pos: (0,0),
        dir: Direction::North,
    };

    for (y,line) in input.lines().enumerate() {
        let row = line.chars().enumerate()
            .map(|(x,c)| {
                match c {
                    '.' => Cell::Free,
                    '#' => Cell::Obstacle,
                    _ => {
                        guard.pos = (x as isize,y as isize);
                        Cell::Free
                    },
                }
            })
            .collect::<Vec<Cell>>();
        data.push(row);
    }

    (Grid::new(data), guard)
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        let (grid, guard) = parse_input(EX);
        let visited = walk_map(&grid, guard.clone()).unwrap();
        assert_eq!(visited.len(), 41)
    }

    #[test]
    fn test2() {
        let answer = p2(EX);
        assert_eq!(answer, "6")
    }

}
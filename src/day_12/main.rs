use std::time::Instant;
use std::collections::VecDeque;
use rustc_hash::FxHashSet;

use ndarray::Array2;


#[derive(Clone)]
struct Region {
    cells: FxHashSet<(usize, usize)>,
    walls: u32,
    plant: char
}

impl std::fmt::Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}=A{:?},P{}]", self.plant, self.cells.len(), self.walls)
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
    let grid = parse_input(input);
    let regions = parse_regions(&grid);
    let price = price_regions(&regions);
    format!("{price}")
}


fn p2(input: &str) -> String {
    let grid = parse_input(input);
    let regions = parse_regions(&grid);
    let price = bulk_price_regions(&regions);
    format!("{price}")
}


fn price_regions(regions: &[Region]) -> u32 {
    regions.into_iter()
        .map(|region| {
            let area = region.cells.len() as u32;
            area * region.walls
        })
        .sum()
}


fn bulk_price_regions(regions: &[Region]) -> u32 {
    regions.into_iter()
        .map(|region| {
            let area = region.cells.len() as u32;
            let sides = get_sides(region);
            area * sides
        })
        .sum()
}


fn parse_regions(grid: &Array2<char>) -> Vec<Region> {
    let mut visited = FxHashSet::default();
    let mut regions = Vec::new();

    for (pos, _) in grid.indexed_iter() {
        if visited.contains(&pos) { continue }
        let region = bfs(pos, grid);
        regions.push(region.clone());
        visited.extend(region.cells);
    }

    regions
}


fn get_neighbor_info(pos: (usize, usize), grid: &Array2<char>) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let plant = *grid.get(pos).unwrap();
    let mut neighbors = Vec::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dx, dy) in directions.iter() {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x >= 0 && new_y >= 0 && new_x < grid.shape()[0] as isize && new_y < grid.shape()[1] as isize {
            let new_pos = (new_x as usize, new_y as usize);
            if *grid.get(new_pos).unwrap() == plant {
                neighbors.push(new_pos);
            }
        }
    }
    neighbors
}


fn get_sides(region: &Region) -> u32 {
    let mut corner_count = 0;

    for (x, y) in region.cells.iter() {
        let corner_dirs = [
            ((-1, 0), (0, -1), (-1, -1)),
            ((-1, 0), (0, 1), (-1, 1)),
            ((1, 0), (0, -1), (1, -1)),
            ((1, 0), (0, 1), (1, 1))
        ];

        for ((dx1, dy1), (dx2, dy2), (dx3, dy3)) in corner_dirs {
            let (x1, y1) = (*x as isize + dx1, *y as isize + dy1);
            let (x2, y2) = (*x as isize + dx2, *y as isize + dy2);
            let (x3, y3) = (*x as isize + dx3, *y as isize + dy3);

            let side_1_matches = 
                if x1 >= 0 && y1 >= 0 {
                    let neighbor = (x1 as usize, y1 as usize);
                    region.cells.contains(&neighbor)
                } else {
                    false
                };
            
            let side_2_matches = 
                if x2 >= 0 && y2 >= 0 {
                    let neighbor = (x2 as usize, y2 as usize);
                    region.cells.contains(&neighbor)
                } else {
                    false
                };
            
            let diag_matches = 
                if x3 >= 0 && y3 >= 0 {
                    let neighbor = (x3 as usize, y3 as usize);
                    region.cells.contains(&neighbor)
                } else {
                    false
                };
                
            // Outside and inside corners
            if (!side_1_matches && !side_2_matches) || (side_1_matches && side_2_matches && !diag_matches) {
                corner_count += 1;
            }
        }
    }
    corner_count
}


fn bfs(pos: (usize, usize), grid: &Array2<char>) -> Region {
    let mut visited = FxHashSet::default();
    let mut to_visit = VecDeque::new();
    let mut walls = 0;
    to_visit.push_back(pos);

    while let Some(pos) = to_visit.pop_front() {
        if visited.contains(&pos) { continue }
        visited.insert(pos);
        let neighbors = get_neighbor_info(pos, grid);
        walls += 4 - neighbors.len() as u32;
        for neighbor in neighbors {
            to_visit.push_back(neighbor);
        }
    }

    Region {
        cells: visited,
        walls,
        plant: *grid.get(pos).unwrap()
    }
}


fn parse_input(input: &str) -> Array2<char> {
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().len();

    let data = input.lines()
        .map(|line| line.chars())
        .flatten()
        .collect::<Vec<char>>();

    Array2::from_shape_vec((nrows,ncols), data).unwrap()
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");
    const EX2: &str = include_str!("example2");

    #[test]
    fn test1() {
        let grid = parse_input(EX);
        println!("{:?}", grid);
        let region_grid = parse_regions(&grid);
        println!("{:?}", region_grid);
        assert_eq!(price_regions(&region_grid), 1930);
    }

    #[test]
    fn test2() {
        let grid = parse_input(EX);
        println!("{:?}", grid);
        let region_grid = parse_regions(&grid);
        println!("{:?}", region_grid);
        assert_eq!(bulk_price_regions(&region_grid), 1206);
    }

    #[test]
    fn test3() {
        let grid = parse_input(EX2);
        let region_grid = parse_regions(&grid);
        for region in region_grid {
            println!("{region:?}: {}", get_sides(&region));
        }
    }

}
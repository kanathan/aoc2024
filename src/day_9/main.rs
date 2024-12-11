use std::time::Instant;

use rangemap::RangeMap;

#[derive(Debug)]
struct DiskEntity {
    pos: usize,
    size: usize,
}

struct Disk {
    files: Vec<DiskEntity>,
    disk_map: Vec<Option<usize>>,
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
    let disk = build_disk(&parse_input(input));
    let disk_map = compress(disk.disk_map);
    let checksum = get_checksum(&disk_map);

    format!("{checksum}")
}


fn p2(input: &str) -> String {
    let disk = build_disk(&parse_input(input));
    let disk_map = defrag(disk);
    let checksum = get_checksum(&disk_map);

    format!("{checksum}")
}


fn build_disk(input: &[u8]) -> Disk {
    let mut files = vec![];
    let mut free_space = vec![];

    let mut input_iter = input.into_iter();
    let mut disk_idx = 0;

    loop {
        let block_size = *input_iter.next().unwrap();
        files.push(DiskEntity {
            pos: disk_idx,
            size: block_size as usize,
        });
        disk_idx += block_size as usize;

        let Some(&block_size) = input_iter.next() else {
            break
        };
        free_space.push(DiskEntity {
            pos: disk_idx,
            size: block_size as usize,
        });
        disk_idx += block_size as usize;
    }

    let disk_size = usize::max(
        files.iter().map(|f| f.pos + f.size).max().unwrap(),
        free_space.iter().map(|s| s.pos+s.size).max().unwrap()
    );

    let mut disk_map: Vec<Option<usize>> = vec![None; disk_size];

    for (id, file) in files.iter().enumerate() {
        for idx in file.pos..(file.pos+file.size) {
            disk_map[idx] = Some(id)
        }
    }

    Disk {
        files,
        disk_map
    }
}


fn compress(mut disk_map: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut head = disk_map.iter().enumerate().find_map(|(idx, item)| {
        if item.is_none() { Some(idx) } else { None }
    }).unwrap();

    let mut tail = disk_map.iter().enumerate().rev().find_map(|(idx, item)| {
        if item.is_some() { Some(idx) } else { None }
    }).unwrap();

    while head < tail {
        disk_map.swap(head, tail);
        while let Some(None) = disk_map.get(tail) {
            tail -= 1;
        }
        while let Some(Some(_)) = disk_map.get(head) {
            head += 1;
        }
    }

    disk_map
}


fn defrag(disk: Disk) -> Vec<Option<usize>> {
    let files = disk.files;

    let mut range_map = RangeMap::new();

    for (id, file) in files.iter().enumerate() {
        range_map.insert(file.pos..(file.pos+file.size), id);
    }

    for (id, file) in files.iter().enumerate().rev() {
        let gaps = range_map.gaps(&(0..file.pos)).collect::<Vec<_>>();
        for gap in gaps {
            if gap.len() >= file.size {
                range_map.insert(gap.start..(gap.start+file.size), id);
                range_map.remove(file.pos..(file.pos+file.size));
                break
            }
        }
    }

    let disk_map = (0..range_map.last_range_value().unwrap().0.end)
        .map(|idx| {
            if let Some(id) = range_map.get(&idx) {
                Some(*id)
            } else {
                None
            }
        })
        .collect();  

    disk_map
}


fn get_checksum(disk_map: &[Option<usize>]) -> usize {
    disk_map.iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            if let Some(id) = *item {
                Some(id * idx)
            } else {
                None
            }
        })
        .sum::<usize>()
}


fn parse_input(input: &str) -> Vec<u8> {
    input.trim()
        .chars()
        .map(|c| {
            if (c as u8) >= ('0' as u8) {
                c as u8 - ('0' as u8)
            } else {
                unimplemented!("Unexpected char {c}")
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
        let disk = build_disk(&parse_input(EX));
        //let disk_map = build_disc_map(&parse_input(EX));
        let disk_map = disk.disk_map;
        for item in disk_map.iter() {
            match item {
                Some(id) => print!("{id}"),
                None => print!("."),
            }
        }
        println!();

        let disk_map = compress(disk_map);
        for item in disk_map.iter() {
            match item {
                Some(id) => print!("{id}"),
                None => print!("."),
            }
        }
        println!();

        assert_eq!(get_checksum(&disk_map), 1928)
    }

    #[test]
    fn test2() {
        let disk = build_disk(&parse_input(EX));
        let disk_map = defrag(disk);
        for item in disk_map.iter() {
            match item {
                Some(id) => print!("{id}"),
                None => print!("."),
            }
        }
        println!();

        assert_eq!(get_checksum(&disk_map), 2858)
    }

}
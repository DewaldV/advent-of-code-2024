#[derive(Debug, Clone)]
struct Block {
    id: usize,
    free: bool,
}

impl Block {
    fn new(id: usize, free: bool) -> Block {
        Block { id, free }
    }
}

impl ToString for Block {
    fn to_string(&self) -> String {
        if self.free {
            return ".".to_string();
        }
        format!("{}", self.id)
    }
}

struct File {
    id: usize,
    start: usize,
    end: usize,
}

pub fn solve(content: &str) -> i128 {
    let mut disk_map: Vec<Block> = Vec::new();
    let mut files: Vec<usize> = Vec::new();

    for l in content.lines() {
        for (i, s) in l.chars().enumerate() {
            let size: u8 = s.to_string().parse().unwrap();

            let free = i % 2 != 0;
            let id: usize = (i / 2).try_into().unwrap();
            if !free {
                files.push(id);
            }

            for _ in 0..size {
                disk_map.push(Block::new(id, free));
            }
        }
    }

    loop {
        let first_free_block = disk_map.iter().position(|b| b.free);
        let last_used_block = disk_map.iter().rposition(|b| !b.free);

        let free_space_available = first_free_block.is_some_and(|b| b < last_used_block.unwrap());

        if !free_space_available {
            break;
        }

        let block = disk_map.remove(last_used_block.unwrap());
        disk_map[first_free_block.unwrap()] = block;
        disk_map.push(Block::new(0, true));
    }

    let checksum: usize = disk_map
        .iter()
        .filter(|b| !b.free)
        .enumerate()
        .map(|(i, b)| b.id * i)
        .sum();

    return checksum.try_into().unwrap();
}

fn print_map(map: Vec<Block>) {
    println!(
        "{}",
        map.iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}

#[cfg(test)]
mod tests {
    use crate::util::run_example_file;

    use super::*;

    #[test]
    fn test_example_file() {
        run_example_file(9, 2858, &solve);
    }
}

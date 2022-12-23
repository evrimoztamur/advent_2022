use std::collections::{HashMap, HashSet};

const ROUNDS: usize = 10;

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut elves: HashSet<(isize, isize)> = HashSet::new();

        for (y, line) in content.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    elves.insert((x as isize, y as isize));
                }
            }
        }

        let neighbours = vec![
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ];

        for i in 0..5000 {
            let mut intents: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();

            for (x, y) in elves.iter() {
                if neighbours
                    .iter()
                    .all(|v| !elves.contains(&(x + v.0, y + v.1)))
                {
                    continue;
                }

                for d in 0..4 {
                    if adjacencies(i + d)
                        .iter()
                        .all(|v| !elves.contains(&(x + v.0, y + v.1)))
                    {
                        intents
                            .entry(target(*x, *y, i + d))
                            .and_modify(|v| v.push((*x, *y)))
                            .or_insert(vec![(*x, *y)]);
                        break;
                    }
                }
            }

            let mut moves = 0;

            for ((x, y), mut loc_intents) in intents {
                if loc_intents.len() == 1 {
                    let elf = loc_intents.pop().unwrap();

                    elves.remove(&elf);
                    elves.insert((x, y));

                    moves += 1;
                }
            }

            if moves == 0 {
                panic!("P2 {}", i + 1);
            }
        }

        let min_loc = elves.iter().fold((isize::MAX, isize::MAX), |acc, loc| {
            (acc.0.min(loc.0), acc.1.min(loc.1))
        });
        let max_loc = elves.iter().fold((isize::MIN, isize::MIN), |acc, loc| {
            (acc.0.max(loc.0), acc.1.max(loc.1))
        });

        for y in min_loc.1..=max_loc.1 {
            for x in min_loc.0..=max_loc.0 {
                if elves.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }

        println!(
            "P1 {}",
            (max_loc.0 - min_loc.0 + 1) * (max_loc.1 - min_loc.1 + 1) - elves.len() as isize
        );
    }
}

fn adjacencies(direction: usize) -> Vec<(isize, isize)> {
    match direction % 4 {
        0 => vec![(-1, -1), (0, -1), (1, -1)], // North
        1 => vec![(-1, 1), (0, 1), (1, 1)],    // South
        2 => vec![(-1, 1), (-1, 0), (-1, -1)], // West
        _ => vec![(1, 1), (1, 0), (1, -1)],    // East
    }
}

fn target(x: isize, y: isize, direction: usize) -> (isize, isize) {
    match direction % 4 {
        0 => (x, y - 1), // North
        1 => (x, y + 1), // South
        2 => (x - 1, y), // West
        _ => (x + 1, y), // East
    }
}

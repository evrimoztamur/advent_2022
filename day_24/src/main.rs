fn main() {
    if let Ok(content) = std::fs::read_to_string("example.txt") {
        let mut blizzards: Vec<(usize, usize, u8)> = Vec::new();

        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in content.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' && ch != '#' {
                    blizzards.push((x, y, into_direction(ch)));
                }
            }

            max_x = line.len() - 1;
            max_y = max_y.max(y);
        }

        println!(
            "P1 {:?}",
            shortest_path(&blizzards, max_x, max_y, (1, 0), (max_x - 1, max_y))
        );
    }
}

fn print_blizzmap(blizzards: &Vec<(usize, usize, u8)>, max_x: usize, max_y: usize) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            if blizzards.iter().find(|vg| v.0 == x && v.1 == y).is_some() {
                print!("%");
            } else {
                print!(".");
            }
        }

        println!();
    }
    println!();
}

fn print_blizzmap_bool(blizzmap: &Vec<bool>, max_x: usize, max_y: usize) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            if blizzmap[x + y * (max_x + 1)] {
                print!("%");
            } else {
                print!(".");
            }
        }

        println!();
    }
    println!();
}

fn record_blizzmap(blizzards: &Vec<(usize, usize, u8)>, max_x: usize, max_y: usize) -> Vec<bool> {
    let mut res = Vec::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            res.push(
                (blizzards.iter().find(|v| v.0 == x && v.1 == y).is_some()
                    || x == 0
                    || x == max_x
                    || y == 0
                    || y == max_y)
                    && !(x == max_x - 1 && y == max_y),
            );
        }
    }

    res
}

fn into_direction(ch: char) -> u8 {
    match ch {
        '^' => 0,
        '>' => 1,
        'v' => 2,
        '<' => 3,
        _ => panic!(),
    }
}

fn move_blizzard(blizzard: &mut (usize, usize, u8), max_x: usize, max_y: usize) {
    let movement = match blizzard.2 {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => panic!(),
    };

    let new_pos = (
        blizzard.0 as isize + movement.0,
        blizzard.1 as isize + movement.1,
    );

    let mut new_pos = (new_pos.0 as usize, new_pos.1 as usize);

    if new_pos.0 == 0 {
        new_pos.0 = max_x - 1;
    } else if new_pos.0 == max_x {
        new_pos.0 = 1;
    }

    if new_pos.1 == 0 {
        new_pos.1 = max_y - 1;
    } else if new_pos.1 == max_y {
        new_pos.1 = 1;
    }

    blizzard.0 = new_pos.0;
    blizzard.1 = new_pos.1;
}

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State(usize, usize, usize);

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(
    blizzards: &Vec<(usize, usize, u8)>,
    max_x: usize,
    max_y: usize,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<usize> {
    let mut blizzmaps: Vec<Vec<bool>> = Vec::new();
    let mut blizzards = blizzards.clone();

    let mut dist: HashSet<(usize, usize, usize)> = HashSet::new();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    dist.insert((start.0, start.1, 0));

    heap.push(State(start.0, start.1, 0));

    while let Some(position) = heap.pop() {
        // println!("Test {:?}", position);
        if (position.0, position.1) == goal {
            return Some(position.2);
        }

        if blizzmaps.len() <= position.2 {
            for blizzard in blizzards.iter_mut() {
                move_blizzard(blizzard, max_x, max_y);
            }

            blizzmaps.push(record_blizzmap(&blizzards, max_x, max_y));

            // print_blizzmap_bool(&blizzmaps[blizzmaps.len() - 1], max_x, max_y);
        }

        // println!("{:?}", dist);

        for edge in find_moves(&blizzmaps[position.2], max_x, max_y, position) {
            // println!("{:?}", edge);

            if !dist.contains(&edge) {
                // println!("!");
                heap.push(State(edge.0, edge.1, edge.2));
                dist.insert(edge);
            }
        }
    }

    // Goal not reachable
    None
}

fn find_moves(
    blizzmap: &Vec<bool>,
    max_x: usize,
    max_y: usize,
    position: State,
) -> Vec<(usize, usize, usize)> {
    let mut moves = Vec::new();

    if !sample_blizzmap(
        blizzmap,
        position.0 as isize,
        position.1 as isize,
        max_x,
        max_y,
    ) {
        moves.push((position.0 as usize, position.1 as usize, position.2 + 1));
    }

    const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    for dir in DIRS {
        let newpos = (position.0 as isize + dir.0, position.1 as isize + dir.1);

        let sample = sample_blizzmap(blizzmap, newpos.0, newpos.1, max_x, max_y);

        if !sample {
            moves.push((newpos.0 as usize, newpos.1 as usize, position.2 + 1));
        }
    }

    moves
}

fn sample_blizzmap(blizzmap: &Vec<bool>, x: isize, y: isize, max_x: usize, max_y: usize) -> bool {
    if x < 0 || x > max_x as isize || y < 0 || y > max_y as isize {
        true
    } else {
        blizzmap[x as usize + y as usize * (max_x + 1)]
    }
}

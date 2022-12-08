use rayon::prelude::*;

const GRID_SIZE: usize = 10000;

fn main() {
    if let Ok(content) = std::fs::read_to_string("bigboy.txt") {
        let mut trees = Vec::new();
        for line in content.lines() {
            trees.push(line.as_bytes());
        }

        let trees = trees.concat();
        let trees: Vec<u8> = trees.iter().map(|v| v - '0' as u8).collect();

        println!(
            "P1 {}",
            (0..GRID_SIZE)
                .into_par_iter()
                .map(|x| {
                    (0..GRID_SIZE)
                        .into_par_iter()
                        .map(|y| assess_score(&trees, x, y).1 as u64)
                        .sum::<u64>()
                })
                .sum::<u64>()
        );

        println!(
            "P2 {}",
            (0..GRID_SIZE)
                .into_par_iter()
                .map(|x| {
                    (0..GRID_SIZE)
                        .into_par_iter()
                        .map(|y| assess_score(&trees, x, y).0)
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap()
        );
    }
}

fn at_position(grid: &Vec<u8>, x: usize, y: usize) -> u8 {
    grid[x + y * GRID_SIZE]
}

fn assess_score(trees: &Vec<u8>, x: usize, y: usize) -> (usize, bool) {
    let tree = at_position(&trees, x, y);
    let mut obs = 0;

    let mut dist_w = x;
    for xq in (0..x).rev() {
        let opposing_tree = at_position(&trees, xq, y);
        if opposing_tree >= tree {
            dist_w = x - xq;
            obs += 1;
            break;
        }
    }

    let mut dist_e = GRID_SIZE - x - 1;
    for xq in (x + 1)..GRID_SIZE {
        let opposing_tree = at_position(&trees, xq, y);
        if opposing_tree >= tree {
            dist_e = xq - x;
            obs += 1;
            break;
        }
    }

    let mut dist_n = y;
    for yq in (0..y).rev() {
        let opposing_tree = at_position(&trees, x, yq);
        if opposing_tree >= tree {
            dist_n = y - yq;
            obs += 1;
            break;
        }
    }

    let mut dist_s = GRID_SIZE - y - 1;
    for yq in (y + 1)..GRID_SIZE {
        let opposing_tree = at_position(&trees, x, yq);
        if opposing_tree >= tree {
            dist_s = yq - y;
            obs += 1;
            break;
        }
    }

    (dist_e * dist_n * dist_s * dist_w, obs != 4)
}

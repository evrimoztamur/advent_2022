const GRID_SIZE: usize = 99;

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut trees = Vec::new();
        for line in content.lines() {
            trees.push(line.as_bytes());
        }

        let trees = trees.concat();
        let trees: Vec<u8> = trees.iter().map(|v| v - '0' as u8).collect();

        let mut visible_trees: u64 = 0;
        let mut max_score: usize = 0;

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let (score, visible) = assess_score(&trees, x, y);
                max_score = max_score.max(score);

                if score == max_score {
                    println!("{} {} {}", x, y, score);
                }

                visible_trees += visible as u64;
            }
        }

        println!("P1 {}", visible_trees);
        println!("P2 {}", max_score);
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

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    steps: u64,
    position: (isize, isize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: (isize, isize),
    cost: u64,
}

fn shortest_path(
    grid: (&Vec<u8>, isize),
    start: (isize, isize),
    goal: (isize, isize),
) -> Option<u64> {
    let mut dist: HashMap<(isize, isize), u64> = HashMap::new();

    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        steps: 0,
        position: start,
    });

    while let Some(State {
        cost,
        steps,
        position,
    }) = heap.pop()
    {
        if position == goal {
            return Some(steps);
        }

        if cost > dist[&position] {
            continue;
        }

        let spot_cost = at_position(grid, position.0, position.1);

        for edge in &adj_list(grid, position.0, position.1) {
            if let Some(edge) = edge {
                let next = State {
                    cost: cost + edge.cost,
                    steps: steps + 1,
                    position: edge.node,
                };

                if spot_cost != 'S' as u64 && edge.cost.checked_sub(spot_cost).unwrap_or(0) > 1 {
                    continue;
                }

                let dist_cost = dist.get(&next.position).unwrap_or(&u64::MAX);

                if next.cost < *dist_cost {
                    heap.push(next);
                    dist.insert(next.position, next.cost);
                }
            }
        }
    }

    None
}

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut grid = Vec::new();
        let mut grid_width = 0;
        for line in content.lines() {
            grid.push(line.as_bytes());
            grid_width = line.len() as isize;
        }

        let grid = grid.concat();
        let grid: Vec<u8> = grid.iter().map(|v| *v as u8).collect();
        let grid = (&grid, grid_width);

        let start = locate(grid, 'S' as u8);
        let goal = locate(grid, 'E' as u8);

        println!("P1 {:?}", shortest_path(grid, start, goal));

        let mut minsteps: Option<u64> = None;

        for start in grid
            .0
            .into_iter()
            .enumerate()
            .filter(|(_, v)| **v == 'a' as u8)
            .map(|(i, _)| into_position(grid_width, i as isize))
        {
            let steps = shortest_path(grid, start, goal);

            if let Some(steps) = steps {
                if let Some(comp) = minsteps {
                    minsteps = Some(steps.min(comp));
                } else {
                    minsteps = Some(steps);
                }
            }
        }

        println!("P2 {:?}", minsteps);
    }
}

fn into_position(width: isize, i: isize) -> (isize, isize) {
    (i % width, i / width)
}

fn locate(grid: (&Vec<u8>, isize), q: u8) -> (isize, isize) {
    into_position(
        grid.1,
        grid.0.into_iter().position(|v| *v == q).unwrap() as isize,
    )
}

fn at_position(grid: (&Vec<u8>, isize), x: isize, y: isize) -> u64 {
    grid.0[(x + y * grid.1) as usize] as u64
}

fn pos_edge(grid: (&Vec<u8>, isize), x: isize, y: isize) -> Option<Edge> {
    if x >= 0 && x < grid.1 && y >= 0 && y < (grid.0.len() as isize / grid.1) {
        Some(Edge {
            node: (x, y),
            cost: at_position(grid, x, y),
        })
    } else {
        None
    }
}

fn adj_list(grid: (&Vec<u8>, isize), x: isize, y: isize) -> Vec<Option<Edge>> {
    let mut edges = Vec::new();

    edges.push(pos_edge(grid, x - 1, y));
    edges.push(pos_edge(grid, x + 1, y));
    edges.push(pos_edge(grid, x, y - 1));
    edges.push(pos_edge(grid, x, y + 1));

    edges
}

use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    steps: usize,
    position: String,
    open_valves: Vec<String>,
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

struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

fn shortest_path(graph: &HashMap<String, Valve>, start: String) -> Option<usize> {
    let mut dist: HashMap<(String, Vec<String>), (usize, usize)> = HashMap::new();

    let mut heap = BinaryHeap::new();

    dist.insert((start.clone(), Vec::new()), (0, 0));

    heap.push(State {
        cost: 0,
        steps: 0,
        position: start,
        open_valves: Vec::new(),
    });

    let mut max_score = 0;

    while let Some(State {
        cost,
        steps,
        position,
        open_valves,
    }) = heap.pop()
    {
        let pressure: usize = open_valves
            .iter()
            .map(|v| graph.get(v).unwrap().flow_rate)
            .sum();

        // println!(
        //     "\n${}\tPosition: {}\nOpen: {:?} ({})\nCost: {}",
        //     steps, position, open_valves, pressure, cost
        // );

        if cost
            < dist
                .get(&(position.clone(), open_valves.clone()))
                .unwrap_or(&(cost, 0))
                .0
        {
            continue;
        }

        if steps + 1 > 30 {
            if cost >= max_score {
                max_score = cost;
                println!("{}", max_score);
            }
            continue;
        }

        if let Some(valve) = graph.get(&position) {
            if !open_valves.contains(&position) && valve.flow_rate > 0 {
                let mut new_open_valves = open_valves.clone();
                new_open_valves.push(position.clone());

                // println!(" -> Valve: {}", position);
                // println!("    Open: {:?}", new_open_valves);
                // println!("    Opening: {:?}", position);

                let next = State {
                    steps: steps + 1,
                    cost: cost + pressure,
                    position: position.clone(),
                    open_valves: new_open_valves.clone(),
                };

                heap.push(next.clone());
                dist.insert(
                    (next.position.clone(), new_open_valves.clone()),
                    (next.cost, next.steps),
                );
            }

            for edge in valve.tunnels.iter().map(|v| graph.get(v).unwrap()) {
                // println!(" -> Valve: {}", edge.name);
                // println!("    Open: {:?}", open_valves);

                let mut next = State {
                    steps: steps + 1,
                    cost: cost + pressure,
                    position: edge.name.clone(),
                    open_valves: open_valves.clone(),
                };

                // if next.steps >= 30 {
                //     if next.cost >= max_score {
                //         max_score = next.cost;
                //         println!("{}", max_score);
                //     }
                //     continue;
                // }

                let target = dist.get(&(next.position.clone(), open_valves.clone()));
                
                // println!("{:?}", target);

                if target.is_none() {
                    heap.push(next.clone());
                    dist.insert(
                        (next.position.clone(), open_valves.clone()),
                        (next.cost, next.steps),
                    );
                } else {
                    let target = target.unwrap();

                    // println!("{:?} {:?}", (next.cost, next.steps), default_next);

                    if (next.cost > target.0) || (next.cost >= target.0 && next.steps < target.1) {
                        heap.push(next.clone());
                        dist.insert(
                            (next.position.clone(), open_valves.clone()),
                            (next.cost, next.steps),
                        );
                    }
                }
            }
        }
    }

    Some(max_score)
}

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let re = Regex::new(
            r"Valve (.+) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.+)",
        )
        .unwrap();

        let mut graph: HashMap<String, Valve> = HashMap::new();

        for line in content.lines() {
            let caps = re.captures(line).unwrap();

            let valve_name = caps.get(1).unwrap().as_str();
            let valve_flow_rate = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let valve_connections: Vec<&str> = caps.get(3).unwrap().as_str().split(", ").collect();

            println!(
                "{} ~ {} -> {:?}",
                valve_name, valve_flow_rate, valve_connections
            );

            graph.insert(
                valve_name.to_string(),
                Valve {
                    name: valve_name.to_string(),
                    flow_rate: valve_flow_rate,
                    tunnels: valve_connections.iter().map(|v| v.to_string()).collect(),
                },
            );
        }

        println!("P1 {:?}", shortest_path(&graph, "AA".to_string()));
    }
}

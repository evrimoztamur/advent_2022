use regex::Regex;
use std::clone;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

#[derive(Clone, Eq, PartialEq, Debug)]
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
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
    connections: HashMap<String, usize>,
}

fn pressure(graph: &HashMap<String, Valve>, open_valves: &Vec<String>) -> usize {
    open_valves
        .iter()
        .map(|v| graph.get(v).unwrap().flow_rate)
        .sum()
}

fn shortest_path(graph: &HashMap<String, Valve>, start: String) -> Option<usize> {
    let mut heap = BinaryHeap::new();

    let start_state = State {
        cost: 0,
        steps: 0,
        position: start.clone(),
        open_valves: Vec::new(),
    };

    heap.push(start_state);

    let mut max_score = 0;

    while let Some(State {
        cost,
        steps,
        position,
        open_valves,
    }) = heap.pop()
    {
        let pressure: usize = pressure(&graph, &open_valves);

        let current_valve = graph.get(&position).unwrap();

        if !open_valves.contains(&position) && current_valve.flow_rate > 0 && steps < 30 {
            let mut new_open_valves = open_valves.clone();
            new_open_valves.push(position.clone());

            let next = State {
                steps: steps + 1,
                cost: cost + pressure,
                position: position.clone(),
                open_valves: new_open_valves.clone(),
            };

            heap.push(next.clone());
        } else {
            for (next_valve, distance) in current_valve
                .connections
                .iter()
                .map(|v| (graph.get(v.0).unwrap(), v.1))
            {
                if position == next_valve.name
                    || next_valve.flow_rate == 0
                    || steps + distance > 30
                    || open_valves.contains(&next_valve.name)
                {
                    continue;
                }

                let next = State {
                    steps: steps + distance,
                    cost: cost + pressure * distance,
                    position: next_valve.name.clone(),
                    open_valves: open_valves.clone(),
                };

                heap.push(next.clone());
            }
        }

        let perp_cost = cost + (30 - steps) * pressure;

        if perp_cost > max_score {
            max_score = perp_cost;
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
                    connections: HashMap::new(),
                },
            );
        }

        search_distances(&mut graph);
        println!("P1 {:?}", shortest_path(&graph, "AA".to_string()));
        // println!("P2 {:?}", shortest_elepath(&graph, "AA".to_string()));
    }
}

fn search_distances(graph: &mut HashMap<String, Valve>) {
    let old_graph = graph.clone();

    for (key, valve) in graph.iter_mut() {
        let mut connections: HashMap<String, usize> = HashMap::new();

        let mut q = VecDeque::new();
        q.push_back((key, 0));

        while let Some((current, distance)) = q.pop_front() {
            for node in &old_graph.get(current).unwrap().tunnels {
                if !connections.contains_key(node) {
                    connections.insert(node.clone(), distance + 1);
                    q.push_back((node, distance + 1));
                }
            }
        }

        valve.connections = connections;
    }
}

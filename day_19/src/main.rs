use std::collections::{HashSet, VecDeque};

use regex::Regex;

const DURATION_P1: usize = 24;

const DURATION_P2: usize = 32;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Blueprint {
    blueprint_id: usize,
    orebot_cost_ore: usize,
    claybot_cost_ore: usize,
    obsidianbot_cost_ore: usize,
    obsidianbot_cost_clay: usize,
    geodebot_cost_ore: usize,
    geodebot_cost_obsidian: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct ProductionState {
    blueprint: Blueprint,
    t: usize,
    duration: usize,
    ore: usize,
    orebots: usize,
    wip_orebots: usize,
    clay: usize,
    claybots: usize,
    wip_claybots: usize,
    obsidian: usize,
    obsidianbots: usize,
    wip_obsidianbots: usize,
    geode: usize,
    geodebots: usize,
    wip_geodebots: usize,
}

impl ProductionState {
    fn new(blueprint: Blueprint, duration: usize) -> ProductionState {
        ProductionState {
            blueprint,
            t: 0,
            duration,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            orebots: 1,
            claybots: 0,
            obsidianbots: 0,
            geodebots: 0,
            wip_orebots: 0,
            wip_claybots: 0,
            wip_obsidianbots: 0,
            wip_geodebots: 0,
        }
    }

    fn botstate(&self) -> (usize, usize, usize, usize, usize, usize, usize) {
        (
            self.ore,
            self.orebots,
            self.clay,
            self.claybots,
            self.obsidianbots,
            self.obsidian,
            self.geodebots,
        )
    }

    fn collect_resources(&mut self) {
        self.ore += self.orebots;

        self.clay += self.claybots;

        self.obsidian += self.obsidianbots;

        self.geode += self.geodebots;
    }

    fn produce_bots(&mut self) {
        self.orebots += self.wip_orebots;
        self.wip_orebots = 0;

        self.claybots += self.wip_claybots;
        self.wip_claybots = 0;

        self.obsidianbots += self.wip_obsidianbots;
        self.wip_obsidianbots = 0;

        self.geodebots += self.wip_geodebots;
        self.wip_geodebots = 0;
    }

    fn decide_production(&self) -> Vec<ProductionState> {
        let mut choices = Vec::new();

        choices.push(self.clone());

        if self.ore >= self.blueprint.geodebot_cost_ore
            && self.obsidian >= self.blueprint.geodebot_cost_obsidian
        {
            let mut new_state = self.clone();

            new_state.ore -= self.blueprint.geodebot_cost_ore;
            new_state.obsidian -= self.blueprint.geodebot_cost_obsidian;
            new_state.wip_geodebots += 1;

            choices.push(new_state);

            return choices;
        }

        if self.ore >= self.blueprint.obsidianbot_cost_ore
            && self.clay >= self.blueprint.obsidianbot_cost_clay
            && self.obsidianbots < 12
        {
            let mut new_state = self.clone();

            new_state.ore -= self.blueprint.obsidianbot_cost_ore;
            new_state.clay -= self.blueprint.obsidianbot_cost_clay;
            new_state.wip_obsidianbots += 1;

            choices.push(new_state);

            return choices;
        }

        if self.ore >= self.blueprint.claybot_cost_ore && self.claybots < 12 {
            let mut new_state = self.clone();

            new_state.ore -= self.blueprint.claybot_cost_ore;
            new_state.wip_claybots += 1;

            choices.push(new_state);
        }

        if self.ore >= self.blueprint.orebot_cost_ore && self.orebots < 12 {
            let mut new_state = self.clone();

            new_state.ore -= self.blueprint.orebot_cost_ore;
            new_state.wip_orebots += 1;

            choices.push(new_state);
        }

        choices
    }

    fn resolve_p1(self) -> usize {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        seen.insert(self.botstate());
        queue.push_back(self.clone());

        let mut max_geode: usize = 0;

        while let Some(production_state) = queue.pop_back() {
            let mut choices = production_state.decide_production();

            for choice in choices.iter_mut() {
                choice.collect_resources();
                choice.produce_bots();

                choice.t += 1;

                if choice.t == choice.duration {
                    if max_geode < choice.geode {
                        max_geode = choice.geode;
                        // println!("{:?} {}", choice, max_geode);
                    }
                } else {
                    if seen.contains(&choice.botstate()) {
                        continue;
                    }

                    seen.insert(choice.botstate());
                    queue.push_back(*choice);
                }
            }
        }

        max_geode
    }
}

fn main() {
    if let Ok(content) = std::fs::read_to_string("example.txt") {
        let re = Regex::new(
            r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
        )
        .unwrap();

        let mut blueprints: Vec<Blueprint> = Vec::new();

        for line in content.lines() {
            let caps = re.captures(line).unwrap();

            let blueprint_id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let orebot_cost_ore = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let claybot_cost_ore = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let obsidianbot_cost_ore = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let obsidianbot_cost_clay = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();
            let geodebot_cost_ore = caps.get(6).unwrap().as_str().parse::<usize>().unwrap();
            let geodebot_cost_obsidian = caps.get(7).unwrap().as_str().parse::<usize>().unwrap();

            blueprints.push(Blueprint {
                blueprint_id,
                orebot_cost_ore,
                claybot_cost_ore,
                obsidianbot_cost_ore,
                obsidianbot_cost_clay,
                geodebot_cost_ore,
                geodebot_cost_obsidian,
            });
        }

        let mut p1 = 0;

        for blueprint in blueprints.iter() {
            println!("{:?}", blueprint);
            let production_state = ProductionState::new(*blueprint, DURATION_P1);
            let geodes = production_state.resolve_p1();
            p1 += geodes * blueprint.blueprint_id;
        }

        println!("P1 {}", p1);
        // let mut p2 = 1;

        // for blueprint in blueprints.iter().take(3) {
        //     println!("{:?}", blueprint);
        //     let production_state = ProductionState::new(*blueprint, DURATION_P2);
        //     let geodes = production_state.resolve_p1();
        //     p2 *= geodes;
        // }

        // println!("P1 {}", p2);
    }
}

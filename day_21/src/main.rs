use core::panic;
use std::collections::{HashMap, VecDeque};

use regex::Regex;

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Operator {
    fn from(op: &str) -> Operator {
        match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("unrecognised operator: {}", op),
        }
    }

    fn resolve(&self, lhs: isize, rhs: isize) -> isize {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
            Operator::Eq => (lhs == rhs) as isize,
        }
    }
}

struct Operation<'a> {
    lhs: &'a str,
    rhs: &'a str,
    op: Operator,
}

impl<'a> Operation<'a> {
    fn from(lhs: &'a str, op: &str, rhs: &'a str) -> Operation<'a> {
        Operation {
            lhs: lhs,
            rhs: rhs,
            op: Operator::from(op),
        }
    }
}

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let re_number = Regex::new(r"([a-z]{4}): (\d+)").unwrap();
        let re_equation = Regex::new(r"([a-z]{4}): ([a-z]{4}) (.) ([a-z]{4})").unwrap();

        let mut variables: HashMap<&str, isize> = HashMap::new();
        let mut operations: HashMap<&str, Operation> = HashMap::new();

        let mut operation_order: Vec<&str> = Vec::new();

        for line in content.lines() {
            if let Some(re_number_cap) = re_number.captures(line) {
                let name = re_number_cap.get(1).unwrap().as_str();
                let value = re_number_cap
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<isize>()
                    .unwrap();

                variables.insert(name, value);
                operation_order.push(name);

                // println!("{} = {}", name, value);
            } else if let Some(re_equation_cap) = re_equation.captures(line) {
                let name = re_equation_cap.get(1).unwrap().as_str();
                let lhs = re_equation_cap.get(2).unwrap().as_str();
                let op = re_equation_cap.get(3).unwrap().as_str();
                let rhs = re_equation_cap.get(4).unwrap().as_str();

                operations.insert(name, Operation::from(lhs, op, rhs));
                operation_order.push(name);

                // println!("{} = {} {} {}", name, lhs, op, rhs);
            } else {
                panic!("neither regex captured line {}", line);
            }
        }

        *variables.get_mut("humn").unwrap() = 3059361893920; // Manual LOL
        // *variables.get_mut("humn").unwrap() = 301;
        operations.get_mut("root").unwrap().op = Operator::Eq;

        let mut queue = VecDeque::new();

        queue.push_back("root");

        while let Some(operation_name) = queue.pop_front() {
            // println!("Processing: {}", operation_name);

            let operation = operations.get(operation_name);

            if operation.is_none() {
                continue;
            }

            let operation = operation.unwrap();

            let lhs_value = variables.get(operation.lhs);
            let rhs_value = variables.get(operation.rhs);

            if lhs_value.is_some() && rhs_value.is_some() {
                let result = operation
                    .op
                    .resolve(*lhs_value.unwrap(), *rhs_value.unwrap());

                println!(
                    "{} (= {}) {:?} {} (= {}) = {}",
                    operation.lhs,
                    lhs_value.unwrap(),
                    operation.op,
                    operation.rhs,
                    rhs_value.unwrap(),
                    result
                );

                variables.insert(
                    operation_name,
                    operation
                        .op
                        .resolve(*lhs_value.unwrap(), *rhs_value.unwrap()),
                );

                operations.remove(operation_name);
            } else {
                if lhs_value.is_none() {
                    queue.push_front(operation.lhs);
                }

                if rhs_value.is_none() {
                    queue.push_front(operation.rhs);
                }

                queue.push_back(operation_name);
            }
        }

        println!("P1 {}", variables.get("root").unwrap());
    }
}

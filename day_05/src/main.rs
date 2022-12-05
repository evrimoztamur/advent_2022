use std::fs;

#[derive(Debug)]
struct Instruction {
    num_crates: usize,
    from: usize,
    to: usize,
}

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let lines: Vec<&str> = content.lines().collect();
        let mut stacks: Vec<Vec<u8>> = vec![vec![]; 9];

        for line in &lines[..9] {
            for i in 0..=8usize {
                let crate_id = line.as_bytes()[1 + i * 4];

                if crate_id != 32 {
                    stacks[i].push(crate_id);
                }
            }
        }

        for stack in &mut stacks {
            stack.reverse();
        }

        let mut stacks_p2 = stacks.clone();

        let mut instructions = Vec::new();

        for line in &lines[10..] {
            let params: Vec<&str> = line.split(" ").collect();

            let instruction = Instruction {
                num_crates: params[1].parse().unwrap(),
                from: params[3].parse::<usize>().unwrap() - 1,
                to: params[5].parse::<usize>().unwrap() - 1,
            };

            instructions.push(instruction);
        }

        {
            for instruction in &instructions {
                execute_instruction_9000(&mut stacks, &instruction);
            }

            print!("P1 ");
            for stack in stacks {
                print!("{}", *stack.last().unwrap() as char);
            }
            println!();
        }

        {
            for instruction in &instructions {
                execute_instruction_9001(&mut stacks_p2, &instruction);
            }

            print!("P2 ");
            for stack in stacks_p2 {
                print!("{}", *stack.last().unwrap() as char);
            }
            println!();
        }
    }
}

fn execute_instruction_9000(stacks: &mut Vec<Vec<u8>>, instruction: &Instruction) {
    for _ in 0..instruction.num_crates {
        let crate_id = stacks[instruction.from].pop().unwrap();
        stacks[instruction.to].push(crate_id);
    }
}

fn execute_instruction_9001(stacks: &mut Vec<Vec<u8>>, instruction: &Instruction) {
    let mut intermediate_stack = Vec::new();
    for _ in 0..instruction.num_crates {
        let crate_id = stacks[instruction.from].pop().unwrap();
        intermediate_stack.push(crate_id);
    }
    intermediate_stack.reverse();
    stacks[instruction.to].append(&mut intermediate_stack);
}

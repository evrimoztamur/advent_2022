use std::fs;

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let mut elves: Vec<Vec<u64>> = Vec::new();
        let mut curr_elf: Vec<u64> = Vec::new();

        for line in content.lines() {
            if line.len() == 0 {
                elves.push(curr_elf);
                curr_elf = Vec::new();
            } else {
                curr_elf.push(line.parse().unwrap());
            }
        }

        let mut elf_totals: Vec<u64> = elves
            .into_iter()
            .map(|calories| calories.iter().sum::<u64>())
            .collect();

        elf_totals.sort();

        println!("P1 {}", elf_totals.last().unwrap());
        println!("P2 {}", elf_totals.iter().rev().take(3).sum::<u64>());
    }
}

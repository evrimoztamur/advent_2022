use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn score(item: &char) -> u64 {
    let item = *item;

    if item >= 'a' && item <= 'z' {
        item as u64 - 'a' as u64 + 1
    } else {
        item as u64 - 'A' as u64 + 27
    }
}

fn strref_into_hashset(input: &str) -> HashSet<char> {
    HashSet::from_iter(input.chars().into_iter())
}

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let mut total_p1 = 0;
        for line in content.lines() {
            let h1 = &line[..(line.len() / 2)];
            let h2 = &line[(line.len() / 2)..];

            let h1s = strref_into_hashset(h1);
            let h2s = strref_into_hashset(h2);

            total_p1 += h1s.intersection(&h2s).into_iter().map(score).sum::<u64>();
        }

        println!("P1 {}", total_p1);

        let mut total_p2 = 0;
        for lines in &content.lines().chunks(3) {
            let lines = lines.collect::<Vec<&str>>();

            let h1s = strref_into_hashset(lines[0]);
            let h2s = strref_into_hashset(lines[1]);
            let h3s = strref_into_hashset(lines[2]);

            total_p2 += h3s
                .intersection(&h1s.intersection(&h2s).cloned().collect::<HashSet<char>>())
                .into_iter()
                .map(score)
                .sum::<u64>();
        }

        println!("P2 {}", total_p2);
    }
}

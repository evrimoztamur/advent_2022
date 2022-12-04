use std::fs;

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let mut total_p1 = 0u64;
        let mut total_p2 = 0u64;

        for line in content.lines() {
            let elf_sections = line.split(",").collect::<Vec<&str>>();
            let elf1_range = elf_sections[0].split("-").collect::<Vec<&str>>();
            let elf1_range =
                elf1_range[0].parse::<u64>().unwrap()..=elf1_range[1].parse::<u64>().unwrap();

            let elf2_range = elf_sections[1].split("-").collect::<Vec<&str>>();
            let elf2_range =
                elf2_range[0].parse::<u64>().unwrap()..=elf2_range[1].parse::<u64>().unwrap();

            if (elf1_range.start() >= elf2_range.start() && elf1_range.end() <= elf2_range.end())
                || (elf2_range.start() >= elf1_range.start()
                    && elf2_range.end() <= elf1_range.end())
            {
                total_p1 += 1;
            }

            if elf1_range.contains(elf2_range.start())
                || elf1_range.contains(elf2_range.end())
                || elf2_range.contains(elf1_range.start())
                || elf2_range.contains(elf1_range.end())
            {
                total_p2 += 1;
            }
        }

        println!("P1 {}", total_p1);
        println!("P2 {}", total_p2);
    }
}

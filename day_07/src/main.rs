use std::collections::HashMap;

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.history") {
        let mut pwd: Vec<&str> = vec![];

        let mut directory_flat: HashMap<Vec<&str>, u64> = HashMap::new();

        for line in content.lines() {
            if line.starts_with("$ cd") {
                if line.eq("$ cd ..") {
                    pwd.pop();
                } else {
                    directory_flat.entry(pwd.clone()).or_insert(0);
                    pwd.push(line.strip_prefix("$ cd ").unwrap());
                }
            } else if line.ne("$ ls") && !line.starts_with("dir") {
                let file_attributes: Vec<&str> = line.split(" ").collect();
                let filesize = file_attributes[0].parse::<u64>().unwrap();

                directory_flat
                    .entry(pwd.clone())
                    .and_modify(|dirsize| *dirsize += filesize)
                    .or_insert(filesize);
            }
        }

        let pwd = vec!["/"];
        let mut directory_complete: HashMap<Vec<&str>, u64> = HashMap::new();

        let disk = calculate_dir_size(&pwd, &directory_flat, &mut directory_complete);

        let mut sum = 0;

        for v in directory_complete.values() {
            if *v <= 100_000 {
                sum += *v;
            }
        }

        println!("P1 {}", sum);

        let mut sizes: Vec<&u64> = directory_complete.values().collect();
        sizes.sort();

        for v in sizes {
            if *v + 40_000_000 > disk {
                println!("P2 {}", *v);
                break;
            }
        }
    }
}

fn calculate_dir_size<'a>(
    pwd: &Vec<&'a str>,
    directory_flat: &HashMap<Vec<&'a str>, u64>,
    directory_complete: &mut HashMap<Vec<&'a str>, u64>,
) -> u64 {
    let mut total = *(directory_flat.get(pwd).unwrap());

    for dir in directory_flat.keys() {
        if dir.len() == pwd.len() + 1 && dir.starts_with(pwd.as_slice()) {
            let dsize = calculate_dir_size(dir, directory_flat, directory_complete);
            total += dsize
        }
    }

    directory_complete.insert(pwd.clone(), total);

    total
}

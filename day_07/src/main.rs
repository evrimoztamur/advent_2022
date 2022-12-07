use std::collections::HashMap;

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.history") {
        let mut pwd: Vec<&str> = vec![];

        let mut directory_flat: HashMap<Vec<&str>, u64> = HashMap::new();

        for line in content.lines() {
            if line.starts_with("$ cd") {
                if line.eq("$ cd ..") {
                    if let Some(directory_departed) = pwd.pop() {
                        // println!("$ cd .. <- {}", directory_departed);
                    }
                } else {
                    let directory_entered = line.strip_prefix("$ cd ").unwrap();
                    // println!("$ cd -> {}", directory_entered);
                    // println!("\t{:?}", pwd);
                    pwd.push(directory_entered);
                }
            } else if line.ne("$ ls") {
                let file_attributes: Vec<&str> = line.split(" ").collect();

                if let Ok(filesize) = file_attributes[0].parse::<u64>() {
                    // println!("{} {}", filesize, file_attributes[1]);
                    if let Some(dirsize) = directory_flat.get(&pwd) {
                        directory_flat.insert(pwd.clone(), dirsize + filesize);
                    } else {
                        directory_flat.insert(pwd.clone(), filesize);
                    }
                }
            }
        }

        // let mut sum = 0;

        // for (k, v) in directory_complete.iter() {
        //     if *v <= 100_000 {
        //         sum += *v;
        //     }
        // }

        // println!("P1 {}", sum);
    }
}

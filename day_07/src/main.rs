use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.history") {
        process_tree(content);
        // process_vector(content);
    }
}

// Tree-based approach: Obnoxious Rc-based memory management but more efficient

#[derive(Clone, Debug)]
struct File {
    size: u64,
    collective_size: u64,
    children: Vec<Rc<RefCell<File>>>,
    parent: Option<Rc<RefCell<File>>>,
}

fn process_tree(content: String) {
    let root = Rc::new(RefCell::new(File {
        size: 0,
        collective_size: 0,
        children: Vec::new(),
        parent: None,
    }));

    let mut cursor = root.clone();

    for line in content.lines() {
        if line.starts_with("$ cd") {
            if line.eq("$ cd ..") {
                let cursor_clone = Rc::clone(&cursor);
                cursor = Rc::clone(cursor_clone.borrow().parent.as_ref().unwrap());
            } else {
                let new_directory = Rc::new(RefCell::new(File {
                    size: 0,
                    collective_size: 0,
                    children: Vec::new(),
                    parent: Some(cursor.clone()),
                }));

                cursor.borrow_mut().children.push(new_directory.clone());
                cursor = new_directory.clone();
            }
        } else if line.ne("$ ls") && !line.starts_with("dir") {
            let file_attributes: Vec<&str> = line.split(" ").collect();
            let filesize = file_attributes[0].parse::<u64>().unwrap();

            cursor.borrow_mut().size += filesize;
        }
    }

    let disk_size = calculate_collective_size(&root);

    println!("P1 {:?}", calculate_p1(&root, 0));
    println!(
        "P2 {:?}",
        calculate_p2(&root, disk_size - 40_000_000, disk_size)
    );
}

fn calculate_collective_size(file: &Rc<RefCell<File>>) -> u64 {
    let mut file = file.borrow_mut();
    let mut collective_size = file.size;

    for child_file in file.children.iter() {
        collective_size += calculate_collective_size(child_file);
    }

    file.collective_size = collective_size;

    collective_size
}

fn calculate_p1(file: &Rc<RefCell<File>>, total: u64) -> u64 {
    let file = file.borrow_mut();
    let mut total = total;

    for child_file in file.children.iter() {
        total = calculate_p1(&child_file, total);

        let collective_size = child_file.borrow().collective_size;

        if collective_size <= 100_000 {
            total += collective_size;
        }
    }

    total
}

fn calculate_p2(file: &Rc<RefCell<File>>, threshold: u64, candidate: u64) -> Option<u64> {
    let file = file.borrow_mut();

    let mut smallest_candidate = candidate;

    for child_file in file.children.iter() {
        let candidate = match calculate_p2(&child_file, threshold, candidate) {
            Some(result) => result,
            None => candidate,
        };

        let candidate = child_file.borrow().collective_size.min(candidate);

        if candidate > threshold {
            smallest_candidate = smallest_candidate.min(candidate);
        }
    }

    Some(smallest_candidate)
}

// Vector-based approach: Slower computation but more straightforward memory management

fn process_vector(content: String) {
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

fn calculate_dir_size<'a>(
    pwd: &Vec<&'a str>,
    directory_flat: &HashMap<Vec<&'a str>, u64>,
    directory_complete: &mut HashMap<Vec<&'a str>, u64>,
) -> u64 {
    let mut total = *(directory_flat.get(pwd).unwrap());

    for dir in directory_flat.keys() {
        // Determine first-level children in the filesystem
        if dir.len() == pwd.len() + 1 && dir.starts_with(pwd.as_slice()) {
            let dsize = calculate_dir_size(dir, directory_flat, directory_complete);
            total += dsize
        }
    }

    directory_complete.insert(pwd.clone(), total);

    total
}

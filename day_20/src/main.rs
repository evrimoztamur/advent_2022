use std::collections::VecDeque;

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut ring: Box<[(usize, usize, isize)]> = content
            .lines()
            .enumerate()
            .map(|(i, l)| (i, i, l.parse::<isize>().unwrap()))
            .collect();

        // i: Original position
        // j: Current position

        println!("{:?}", ring);

        for i in 0..ring.len() {
            let x = ring.iter().position(|(j, q, v)| *j == i).unwrap();

            let (_, j, v) = ring[x];

            let wj = wrap(j as isize + v, ring.len());

            // println!();
            // println!("Step {} [{}] {} -> {}", v, i, j, wj);
            // println!("{:?}", ring);

            if wj > j {
                for q in ring.iter_mut() {
                    if q.1 <= wj && q.1 > j {
                        q.1 -= 1;
                    }
                }

                ring[i].1 = wj;
            } else if wj < j {
                for q in ring.iter_mut() {
                    if q.1 >= wj && q.1 < j {
                        q.1 += 1;
                    }
                }

                ring[i].1 = wj;
            }

            // println!("{:?}", ring);

            // let mut sorted = ring.clone();
            // sorted.sort_by(|(i0, j0, v0), (i1, j1, v1)| j0.cmp(j1));

            // println!("{:?}", sorted.iter().map(|v| v.2).collect::<Vec<isize>>());
        }

        let mut sorted = ring.clone();
        sorted.sort_by(|(i0, j0, v0), (i1, j1, v1)| j0.cmp(j1));

        println!("{:?}", sorted.iter().map(|v| v.2).collect::<Vec<isize>>());

        let zero = sorted.iter().position(|(j, q, v)| *v == 0).unwrap();

        let k1 = sorted[(zero + 1000) % sorted.len()];
        let k2 = sorted[(zero + 2000) % sorted.len()];
        let k3 = sorted[(zero + 3000) % sorted.len()];

        println!("P1 {}", k1.2 + k2.2 + k3.2);
    }
}

fn wrap(v: isize, len: usize) -> usize {
    let mut v = v;
    let ilen = len as isize - 1;

    while v < 0 {
        v += ilen;
    }

    while v > ilen {
        v -= ilen;
    }

    if v == 0 {
        len - 1
    } else {
        v as usize
    }
}

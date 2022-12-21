fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut ring: Box<[(usize, usize, isize)]> = content
            .lines()
            .enumerate()
            .map(|(i, l)| (i, i, l.parse::<isize>().unwrap() * 811589153))
            .collect();

        for _ in 0..10 {
            for i in 0..ring.len() {
                let x = ring.iter().position(|a| a.0 == i).unwrap();
                let (_, j, v) = ring[x];
                let wj = wrap(j as isize + v, ring.len());

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
            }
        }

        let mut sorted = ring.clone();
        sorted.sort_by(|a, b| a.1.cmp(&b.1));

        let zero = sorted.iter().position(|a| a.2 == 0).unwrap();

        let k1 = sorted[(zero + 1000) % sorted.len()];
        let k2 = sorted[(zero + 2000) % sorted.len()];
        let k3 = sorted[(zero + 3000) % sorted.len()];

        println!(
            "P2 {} {:?} {:?} {:?}",
            (k1.2 + k2.2 + k3.2),
            k1.2,
            k2.2,
            k3.2
        );
    }
}

fn wrap(v: isize, len: usize) -> usize {
    let mut v = v;
    let ilen = len as isize - 1;

    v = ((v % ilen) + ilen) % ilen;

    if v == 0 {
        len - 1
    } else {
        v as usize
    }
}

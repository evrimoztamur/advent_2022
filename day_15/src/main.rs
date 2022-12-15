use regex::Regex;

fn manhattan(a: (i64, i64), b: (i64, i64)) -> i64 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut pairs: Vec<((i64, i64), (i64, i64), i64)> = Vec::new();

        for line in content.lines() {
            let re = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
            )
            .unwrap();

            let caps = re.captures(line).unwrap();

            let sensor = (
                caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            );

            let beacon = (
                caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            );

            let dist = manhattan(sensor, beacon);
            // println!("{:?} {:?}", sensor, beacon);
            // println!("{}", dist);

            pairs.push((sensor, beacon, dist));
        }

        let minx = pairs.iter().map(|v| v.0 .0 - v.2).min().unwrap();
        let maxx = pairs.iter().map(|v| v.0 .0 + v.2).max().unwrap();

        // println!("{}..={}", minx, maxx);

        let mut rowsize = 0;

        let y = 2000000;
        for x in minx..=maxx {
            for (sensor, beacon, dist) in pairs.iter() {
                if manhattan((x, y), *sensor) <= *dist && (x, y) != *beacon {
                    rowsize += 1;
                    break;
                }
            }
        }

        println!("P1 {}", rowsize);

        for x in 0..=4000000 {
            let mut y = 0;

            while y <= 4000000 {
                let mut mv = false;

                for (sensor, _, dist) in pairs.iter() {
                    let md = manhattan((x, y), *sensor);
                    if md <= *dist {
                        mv = true;
                        y += (md - *dist).abs();
                        break;
                    }
                }

                if !mv {
                    println!("P2 {}", (x * 40000000) + y);
                    return;
                }

                y += 1;
            }
        }
    }
}

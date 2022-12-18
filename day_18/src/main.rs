use std::collections::{HashSet, VecDeque};

type Point = (i8, i8, i8);

fn main() {
    const DIRS: [Point; 6] = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let points: Vec<Point> = content
            .lines()
            .map(|v| {
                v.split(",")
                    .map(|v| v.parse::<i8>().unwrap())
                    .collect::<Vec<i8>>()
            })
            .map(|v| vec_as_triple(v))
            .collect();

        let points: HashSet<Point> = points.into_iter().collect();

        println!(
            "P1 {}",
            points
                .iter()
                .flat_map(|v| DIRS.into_iter().map(|dir| add_triples(dir, *v)))
                .fold(0, |acc, v| acc + !points.contains(&v) as usize)
        );

        let maxs = add_triples(
            points.iter().fold((0, 0, 0), |acc, v| {
                (acc.0.max(v.0), acc.1.max(v.1), acc.2.max(v.2))
            }),
            (1, 1, 1),
        );
        let mins = add_triples(
            points.iter().fold(maxs, |acc, v| {
                (acc.0.min(v.0), acc.1.min(v.1), acc.2.min(v.2))
            }),
            (-1, -1, -1),
        );

        let mut surface = 0;

        let mut seen: HashSet<Point> = HashSet::new();
        let mut boundaries = VecDeque::new();

        seen.insert(mins);
        boundaries.push_back(mins);

        while let Some(boundary) = boundaries.pop_back() {
            // Find valid neighbours to search (i.e. within boundaries and not already searched)

            let surrounding: Vec<Point> = DIRS
                .into_iter()
                .map(|dir| add_triples(dir, boundary))
                .filter(|v| in_bounds(v, mins, maxs) && !seen.contains(v))
                .collect();

            // Find neighbours which are a part of the blob

            let intersecting = surrounding
                .iter()
                .filter(|v| points.contains(v))
                .collect::<Vec<&Point>>();

            // Sum of intersections are adjacent 'filled' surfaces

            surface += intersecting.len();

            // Search empty neighbours

            for next in surrounding.iter().filter(|v| !points.contains(v)) {
                seen.insert(*next);
                boundaries.push_back(*next);
            }
        }

        println!("P2 {}", surface);
    }
}

fn vec_as_triple(vector: Vec<i8>) -> Point {
    (vector[0], vector[1], vector[2])
}

fn add_triples(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn in_bounds(a: &Point, mins: Point, maxs: Point) -> bool {
    (mins.0..=maxs.0).contains(&a.0)
        && (mins.1..=maxs.1).contains(&a.1)
        && (mins.2..=maxs.2).contains(&a.2)
}

use regex::Regex;

enum Action {
    Move(usize),
    Turn(char),
}

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut reading_map = true;

        let mut actions: Vec<Action> = Vec::new();
        let mut map: Vec<u8> = Vec::new();
        let mut map_width = 0;

        for line in content.lines() {
            if line.is_empty() {
                reading_map = false;
                continue;
            }

            if reading_map {
                map_width = map_width.max(line.len());

                for char in line.chars() {
                    map.push(match char {
                        '.' => 1,
                        '#' => 2,
                        _ => 0,
                    });
                }

                for _ in 0..(map_width - line.len()) {
                    map.push(0);
                }
            } else {
                let re = Regex::new(r"(\d+)|([RDLU])").unwrap();

                for caps in re.captures_iter(line) {
                    if let Some(length) = caps.get(1) {
                        let length = length.as_str().parse::<usize>().unwrap();
                        // println!("Move {}", length);
                        actions.push(Action::Move(length));
                    } else if let Some(direction) = caps.get(2) {
                        let direction = direction.as_str();
                        // println!("Turn {}", direction);
                        actions.push(Action::Turn(direction.chars().nth(0).unwrap()));
                    }
                }
            }
        }

        let map_height = map.len() / map_width;

        println!("{} {} {}", map.len(), map_width, (map.len() / map_width));

        // let mut x = map.iter().position(|v| *v == 1).unwrap() as isize;
        // let mut y = 0;
        // let mut direction = 'R';

        // for action in actions {
        //     match action {
        //         Action::Move(length) => {
        //             for i in 0..length {
        //                 // println!("@ : {},{}", x, y);

        //                 let intent = match direction {
        //                     'R' => (x + 1, y),
        //                     'D' => (x, y + 1),
        //                     'L' => (x - 1, y),
        //                     'U' => (x, y - 1),
        //                     _ => panic!(),
        //                 };

        //                 // println!("{}   {},{}", direction, intent.0, intent.1);

        //                 let result = sample_map(&map, map_width, intent.0, intent.1, direction);

        //                 if result.2 == 1 {
        //                     // println!(". : {},{}", result.0, result.1);
        //                     x = result.0;
        //                     y = result.1;
        //                 } else {
        //                     // println!("# : {},{}", intent.0, intent.1);
        //                 }
        //             }
        //         }
        //         Action::Turn(d) => direction = turn(direction, d),
        //     }
        // }

        // let fd = match direction {
        //     'R' => 0,
        //     'D' => 1,
        //     'L' => 2,
        //     'U' => 3,
        //     _ => panic!(),
        // };

        // println!("P1 {}", (y + 1) * 1000 + (x + 1) * 4 + fd);

        // let face_width = map_width / 4;
        // let face_height = map_height / 3;
        let face_width = map_width / 3;
        let face_height = map_height / 4;

        println!(
            "{} {} {} {}",
            map_width, map_height, face_width, face_height
        );

        let mut faces = Vec::new();
        let mut face = 0;

        for y in (0..map_height).step_by(face_height) {
            for x in (0..map_width).step_by(face_width) {
                if map[x + y * map_width] != 0 {
                    face += 1;
                    faces.push(Some(face));
                } else {
                    faces.push(None);
                }
            }
        }

        println!("{:?}", faces);

        let mut x = map.iter().position(|v| *v == 1).unwrap() as isize;
        let mut y = 0;
        let mut direction = 'R';

        for action in actions {
            match action {
                Action::Move(length) => {
                    for i in 0..length {
                        // println!("@ : {},{}", x, y);

                        let intent = match direction {
                            'R' => (x + 1, y),
                            'D' => (x, y + 1),
                            'L' => (x - 1, y),
                            'U' => (x, y - 1),
                            _ => panic!(),
                        };

                        // println!("{}   {},{}", direction, intent.0, intent.1);

                        let result = sample_map_cube(
                            &map,
                            map_width,
                            face_width,
                            face_height,
                            intent.0,
                            intent.1,
                            x,
                            y,
                            direction,
                        );

                        if result.3 == 1 {
                            // println!(". : {},{}", result.0, result.1);
                            x = result.0;
                            y = result.1;
                            direction = result.2;
                        } else {
                            // println!("# : {},{}", intent.0, intent.1);
                        }
                    }
                }
                Action::Turn(d) => direction = turn(direction, d),
            }
        }

        let fd = match direction {
            'R' => 0,
            'D' => 1,
            'L' => 2,
            'U' => 3,
            _ => panic!(),
        };

        println!("P2 {}", (y + 1) * 1000 + (x + 1) * 4 + fd);
    }
}

fn sample_map(
    map: &Vec<u8>,
    map_width: usize,
    x: isize,
    y: isize,
    direction: char,
) -> (isize, isize, u8) {
    let map_height = map.len() / map_width;
    let mut x = x;
    let mut y = y;

    if x < 0 {
        x += map_width as isize;
    }

    if y < 0 {
        y += map_height as isize;
    }

    x %= map_width as isize;
    y %= map_height as isize;

    let ux = x as usize;
    let uy = y as usize;

    let mut tile = map[ux + uy * map_width];

    if tile == 0 {
        let mut wl = 0;

        while tile == 0 {
            wl += 1;

            match direction {
                'R' => tile = map[wmod(x + wl, map_width as isize) as usize + uy * map_width],
                'D' => tile = map[ux + wmod(y + wl, map_height as isize) as usize * map_width],
                'L' => tile = map[wmod(x - wl, map_width as isize) as usize + uy * map_width],
                'U' => tile = map[ux + wmod(y - wl, map_height as isize) as usize * map_width],
                _ => panic!(),
            }
        }

        match direction {
            'R' => (wmod(x + wl, map_width as isize), y, tile),
            'D' => (x, wmod(y + wl, map_height as isize), tile),
            'L' => (wmod(x - wl, map_width as isize), y, tile),
            'U' => (x, wmod(y - wl, map_height as isize), tile),
            _ => panic!(),
        }
    } else {
        (x, y, tile)
    }
}

fn sample_map_cube(
    map: &Vec<u8>,
    map_width: usize,
    face_width: usize,
    face_height: usize,
    x: isize,
    y: isize,
    fx: isize,
    fy: isize,
    direction: char,
) -> (isize, isize, char, u8) {
    let map_height = map.len() / map_width;
    let mut x = x;
    let mut y = y;

    if x < 0 {
        x += map_width as isize;
    }

    if y < 0 {
        y += map_height as isize;
    }

    x %= map_width as isize;
    y %= map_height as isize;

    let ux = x as usize;
    let uy = y as usize;

    let ifw = face_width as isize;
    let ifh = face_height as isize;

    let current_face = ((fx + ifw) / ifw as isize, (fy + ifh) / ifh as isize);
    let intent_face = ((x + ifw) / ifw as isize, (y + ifh) / ifh as isize);

    if current_face != intent_face {
        println!(
            "! ({:?}, {:?}) => ((x, y), direction)",
            current_face, intent_face
        );

        let ((x, y), direction) = transform(
            x,
            y,
            direction,
            face_width,
            face_height,
            current_face,
            intent_face,
        );

        println!("{} % {},{}", direction, x, y);

        sample_map_cube(
            map,
            map_width,
            face_width,
            face_height,
            x,
            y,
            x,
            y,
            direction,
        )
    } else {
        let tile = map[ux + uy * map_width];

        if tile == 0 {
            panic!("{} {} <- {} {}", x, y, fx, fy)
        }

        (x, y, direction, tile)
    }
}

fn transform(
    x: isize,
    y: isize,
    direction: char,
    face_width: usize,
    face_height: usize,
    current_face: (isize, isize),
    intent_face: (isize, isize),
) -> ((isize, isize), char) {
    match (current_face, intent_face) {
        // ((3, 1), (3, 2)) => ((x, y), direction),
        // ((3, 2), (4, 2)) => (
        //     mtxpos(x, y, face_width, face_height, 4, 3, 'R', 1),
        //     mturn(direction, 'R', 1),
        // ),
        // ((4, 3), (3, 3)) => ((x, y), direction),
        // ((3, 3), (3, 1)) => (
        //     mtxpos(x, y, face_width, face_height, 1, 2, 'L', 2),
        //     mturn(direction, 'L', 2),
        // ),
        // ((1, 2), (2, 2)) => ((x, y), direction),
        ((2, 1), (2, 4)) => (
            mtxpos(x, y, face_width, face_height, 1, 4, 'R', 1),
            mturn(direction, 'R', 1),
        ),
        ((1, 4), (1, 3)) => ((x, y), direction),
        ((1, 4), (3, 4)) => (
            mtxpos(x, y, face_width, face_height, 2, 1, 'L', 1),
            mturn(direction, 'L', 1),
        ),
        ((2, 1), (1, 1)) => (
            mtxpos(x, y, face_width, face_height, 1, 3, 'L', 2),
            mturn(direction, 'L', 2),
        ),
        ((1, 3), (2, 3)) => ((x, y), direction),
        ((1, 3), (1, 4)) => ((x, y), direction),
        ((2, 3), (2, 4)) => (
            mtxpos(x, y, face_width, face_height, 1, 4, 'R', 1),
            mturn(direction, 'R', 1),
        ),
        ((1, 3), (3, 3)) => (
            mtxpos(x, y, face_width, face_height, 2, 1, 'L', 2),
            mturn(direction, 'L', 2),
        ),
        ((1, 3), (1, 2)) => (
            mtxpos(x, y, face_width, face_height, 2, 2, 'R', 1),
            mturn(direction, 'R', 1),
        ),
        ((2, 2), (1, 2)) => (
            mtxpos(x, y, face_width, face_height, 1, 3, 'L', 1),
            mturn(direction, 'L', 1),
        ),
        ((2, 2), (2, 3)) => ((x, y), direction),
        ((2, 3), (1, 3)) => ((x, y), direction),
        ((2, 3), (2, 2)) => ((x, y), direction),
        ((2, 2), (2, 1)) => ((x, y), direction),
        ((2, 1), (2, 2)) => ((x, y), direction),
        ((2, 2), (3, 2)) => (
            mtxpos(x, y, face_width, face_height, 3, 1, 'L', 1),
            mturn(direction, 'L', 1),
        ),
        ((3, 1), (3, 2)) => (
            mtxpos(x, y, face_width, face_height, 2, 2, 'R', 1),
            mturn(direction, 'R', 1),
        ),
        ((2, 1), (3, 1)) => ((x, y), direction),
        ((3, 1), (2, 1)) => ((x, y), direction),
        ((1, 4), (1, 1)) => (
            mtxpos(x, y, face_width, face_height, 3, 1, 'R', 4),
            mturn(direction, 'R', 4),
        ),
        ((3, 1), (3, 4)) => (
            mtxpos(x, y, face_width, face_height, 1, 4, 'R', 4),
            mturn(direction, 'R', 4),
        ),
        ((3, 1), (1, 1)) => (
            mtxpos(x, y, face_width, face_height, 2, 3, 'L', 2),
            mturn(direction, 'L', 2),
        ),
        ((2, 3), (3, 3)) => (
            mtxpos(x, y, face_width, face_height, 3, 1, 'L', 2),
            mturn(direction, 'L', 2),
        ),
        ((1, 4), (2, 4)) => (
            mtxpos(x, y, face_width, face_height, 2, 3, 'L', 1),
            mturn(direction, 'L', 1),
        ),
        _ => panic!(),
        // _ => ((x, y), direction),
    }
}

fn wmod(v: isize, m: isize) -> isize {
    ((v % m) + m) % m
}

fn turn(direction: char, d: char) -> char {
    match d {
        'L' => match direction {
            'R' => 'U',
            'D' => 'R',
            'L' => 'D',
            'U' => 'L',
            _ => panic!(),
        },
        'R' => match direction {
            'R' => 'D',
            'D' => 'L',
            'L' => 'U',
            'U' => 'R',
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn mturn(direction: char, d: char, c: usize) -> char {
    let mut direction = direction;

    for _ in 0..c {
        direction = turn(direction, d);
    }

    direction
}

fn txpos(x: isize, y: isize, face_width: usize, face_height: usize, d: char) -> (isize, isize) {
    let lx = wmod(x, face_width as isize);
    let ly = wmod(y, face_height as isize);

    println!("{} {} {} {}", x, y, lx, ly);

    let (lx, ly) = match d {
        'L' => (ly, face_height as isize - lx - 1),
        'R' => (face_width as isize - ly - 1, lx),
        _ => panic!(),
    };
    println!("{} {} {} {}", x, y, lx, ly);

    (lx, ly)
}

fn mtxpos(
    x: isize,
    y: isize,
    face_width: usize,
    face_height: usize,
    tfx: usize,
    tfy: usize,
    d: char,
    c: usize,
) -> (isize, isize) {
    let mut pos = (x, y);

    for _ in 0..c {
        pos = txpos(pos.0, pos.1, face_width, face_height, d);
        println!("{:?}", pos);
    }

    (
        pos.0 + ((tfx - 1) * face_width) as isize,
        pos.1 + ((tfy - 1) * face_height) as isize,
    )
}

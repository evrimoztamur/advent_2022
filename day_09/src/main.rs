use std::collections::HashSet;

enum Cardinal {
    Up,
    Down,
    Left,
    Right,
}

impl Cardinal {
    fn move_location(&self, location: &Location) -> Location {
        match self {
            Cardinal::Up => Location {
                x: location.x,
                y: location.y + 1,
            },
            Cardinal::Down => Location {
                x: location.x,
                y: location.y - 1,
            },
            Cardinal::Left => Location {
                x: location.x - 1,
                y: location.y,
            },
            Cardinal::Right => Location {
                x: location.x + 1,
                y: location.y,
            },
        }
    }
}

struct Move {
    direction: Cardinal,
    distance: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Location {
    x: i64,
    y: i64,
}

impl Location {
    fn distance_to(&self, location: &Location) -> u64 {
        (location.x - self.x).abs().max((location.y - self.y).abs()) as u64
    }

    fn chase(&self, location: &Location) -> Location {
        if self.distance_to(location) > 1 {
            Location {
                x: self.x + (location.x - self.x).signum(),
                y: self.y + (location.y - self.y).signum(),
            }
        } else {
            self.clone()
        }
    }
}

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut moves: Vec<Move> = Vec::new();

        for line in content.lines() {
            moves.push(
                match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
                    ["U", v] => Move {
                        direction: Cardinal::Up,
                        distance: v.parse::<u64>().unwrap(),
                    },
                    ["D", v] => Move {
                        direction: Cardinal::Down,
                        distance: v.parse::<u64>().unwrap(),
                    },
                    ["L", v] => Move {
                        direction: Cardinal::Left,
                        distance: v.parse::<u64>().unwrap(),
                    },
                    ["R", v] => Move {
                        direction: Cardinal::Right,
                        distance: v.parse::<u64>().unwrap(),
                    },
                    _ => Move {
                        direction: Cardinal::Up,
                        distance: 0,
                    },
                },
            );
        }

        {
            let mut visited: HashSet<Location> = HashSet::new();
            let mut head = Location { x: 0, y: 0 };
            let mut tail = Location { x: 0, y: 0 };

            for head_move in &moves {
                for _ in 0..head_move.distance {
                    head = head_move.direction.move_location(&head);

                    if head.distance_to(&tail) > 1 {
                        tail = tail.chase(&head);
                    }

                    visited.insert(tail.clone());
                }
            }

            println!("P1 {}", visited.len());
        }

        {
            let mut visited: HashSet<Location> = HashSet::new();
            let mut snake: Vec<Location> = vec![Location { x: 0, y: 0 }; 10];

            for head_move in &moves {
                for _ in 0..head_move.distance {
                    snake[0] = head_move.direction.move_location(&snake[0]);

                    for i in 1..snake.len() {
                        let tail = snake[i].clone();

                        if tail.distance_to(&snake[i - 1]) > 1 {
                            snake[i] = snake[i].chase(&snake[i - 1]);
                        }
                    }

                    visited.insert(snake.last().unwrap().clone());
                }
            }

            println!("P2 {}", visited.len());
        }
    }
}

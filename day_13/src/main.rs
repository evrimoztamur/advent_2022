use itertools::{zip_eq, Itertools};

#[derive(Debug, Clone, Eq, Ord)]
enum Entry {
    Number(u8),
    Vector(Vec<Entry>),
}

impl Entry {
    fn push(&mut self, entry: Entry) {
        match self {
            Entry::Number(_) => panic!(),
            Entry::Vector(contents) => contents.push(entry),
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Entry::Number(a), Entry::Number(b)) => Some(a.cmp(b)),
            (Entry::Number(a), Entry::Vector(b)) => {
                Entry::Vector(vec![Entry::Number(*a)]).partial_cmp(&Entry::Vector(b.to_vec()))
            }
            (Entry::Vector(a), Entry::Number(b)) => {
                Entry::Vector(a.to_vec()).partial_cmp(&Entry::Vector(vec![Entry::Number(*b)]))
            }
            (Entry::Vector(a), Entry::Vector(b)) => {
                if a.len() != b.len() {
                    for v in a.iter().zip_longest(b) {
                        match v {
                            itertools::EitherOrBoth::Both(a, b) => match a.partial_cmp(b) {
                                Some(c) => match c {
                                    std::cmp::Ordering::Less => return Some(c),
                                    std::cmp::Ordering::Equal => continue,
                                    std::cmp::Ordering::Greater => return Some(c),
                                },
                                None => continue,
                            },
                            itertools::EitherOrBoth::Left(_) => {
                                return Some(std::cmp::Ordering::Greater)
                            }
                            itertools::EitherOrBoth::Right(_) => {
                                return Some(std::cmp::Ordering::Less)
                            }
                        }
                    }

                    Some(std::cmp::Ordering::Equal)
                } else {
                    for (l, r) in zip_eq(a, b) {
                        match l.partial_cmp(r) {
                            Some(c) => match c {
                                std::cmp::Ordering::Less => return Some(c),
                                std::cmp::Ordering::Equal => continue,
                                std::cmp::Ordering::Greater => return Some(c),
                            },
                            None => continue,
                        }
                    }

                    Some(std::cmp::Ordering::Equal)
                }
            }
        }
    }
}

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut pairs = Vec::new();

        for line in content.lines() {
            if !line.is_empty() {
                let mut repr: Vec<Entry> = vec![];
                let mut tempnum = "".to_string();

                for tk in line.chars() {
                    match tk {
                        '[' => {
                            repr.push(Entry::Vector(Vec::new()));
                        }
                        ']' => {
                            if !tempnum.is_empty() {
                                repr.last_mut()
                                    .unwrap()
                                    .push(Entry::Number(tempnum.parse().unwrap()));
                            }

                            let tip = repr.pop().unwrap();

                            if let Some(tail) = repr.last_mut() {
                                tail.push(tip);
                            } else {
                                repr.push(tip);
                            }
                        }
                        '0'..='9' => {
                            tempnum.push(tk);
                        }
                        ',' => {
                            if !tempnum.is_empty() {
                                repr.last_mut()
                                    .unwrap()
                                    .push(Entry::Number(tempnum.parse().unwrap()));
                            }
                        }
                        _ => (),
                    }

                    match tk {
                        '0'..='9' => (),
                        _ => {
                            tempnum = "".to_string();
                        }
                    }
                }

                pairs.push(repr.pop().unwrap());
            }
        }

        let mut i = 0;
        let mut totalind = 0;

        let chunkedpairs = pairs.clone();

        for chunk in &chunkedpairs.into_iter().chunks(2) {
            i += 1;

            let mut pair: Vec<Entry> = chunk.collect();
            let r = pair.pop().unwrap();
            let l = pair.pop().unwrap();

            let compres = l.partial_cmp(&r).unwrap();
            match compres {
                std::cmp::Ordering::Less => totalind += i,
                _ => (),
            }
        }

        println!("P1 {}", totalind);

        pairs.push(Entry::Vector(vec![Entry::Vector(vec![Entry::Number(2)])]));
        pairs.push(Entry::Vector(vec![Entry::Vector(vec![Entry::Number(6)])]));

        pairs.sort();

        let pos2 = pairs
            .iter()
            .position(|v| *v == Entry::Vector(vec![Entry::Vector(vec![Entry::Number(2)])]));
        let pos6 = pairs
            .iter()
            .position(|v| *v == Entry::Vector(vec![Entry::Vector(vec![Entry::Number(6)])]));

        println!("P2 {}", (pos2.unwrap() + 1) * (pos6.unwrap() + 1));
    }
}

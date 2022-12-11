#![feature(drain_filter)]
use std::{cell::RefCell, rc::Rc};

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test_num: i64,
    throws: (usize, usize),
    inspections: u64,
}

impl Monkey {
    fn inspect(&mut self, i: usize) -> usize {
        let item = &mut self.items[i];
        *item = (self.operation)(*item) / 3;

        self.inspections += 1;

        if *item % self.test_num == 0 {
            self.throws.0
        } else {
            self.throws.1
        }
    }

    fn inspect_coeff(&mut self, i: usize, coeff: i64) -> usize {
        let item = &mut self.items[i];
        *item = (self.operation)(*item) % coeff;

        self.inspections += 1;

        if *item % self.test_num == 0 {
            self.throws.0
        } else {
            self.throws.1
        }
    }
}

fn main() {
    let mut monkeys = Vec::new();
    // // Monkey 0:
    // //   Starting items: 79, 98
    // //   Operation: new = old * 19
    // //   Test: divisible by 23
    // //     If true: throw to monkey 2
    // //     If false: throw to monkey 3
    // let monkey = Rc::new(RefCell::new(Monkey {
    //     items: vec![79, 98].into(),
    //     operation: Box::new(|v| v * 19),
    //     test_num: 23,
    //     throws: (2, 3),
    //     inspections: 0,
    // }));
    // monkeys.push(monkey);
    // // Monkey 1:
    // //   Starting items: 54, 65, 75, 74
    // //   Operation: new = old + 6
    // //   Test: divisible by 19
    // //     If true: throw to monkey 2
    // //     If false: throw to monkey 0
    // let monkey = Rc::new(RefCell::new(Monkey {
    //     items: vec![54, 65, 75, 74].into(),
    //     operation: Box::new(|v| v + 6),
    //     test_num: 19,
    //     throws: (2, 0),
    //     inspections: 0,
    // }));
    // monkeys.push(monkey);
    // // Monkey 2:
    // //   Starting items: 79, 60, 97
    // //   Operation: new = old * old
    // //   Test: divisible by 13
    // //     If true: throw to monkey 1
    // //     If false: throw to monkey 3
    // let monkey = Rc::new(RefCell::new(Monkey {
    //     items: vec![79, 60, 97].into(),
    //     operation: Box::new(|v| v * v),
    //     test_num: 13,
    //     throws: (1, 3),
    //     inspections: 0,
    // }));
    // monkeys.push(monkey);
    // // Monkey 3:
    // //   Starting items: 74
    // //   Operation: new = old + 3
    // //   Test: divisible by 17
    // //     If true: throw to monkey 0
    // //     If false: throw to monkey 1
    // let monkey = Rc::new(RefCell::new(Monkey {
    //     items: vec![74].into(),
    //     operation: Box::new(|v| v + 3),
    //     test_num: 17,
    //     throws: (0, 1),
    //     inspections: 0,
    // }));
    // monkeys.push(monkey);

    //     Monkey 0:
    //     Starting items: 54, 98, 50, 94, 69, 62, 53, 85
    //     Operation: new = old * 13
    //     Test: divisible by 3
    //       If true: throw to monkey 2
    //       If false: throw to monkey 1
    let monkey = Rc::new(RefCell::new(Monkey {
        items: vec![54, 98, 50, 94, 69, 62, 53, 85].into(),
        operation: Box::new(|v| v * 13),
        test_num: 3,
        throws: (2, 1),
        inspections: 0,
    }));
    monkeys.push(monkey);

    //   Monkey 1:
    //     Starting items: 71, 55, 82
    //     Operation: new = old + 2
    //     Test: divisible by 13
    //       If true: throw to monkey 7
    //       If false: throw to monkey 2
    let monkey = Rc::new(RefCell::new(Monkey {
        items: vec![71, 55, 82].into(),
        operation: Box::new(|v| v + 2),
        test_num: 13,
        throws: (7, 2),
        inspections: 0,
    }));
    monkeys.push(monkey);

    //   Monkey 2:
    //     Starting items: 77, 73, 86, 72, 87
    //     Operation: new = old + 8
    //     Test: divisible by 19
    //       If true: throw to monkey 4
    //       If false: throw to monkey 7
    let monkey = Rc::new(RefCell::new(Monkey {
        items: vec![77, 73, 86, 72, 87].into(),
        operation: Box::new(|v| v + 8),
        test_num: 19,
        throws: (4, 7),
        inspections: 0,
    }));
    monkeys.push(monkey);

    //   Monkey 3:
    //     Starting items: 97, 91
    //     Operation: new = old + 1
    //     Test: divisible by 17
    //       If true: throw to monkey 6
    //       If false: throw to monkey 5
    let monkey = Rc::new(RefCell::new(Monkey {
        items: vec![97, 91].into(),
        operation: Box::new(|v| v + 1),
        test_num: 17,
        throws: (6, 5),
        inspections: 0,
    }));
    monkeys.push(monkey);

    //   Monkey 4:
    //     Starting items: 78, 97, 51, 85, 66, 63, 62
    //     Operation: new = old * 17
    //     Test: divisible by 5
    //       If true: throw to monkey 6
    //       If false: throw to monkey 3
    let monkey = Rc::new(RefCell::new(Monkey {
        items: vec![78, 97, 51, 85, 66, 63, 62].into(),
        operation: Box::new(|v| v * 17),
        test_num: 5,
        throws: (6, 3),
        inspections: 0,
    }));
    monkeys.push(monkey);

    //   Monkey 5:
    //     Starting items: 88
    //     Operation: new = old + 3
    //     Test: divisible by 7
    //       If true: throw to monkey 1
    //       If false: throw to monkey 0
    let monkey = Rc::new(RefCell::new(Monkey {
        items: vec![88].into(),
        operation: Box::new(|v| v + 3),
        test_num: 7,
        throws: (1, 0),
        inspections: 0,
    }));
    monkeys.push(monkey);

    //   Monkey 6:
    //     Starting items: 87, 57, 63, 86, 87, 53
    //     Operation: new = old * old
    //     Test: divisible by 11
    //       If true: throw to monkey 5
    //       If false: throw to monkey 0
    let monkey = Rc::new(RefCell::new(Monkey {
        items: vec![87, 57, 63, 86, 87, 53].into(),
        operation: Box::new(|v| v * v),
        test_num: 11,
        throws: (5, 0),
        inspections: 0,
    }));
    monkeys.push(monkey);

    //   Monkey 7:
    //     Starting items: 73, 59, 82, 65
    //     Operation: new = old + 6
    //     Test: divisible by 2
    //       If true: throw to monkey 4
    //       If false: throw to monkey 3
    let monkey = Rc::new(RefCell::new(Monkey {
        items: vec![73, 59, 82, 65].into(),
        operation: Box::new(|v| v + 6),
        test_num: 2,
        throws: (4, 3),
        inspections: 0,
    }));
    monkeys.push(monkey);

    {
        let monkeys = monkeys.clone();

        for _ in 0..20 {
            for monkey in monkeys.iter().enumerate() {
                let mut tosses = Vec::new();
                let this_monkey = monkey.1.clone();
                let mut this_monkey = this_monkey.borrow_mut();

                for i in 0..this_monkey.items.len() {
                    let toss = this_monkey.inspect(i);
                    tosses.push((i, this_monkey.items[i], toss));
                }

                for toss in tosses.iter() {
                    let other_monkey = monkeys[toss.2].clone();
                    let mut other_monkey = other_monkey.borrow_mut();
                    other_monkey.items.push(toss.1);
                }

                this_monkey
                    .items
                    .drain_filter(|v| tosses.iter().any(|toss| toss.1 == *v));
            }
        }

        let mut scores = Vec::new();

        for monkey in monkeys.iter().enumerate() {
            let this_monkey = monkey.1.borrow_mut();
            println!("{} {:?}", monkey.0, this_monkey.inspections);
            scores.push(this_monkey.inspections);
        }
        scores.sort();

        println!(
            "P1 {}",
            scores.iter().rev().take(2).fold(1, |mut acc, v| {
                acc *= v;
                acc
            })
        );
    }

    {
        let monkeys = monkeys.clone();

        let coeff = monkeys.iter().fold(1, |mut acc, v| {
            acc *= v.borrow_mut().test_num;
            acc
        });

        for _ in 0..10000 {
            for monkey in monkeys.iter().enumerate() {
                let mut tosses = Vec::new();
                let this_monkey = monkey.1.clone();
                let mut this_monkey = this_monkey.borrow_mut();

                for i in 0..this_monkey.items.len() {
                    let toss = this_monkey.inspect_coeff(i, coeff);
                    tosses.push((i, this_monkey.items[i], toss));
                }

                for toss in tosses.iter() {
                    let other_monkey = monkeys[toss.2].clone();
                    let mut other_monkey = other_monkey.borrow_mut();
                    other_monkey.items.push(toss.1);
                }

                this_monkey
                    .items
                    .drain_filter(|v| tosses.iter().any(|toss| toss.1 == *v));
            }
        }

        let mut scores = Vec::new();

        for monkey in monkeys.iter().enumerate() {
            let this_monkey = monkey.1.borrow_mut();
            println!("{} {:?}", monkey.0, this_monkey.inspections);
            scores.push(this_monkey.inspections);
        }
        scores.sort();

        println!(
            "P2 {}",
            scores.iter().rev().take(2).fold(1, |mut acc, v| {
                acc *= v;
                acc
            })
        );
    }
}

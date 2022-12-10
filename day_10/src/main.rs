#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX(i64),
}

impl Instruction {
    fn cycles(&self) -> u64 {
        match self {
            Instruction::NOOP => 1,
            Instruction::ADDX(_) => 2,
        }
    }
}

struct Device {
    cycles: u64,
    rx: i64,
    screen: [u8; 240],
    seeking_cycle: u64,
}

impl Device {
    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::NOOP => {}
            Instruction::ADDX(x) => {
                self.rx += x;
            }
        }
    }

    fn assess_signal(&mut self) -> u64 {
        if self.cycles == self.seeking_cycle {
            let signal = self.seeking_cycle * self.rx as u64;
            self.seeking_cycle += 40;
            signal
        } else {
            0
        }
    }

    fn plot_sprite(&mut self) {
        if (self.rx - (self.cycles % 40) as i64).abs() <= 1 {
            self.screen[self.cycles as usize] = 1;
        }
    }

    fn print_screen(&mut self) {
        for i in 0..6 {
            println!(
                "{}",
                self.screen[(i * 40)..((i + 1) * 40)]
                    .into_iter()
                    .map(|v| if *v == 0 { '.' } else { '#' })
                    .collect::<String>()
            );
        }
    }
}

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut device = Device {
            cycles: 0,
            rx: 1,
            screen: [0u8; 40 * 6],
            seeking_cycle: 20,
        };
        let mut instructions: Vec<Instruction> = Vec::new();

        for line in content.lines() {
            let split_line: Vec<&str> = line.split_whitespace().collect();

            let instruction = match split_line[..] {
                ["addx", x] => Instruction::ADDX(x.parse().unwrap()),
                _ => Instruction::NOOP,
            };

            instructions.push(instruction);
        }

        let mut signal = 0;

        for instruction in instructions {
            for _ in 0..instruction.cycles() {
                device.plot_sprite();
                device.cycles += 1;
                signal += device.assess_signal();
            }

            device.execute(&instruction);
        }

        println!("P1 {}", signal);

        device.print_screen();
    }
}

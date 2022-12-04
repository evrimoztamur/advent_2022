use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Pick {
    Rock,
    Paper,
    Scissors,
}

impl Pick {
    fn determine_result(&self, opponent: &Pick) -> GameResult {
        if self == opponent {
            return GameResult::Draw;
        }

        match (self, opponent) {
            (Pick::Rock, Pick::Paper) => GameResult::Lose,
            (Pick::Paper, Pick::Scissors) => GameResult::Lose,
            (Pick::Scissors, Pick::Rock) => GameResult::Lose,
            _ => GameResult::Win,
        }
    }

    fn score(self) -> u64 {
        match self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissors => 3,
        }
    }

    fn parse_opponent_pick(pick: &str) -> Pick {
        match pick {
            "A" => Pick::Rock,
            "B" => Pick::Paper,
            _ => Pick::Scissors,
        }
    }

    fn parse_your_pick(pick: &str) -> Pick {
        match pick {
            "X" => Pick::Rock,
            "Y" => Pick::Paper,
            _ => Pick::Scissors,
        }
    }

    fn pick_for_result(&self, target_result: &GameResult) -> Pick {
        match target_result {
            GameResult::Win => match self {
                Pick::Rock => Pick::Paper,
                Pick::Paper => Pick::Scissors,
                Pick::Scissors => Pick::Rock,
            },
            GameResult::Lose => match self {
                Pick::Rock => Pick::Scissors,
                Pick::Paper => Pick::Rock,
                Pick::Scissors => Pick::Paper,
            },
            GameResult::Draw => *self,
        }
    }
}

#[derive(PartialEq)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn score(self) -> u64 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }

    fn your_result(pick: &str) -> GameResult {
        match pick {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            _ => GameResult::Win,
        }
    }
}

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let mut score_p1 = 0;
        let mut score_p2 = 0;

        for line in content.lines() {
            let picks = line.split(' ').collect::<Vec<&str>>();
            let opponent_pick = Pick::parse_opponent_pick(picks[0]);

            {
                let your_pick = Pick::parse_your_pick(picks[1]);

                score_p1 += your_pick.determine_result(&opponent_pick).score();
                score_p1 += your_pick.score();
            }

            {
                let target_result = GameResult::your_result(picks[1]);
                let your_pick = &opponent_pick.pick_for_result(&target_result);

                score_p2 += target_result.score();
                score_p2 += your_pick.score();
            }
        }

        println!("P1 {}", score_p1);
        println!("P2 {}", score_p2);
    }
}

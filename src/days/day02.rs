use crate::days::Day;

pub const DAY2: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let rounds = parse_input(input).unwrap();

    let result: i32 = rounds.iter().map(|r| r.get_score_1()).sum();
    println!("Total score of the strategy guide: {}", result);
}
fn puzzle2(input: &String) {
    let rounds = parse_input(input).unwrap();

    let result: i32 = rounds.iter().map(|r| r.get_score_2()).sum();
    println!("Total score of the correct strategy guide: {}", result);
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    const VALUE_ROCK: i32 = 1;
    const VALUE_PAPER: i32 = 2;
    const VALUE_SCISSORS: i32 = 3;

    const SCORE_LOSE: i32 = 0;
    const SCORE_DRAW: i32 = 3;
    const SCORE_WIN: i32 = 6;
    /// The score for a single round is the score for the shape you selected
    /// (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round
    /// (0 if you lost, 3 if the round was a draw, and 6 if you won).
    fn score_against(&self, other: &RPS) -> i32 {
        match (self, other) {
            (RPS::Rock, RPS::Scissors) => RPS::SCORE_WIN + RPS::VALUE_ROCK,
            (RPS::Rock, RPS::Rock) => RPS::SCORE_DRAW + RPS::VALUE_ROCK,
            (RPS::Rock, RPS::Paper) => RPS::SCORE_LOSE + RPS::VALUE_ROCK,
            (RPS::Paper, RPS::Rock) => RPS::SCORE_WIN + RPS::VALUE_PAPER,
            (RPS::Paper, RPS::Paper) => RPS::SCORE_DRAW + RPS::VALUE_PAPER,
            (RPS::Paper, RPS::Scissors) => RPS::SCORE_LOSE + RPS::VALUE_PAPER,
            (RPS::Scissors, RPS::Paper) => RPS::SCORE_WIN + RPS::VALUE_SCISSORS,
            (RPS::Scissors, RPS::Scissors) => RPS::SCORE_DRAW + RPS::VALUE_SCISSORS,
            (RPS::Scissors, RPS::Rock) => RPS::SCORE_LOSE + RPS::VALUE_SCISSORS,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose
}

impl Outcome {
    fn to_rps(&self, against: &RPS) -> RPS {
        match (self, against) {
            (Outcome::Win, RPS::Rock) => RPS::Paper,
            (Outcome::Win, RPS::Paper) => RPS::Scissors,
            (Outcome::Win, RPS::Scissors) => RPS::Rock,
            (Outcome::Draw, RPS::Rock) => RPS::Rock,
            (Outcome::Draw, RPS::Paper) => RPS::Paper,
            (Outcome::Draw, RPS::Scissors) => RPS::Scissors,
            (Outcome::Lose, RPS::Rock) => RPS::Scissors,
            (Outcome::Lose, RPS::Paper) => RPS::Rock,
            (Outcome::Lose, RPS::Scissors) => RPS::Paper,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Round {
    /// Games: puzzle 1 uses the second RPS as our input, puzzle 2 uses outcome to determine input.
    games: Vec<(RPS, RPS, Outcome)>,

}

impl Round {
    fn get_score_1(&self) -> i32 {
        self.games.iter().map(|(opponent, us, _)| us.score_against(opponent)).sum()
    }

    fn get_score_2(&self) -> i32 {
        self.games.iter().map(|(opponent, _, outcome)| outcome.to_rps(opponent).score_against(opponent)).sum()
    }
}

fn parse_input(input: &str) -> Result<Vec<Round>, String> {
    let mut result: Vec<Round> = vec![];
    let mut games: Vec<(RPS, RPS, Outcome)> = vec![];
    for line in input.lines() {
        if line.trim().is_empty() && !games.is_empty() {
            result.push(Round { games });
            games = vec![];
            continue;
        }

        games.push(parse_game(line)?);
    }

    if !games.is_empty() {
        result.push(Round { games });
    }

    Ok(result)
}

fn parse_game(input: &str) -> Result<(RPS, RPS, Outcome), String> {
    let parts: Vec<_> = input.split(" ").collect();
    if parts.len() != 2 {
        return Err(format!("Expected exactly 2 parts in line '{}', but got {}", input, parts.len()));
    }

    // The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors.
    let opponent = match parts[0] {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => return Err(format!("Invalid RPS value for first column: {}", parts[0]))
    };

    // The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors.
    let puzzle_1 = match parts[1] {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => return Err(format!("Invalid RPS value for second column: {}", parts[0]))
    };
    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
    let puzzle_2 = match parts[1] {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => return Err(format!("Invalid Outcome value for second column: {}", parts[0]))
    };

    Ok((opponent, puzzle_1, puzzle_2))
}
#[cfg(test)]
mod tests {
    use crate::days::day02::{Outcome, parse_input, Round, RPS};

    const TEST_INPUT: &str = "\
        A Y\n\
        B X\n\
        C Z\n\
    ";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);

        assert!(result.is_ok(), "Expected to successfully parse test input");

        let rounds = result.unwrap();
        assert_eq!(rounds.len(), 1, "Expected to parse a single round");
        assert_eq!(rounds[0].games.len(), 3, "Expected three games in the round");
        assert_eq!(rounds[0].games[0], (RPS::Rock, RPS::Paper, Outcome::Draw));
        assert_eq!(rounds[0].games[1], (RPS::Paper, RPS::Rock, Outcome::Lose));
        assert_eq!(rounds[0].games[2], (RPS::Scissors, RPS::Scissors, Outcome::Win));
    }

    #[test]
    fn test_round_get_score() {
        let round = Round {
            games: vec![
                (RPS::Rock, RPS::Paper, Outcome::Draw),
                (RPS::Paper, RPS::Rock, Outcome::Lose),
                (RPS::Scissors, RPS::Scissors, Outcome::Win)
            ]
        };

        assert_eq!(round.get_score_1(), 15);
        assert_eq!(round.get_score_2(), 12);
    }
}
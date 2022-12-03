use std::ops::Add;
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn from_str(c: &str) -> Move {
        match c {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("{} doesn't map to a valid Move", c),
        }
    }
}

impl Add<Outcome> for Move {
    type Output = u32;

    fn add(self, rhs: Outcome) -> u32 {
        self as u32 + rhs as u32
    }
}
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

type Turn = (Move, Move);
type Strategy = Vec<Turn>;

fn load_strategy() -> Strategy {
    let raw_string = fs::read_to_string("src/day2/input.txt").expect("Failed to read input file");
    let mut strategy: Strategy = vec![];
    for line in raw_string.split('\n') {
        let mut chs = line.split_whitespace();
        let turn: Turn = (Move::from_str(chs.next().unwrap()), Move::from_str(chs.next().unwrap()));
        strategy.push(turn);
    }
    strategy
}



fn play_strategy(strategy: Strategy) -> u32 {
    let mut total_score = 0;

    for turn in strategy {
        total_score += get_score(turn);
    }

    total_score
}

fn get_score(turn: Turn) -> u32 {
    let (elf_move, my_move) = turn;

    if elf_move == my_move {
        return my_move + Outcome::Draw;
    };

    match (my_move, elf_move) {
        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => my_move + Outcome::Loss,
        (_, _) => my_move + Outcome::Win,
    }
}

fn main() {
    let strategy = load_strategy();
    println!("{}", play_strategy(strategy));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_turn() {
        let elf_move = Move::from_str("A");
        let my_move = Move::from_str("Y");
        let turn = (elf_move, my_move);
        assert_eq!(get_score(turn), 8);
    }

    #[test]
    fn test_example_strategy() {
        // Tests the example from the AoC explaination page
        // https://adventofcode.com/2022/day/2

        let strategy: Strategy = vec![
            (Move::from_str("A"), Move::from_str("Y")),
            (Move::from_str("B"), Move::from_str("X")),
            (Move::from_str("C"), Move::from_str("Z")),];
        let score = play_strategy(strategy);
        assert_eq!(score, 15);
    }


}

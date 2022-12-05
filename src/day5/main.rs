/*
Crate Label ->          [D]
Crate Label ->      [N] [C]
Crate Label ->      [Z] [M] [P]
Stack num ->         1   2   3

Instruction ->    move 1 from 2 to 1
Instruction ->    move 3 from 1 to 3
Instruction ->    move 2 from 2 to 1
Instruction ->    move 1 from 1 to 2
*/

use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, usize};
use std::fs;

type Crate = char;
type Stack = Vec<Crate>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_str(input_str: &str) -> Self {
        lazy_static! {
            static ref INST_RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }

        let cap = INST_RE.captures_iter(input_str).next().unwrap();
        println!("Num: {}, From: {}, To: {}", &cap[1], &cap[2], &cap[3]);

        let count = cap[1].parse::<usize>().unwrap();
        let from = cap[2].parse::<usize>().unwrap();
        let to = cap[3].parse::<usize>().unwrap();

        Instruction { count, from, to }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Problem {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

trait StackExt {
    fn from_str(input_str: &str) -> Stack;
}

impl StackExt for Stack {
    fn from_str(input_str: &str) -> Stack {
        todo!()
    }
}

impl Problem {
    fn from_str(input_str: &str) -> Self {
        let mut splits = input_str.split("\n\n");
        let stacks_str = splits.next().unwrap();
        let instructions_str = splits.next().unwrap();

        let instructions = instructions_str
            .split('\n')
            .map(Instruction::from_str)
            .collect::<Vec<Instruction>>();

        let stacks = Problem::stacks_from_str(stacks_str);
        Problem {
            stacks,
            instructions,
        }
    }

    fn stacks_from_str(input_str: &str) -> Vec<Stack> {
        /*
        Expected input string ex

            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3
        */

        let mut out: Vec<Stack> = Vec::new();

        let mut split = input_str.split('\n').rev();
        let idx_line_chars = split.next().unwrap().chars();

        let v_ = idx_line_chars.clone();

        let stack_idx = idx_line_chars
            .filter_map(|c| match c.is_ascii_digit() {
                true => Some(v_.clone().position(|c_| c == c_).unwrap()),
                false => None,
            })
            .collect::<Vec<usize>>();

        let mut stacks_by_id: HashMap<usize, usize> = HashMap::new();
        for (i, s_id) in stack_idx.iter().enumerate() {
            out.push(Vec::new());
            stacks_by_id.insert(*s_id, i);
        }

        for stack_line in split {
            for idx in &stack_idx {
                let s = stack_line.chars().nth(*idx).unwrap();
                if s.is_alphabetic() {
                    out[*stacks_by_id.get(idx).unwrap()].push(s);
                };
            }
        }
        out
    }

    fn solve(&self) -> Problem {
        let mut out = self.clone();
        for instruction in &self.instructions {

            let mut t_stack = Vec::new();
            for _ in 0..instruction.count {
                let v = out.stacks[instruction.from - 1].pop().unwrap();
                t_stack.push(v);
            }
            for v in t_stack.iter().rev() {
                out.stacks[instruction.to - 1].push(*v)
            }
        }

        out
    }

    fn display_answer(&self) {
        print!("Answer: ");
        for stack in &self.stacks {
            print!("{}", stack.last().unwrap());
        }
        print!("\n");
    }
}


fn load_problem_from_file() -> Problem {
    let raw_string = fs::read_to_string("/home/cole/rust/advent2022/src/day5/input.txt")
        .expect("Failed to read input file");

    Problem::from_str(raw_string.as_str())
}
fn main() {
    let problem = load_problem_from_file();
    let problem = problem.solve();
    problem.display_answer();

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_instruction_from_str() {
        let input_str = "move 1 from 2 to 1";
        let actual = Instruction::from_str(input_str);
        let expected = Instruction {
            count: 1,
            from: 2,
            to: 1,
        };
        assert_eq!(actual, expected);

        let input_str = "move 3 from 1 to 3";
        let actual = Instruction::from_str(input_str);
        let expected = Instruction {
            count: 3,
            from: 1,
            to: 3,
        };
        assert_eq!(actual, expected);

        let input_str = "move 2 from 2 to 1";
        let actual = Instruction::from_str(input_str);
        let expected = Instruction {
            count: 2,
            from: 2,
            to: 1,
        };
        assert_eq!(actual, expected);

        let input_str = "move 1 from 1 to 2";
        let actual = Instruction::from_str(input_str);
        let expected = Instruction {
            count: 1,
            from: 1,
            to: 2,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_construct_problem() {
        let input_str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1"#;
        println!("{}", input_str);
        let expected = Problem {
            stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            instructions: vec![Instruction {
                count: 1,
                from: 2,
                to: 1,
            }],
        };

        let actual = Problem::from_str(input_str);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve() {
        let input_str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let problem = Problem::from_str(input_str).solve();
        problem.display_answer();

    }
}

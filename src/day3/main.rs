use std::collections::hash_set::HashSet;
use std::fs;
use itertools::Itertools;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug)]
struct Rucksack {
    compartments: Vec<HashSet<char>>,
    all_items: HashSet<char>,
}

trait CharExt {
    fn priority(&self) -> u32;
}

impl CharExt for char {
    fn priority(&self) -> u32 {
        if self.is_uppercase() {
            return (ASCII_UPPER.iter().position(|c| c == self).unwrap() + 26 + 1) as u32;
        } else {
            return (ASCII_LOWER.iter().position(|c| c == self).unwrap() + 1) as u32;
        }
    }
}

impl Rucksack {
    fn from_str(input_str: &str) -> Rucksack {
        let mut chars = input_str.chars();

        let mut container1 = HashSet::new();
        let mut container2 = HashSet::new();

        let mut all_items = HashSet::new();
        while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
            container1.insert(c1);
            container2.insert(c2);

            all_items.insert(c1);
            all_items.insert(c2);
        }

        Rucksack {
            compartments: vec![container1, container2],
            all_items
        }
    }

    fn get_error(&self) -> char {
        *self.compartments[0]
            .intersection(&self.compartments[1])
            .last()
            .unwrap()
    }
}

fn error_priority_sum(rucksacks: Vec<Rucksack>) -> u32 {
    let mut priority_sum = 0;
    for rucksack in rucksacks {
        let common_item = rucksack.get_error();
        priority_sum += common_item.priority();
    }

    priority_sum
}

fn find_badge_type(rucksack1: &mut Rucksack, rucksack2: &Rucksack, rucksack3: &Rucksack) -> char {
    rucksack1.all_items.retain(|item| rucksack2.all_items.contains(item));    
    rucksack1.all_items.retain(|item| rucksack3.all_items.contains(item));

    *rucksack1.all_items.iter().last().unwrap()
}

fn load_rucksacks() -> Vec<Rucksack> {
    let raw_string = fs::read_to_string("src/day3/input.txt").expect("Failed to read input file");
    let lines = raw_string.split('\n');

    let mut rucksacks = vec![];
    for line in lines {
        rucksacks.push(Rucksack::from_str(line));
    }

    rucksacks
}

fn main() {
    println!("Day 3!");
    let rucksacks = load_rucksacks();
    println!("{:?}", error_priority_sum(rucksacks));

    let mut error_sum = 0;
    let mut rucksacks = load_rucksacks();
    for (r1, r2, r3) in rucksacks.iter_mut().tuples() {
        error_sum += find_badge_type(r1, r2, r3).priority();
    }

    println!("{:?}", error_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack_from_str() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::from_str(input_str);

        println!("{:?}", rucksack);
    }

    #[test]
    fn test_find_common_entry() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::from_str(input_str);

        let common_item = rucksack.compartments[0]
            .intersection(&rucksack.compartments[1])
            .last();
        assert_eq!(*common_item.unwrap(), 'p')
    }

    #[test]
    fn test_aoc_examples() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::from_str(input_str);

        let common_item = rucksack.get_error();
        assert_eq!(common_item, 'p');

        let input_str = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let rucksack = Rucksack::from_str(input_str);

        let common_item = rucksack.get_error();
        assert_eq!(common_item, 'L');

        let input_str = "PmmdzqPrVvPwwTWBwg";
        let rucksack = Rucksack::from_str(input_str);

        let common_item = rucksack.get_error();
        assert_eq!(common_item, 'P');

        let input_str = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let rucksack = Rucksack::from_str(input_str);

        let common_item = rucksack.get_error();
        assert_eq!(common_item, 'v');

        let input_str = "ttgJtRGJQctTZtZT";
        let rucksack = Rucksack::from_str(input_str);

        let common_item = rucksack.get_error();
        assert_eq!(common_item, 't');

        let input_str = "CrZsJsPPZsGzwwsLwLmpwMDw";
        let rucksack = Rucksack::from_str(input_str);

        let common_item = rucksack.get_error();
        assert_eq!(common_item, 's');
    }

    #[test]
    fn test_char_priority() {
        assert_eq!('p'.priority(), 16);
        assert_eq!('L'.priority(), 38);
        assert_eq!('P'.priority(), 42);
        assert_eq!('v'.priority(), 22);
        assert_eq!('t'.priority(), 20);
        assert_eq!('s'.priority(), 19);
    }

    #[test]
    fn test_error_sum() {
        let rucksacks = vec![
            Rucksack::from_str("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::from_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::from_str("PmmdzqPrVvPwwTWBwg"),
            Rucksack::from_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            Rucksack::from_str("ttgJtRGJQctTZtZT"),
            Rucksack::from_str("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];
        let sum_of_priorities = error_priority_sum(rucksacks);
        assert_eq!(sum_of_priorities, 157);
    }

    #[test]
    fn test_find_badge_type() {
        let mut rucksack1 = Rucksack::from_str("vJrwpWtwJgWrhcsFMMfFFhFp");
        let rucksack2 = Rucksack::from_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let rucksack3 = Rucksack::from_str("PmmdzqPrVvPwwTWBwg");

        assert_eq!(find_badge_type(&mut rucksack1, &rucksack2, &rucksack3), 'r');

        let mut rucksack4 = Rucksack::from_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        let rucksack5 = Rucksack::from_str("ttgJtRGJQctTZtZT");
        let rucksack6 = Rucksack::from_str("CrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!(find_badge_type(&mut rucksack4, &rucksack5, &rucksack6), 'Z');
    }

    #[test]
    fn test_error_sum_day2() {
        let mut rucksack1 = Rucksack::from_str("vJrwpWtwJgWrhcsFMMfFFhFp");
        let rucksack2 = Rucksack::from_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let rucksack3 = Rucksack::from_str("PmmdzqPrVvPwwTWBwg");

        let mut error_sum = find_badge_type(&mut rucksack1, &rucksack2, &rucksack3).priority();

        let mut rucksack4 = Rucksack::from_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        let rucksack5 = Rucksack::from_str("ttgJtRGJQctTZtZT");
        let rucksack6 = Rucksack::from_str("CrZsJsPPZsGzwwsLwLmpwMDw");
        error_sum += find_badge_type(&mut rucksack4, &rucksack5, &rucksack6).priority();

        assert_eq!(error_sum, 70);
    }
}

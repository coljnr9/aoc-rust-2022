use itertools::Itertools;
use std::collections::hash_set::HashSet;
use std::fs;

fn section_pairs_from_str(input_str: &str) -> (HashSet<u32>, HashSet<u32>) {
    // Input string looks like 2-4,6-8
    // Want to turn into 2..=4  6..=8
    let mut v = input_str.split(',').map(|seg| {
        seg.split('-')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple::<(u32, u32)>()
            .unwrap()
    });

    let (elf1_range, elf2_range) = (v.next().unwrap(), v.next().unwrap());
    (
        HashSet::from_iter(elf1_range.0..=elf1_range.1),
        HashSet::from_iter(elf2_range.0..=elf2_range.1),
    )
}

fn sections_from_str(input_str: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    let section_strings = input_str.split('\n').collect::<Vec<&str>>();

    let mut sections = Vec::new();
    for in_string in section_strings {
        sections.push(section_pairs_from_str(in_string));
    }

    sections
}

fn load_sections_from_file() -> Vec<(HashSet<u32>, HashSet<u32>)> {
    let raw_string = fs::read_to_string("/home/cole/rust/advent2022/src/day4/input.txt")
        .expect("Failed to read input file");
    sections_from_str(raw_string.as_str())
}

fn complete_overlap(sections: (HashSet<u32>, HashSet<u32>)) -> bool {
    sections.0.is_subset(&sections.1) || sections.1.is_subset(&sections.0)
}

fn any_overlap(sections: (HashSet<u32>, HashSet<u32>)) -> bool {
    !sections.0.is_disjoint(&sections.1)
}

fn main() {
    println!("Day 4");
    let sections = load_sections_from_file();

    let mut num_overlaps = 0;
    let mut num_any_overlap = 0;
    for section in sections {
        if complete_overlap(section.clone()) {
            num_overlaps += 1;
        }

        if any_overlap(section) {
            num_any_overlap += 1;
        }
    }
    dbg!(num_overlaps);
    dbg!(num_any_overlap);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_segments_from_str() {
        let input_str = "2-4,6-8";
        let sections = section_pairs_from_str(input_str);

        assert_eq!(
            sections,
            (
                HashSet::from_iter(vec![2, 3, 4]),
                HashSet::from_iter(vec![6, 7, 8])
            )
        );
    }

    #[test]
    fn test_sections_from_str() {
        // Example from AoC problem page
        // https://adventofcode.com/2022/day/4
        let test_str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let sections = sections_from_str(test_str);
        println!("{:?}", sections);
    }

    #[test]
    fn test_complete_overlap() {
        let sections = section_pairs_from_str("2-8,3-7");
        assert!(complete_overlap(sections));

        let sections = section_pairs_from_str("5-7,7-9");
        assert!(!complete_overlap(sections))
    }
}

use std::fs;

fn load_input_file() -> String {
    fs::read_to_string("src/day1/input.txt").expect("Failed to read input file")
}

fn sum_top_three_elves(input: String) -> u32 {
    let strings = input.split("\n\n");
    let strings = strings.map(|s| s.split('\n'));

    let mut elf_cals: Vec<u32> = Vec::default();

    for string in strings {
        let mut this_elf_cals = 0;
        for s in string {
            match s.parse::<u32>() {
                Ok(v) => this_elf_cals += v,
                Err(_) => eprintln!("ruh roh"),
            }
        }
        elf_cals.push(this_elf_cals);
    }
    elf_cals.sort();
    elf_cals.reverse();
    let v: u32 = elf_cals[..3].iter().sum();
    v
}

fn main() {
    let input_str = load_input_file();
    let best_elf = sum_top_three_elves(input_str);
    println!("{:?}", best_elf);
}

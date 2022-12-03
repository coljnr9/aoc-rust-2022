use std::fs;

fn load_input_file() -> String {
    fs::read_to_string("src/day1/input.txt").expect("Failed to read input file")
}

fn process_input_string(input: String) -> u32 {
    let strings = input.split("\n\n");
    let strings = strings.map(|s| s.split('\n'));

    let mut max_sum = 0;
    for string in strings {
        let mut elf_calories = 0;
        for s in string {
            match s.parse::<u32>() {
                Ok(v) => elf_calories += v,
                Err(_) => eprintln!("ruh roh"),
            }
        }
        if elf_calories > max_sum {
            max_sum = elf_calories;
        }
    }
    max_sum
}

fn main() {
    let input_str = load_input_file();
    let best_elf = process_input_string(input_str);
    println!("{:?}", best_elf);
}

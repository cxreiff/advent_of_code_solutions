use std::env;
use std::fs;
use day_02::part_1;
use day_02::part_2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let input = fs::read_to_string(input_path)
       .unwrap_or_else(|_| panic!("Should have an input file at path: {}", input_path));
    
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}


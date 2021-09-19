mod days;
use days::one;
use std::fs::read_to_string;

fn main() {
    let one_input = read_to_string("src/inputs/one.txt").expect("Konnte Eingabe für Tag 1 nicht lesen");
    let one_a_result = one::a(&one_input);
    let one_b_result = one::b(&one_input);
    println!("Day 1: \n A: {}\n B: {}", one_a_result, one_b_result);


}

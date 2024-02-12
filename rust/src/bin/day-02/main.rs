use crate::{part1::part1, part2::part2};


mod part1;
mod part2;

fn main() {
    let input = include_str!("./testinput.txt");

    println!("Answer part 1: {}", part1(input));
    println!("Answer part 2: {}", part2(input));
}

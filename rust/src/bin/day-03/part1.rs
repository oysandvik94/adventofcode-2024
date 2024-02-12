/// algorithm
/// scan for symbol
///     if found symbol:
///         go back and scan for number
///             peek left:
///                 if number, go left
///                 if symbol, pop
///     if not:
///         keep scanning
///
///
fn main() {
    let input = include_str!("./testinput.txt");
    let answer = part1(input);
    dbg!(answer);
}

trait IsSymbol {
    fn is_symbol(&self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(&self) -> bool {
        let not_numeric = !&self.is_numeric();
        let not_dot: bool = self != &'.';

        not_numeric && not_dot && !&self.is_whitespace()
    }
}

fn part1(input_ok: &str) -> u32 {
    let matrix_length: i32 = input_ok.lines().next().unwrap().len() as i32;
    let input = input_ok.replace("\n", "");
    let num_vec = parse_numbers(input.to_string());

    num_vec
        .iter()
        .map(|pair| sum_pairs(input.to_string(), pair, matrix_length))
        .sum::<u32>()
}

fn sum_pairs(input: String, pair: &(u32, usize), matrix_length: i32) -> u32 {
    println!("----------------");
    println!("analyzing pair {pair:?} with matrix length {matrix_length}");
    let left_bound = pair.1 as i32 - pair.0.to_string().len() as i32;
    let right_bound = pair.1 as i32 + 1;

    let current_range: Vec<i32> = (left_bound..=right_bound).collect();
    let above_range: Vec<i32> = current_range
        .iter()
        .map(|num| num - matrix_length)
        .collect();
    let under_range: Vec<i32> = current_range
        .iter()
        .map(|num| num + matrix_length)
        .collect();

    let vec3 = [current_range, under_range, above_range].concat();

    let found_symbol = vec3
        .iter()
        .inspect(|f| println!("checking {f}"))
        .any(|f| match input.chars().nth(f.unsigned_abs() as usize) {
            Some(c) => c.is_symbol(),
            None => false,
        });

    println!("is symbol: {found_symbol}");
    match found_symbol {
        true => pair.0,
        false => 0,
    }
}

fn parse_numbers(input: String) -> Vec<(u32, usize)> {
    let mut numbers: Vec<(u32, usize)> = Vec::new();

    let mut current_string = String::from("");

    for (i, el) in input.chars().enumerate() {
        if el.is_ascii_digit() {
            current_string.push(el);
        }

        let next_is_not_digit = input.chars().nth(i + 1).is_some_and(|c| !c.is_ascii_digit());
        let has_digit = !current_string.is_empty();
        if next_is_not_digit && has_digit {
            numbers.push((current_string.parse::<u32>().unwrap(), i));
            current_string.clear();
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part_one() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let output = part1(input);

        assert_eq!(output, 4361);
    }
}

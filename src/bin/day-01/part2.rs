use std::{collections::HashMap, num::ParseIntError, ops::Add};

fn main() {
    let input = include_str!("./testinput.txt");
    let answer = part2(input);
    dbg!(answer);
}

fn part2(input: &str) -> i32 {
    struct Pair {
        first: u8,
        last: u8,
    }

    impl Pair {
        fn to_sum(&self) -> Result<i32, ParseIntError> {
            let x: char = char::from_digit(self.first as u32, 10).unwrap();
            let y: char = char::from_digit(self.last as u32, 10).unwrap();

            println!("summing {} and {}", x, y);
            let xy: String = vec![x, y].iter().collect();

            return xy.parse();
        }
    }

    let mut total_sum: i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let first: u8 = scan_right(line);
        let last: u8 = scan_left(line);

        let pair: Pair = Pair { first, last };

        total_sum = total_sum.add(pair.to_sum().unwrap());
    }

    return total_sum;
}

fn scan_line(line: &str, reversed: bool) -> u8 {
    let string_digits: HashMap<&str, u8> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let closure_match = |potential: &str, reversed: bool| -> bool {
        string_digits.keys().any(|key: &&str| match reversed {
            true => {
                let reversed_key: String = key.chars().rev().collect();
                return reversed_key.starts_with(potential);
            }
            false => key.starts_with(potential),
        })
    };

    let mut match_string = String::from("");
    for c in line.chars() {
        if c.is_whitespace() {
            continue;
        };

        println!("---------- analyzing {}", c);
        match c.is_digit(10) {
            true => return c.to_digit(10).unwrap() as u8,
            false => {
                match_string.push(c);

                println!("checking match for {}", match_string);
                if closure_match(&match_string, reversed) {
                    println!("there is match {}", match_string);
                    let hjelp: String;
                    match reversed {
                        true => {
                            hjelp = match_string.chars().rev().collect();
                        }
                        false => hjelp = match_string.to_string(),
                    };

                    match string_digits.get(&hjelp[..]) {
                        Some(&value) => return value,
                        None => continue,
                    }
                } else {
                    match_string.remove(0);
                    while match_string.len() > 0 {
                        match closure_match(&match_string, reversed) {
                            true => break,
                            false => {
                                match_string.remove(0);
                            },
                        }
                    }
                }
            }
        }
    }

    return 0;
}

fn scan_right(line: &str) -> u8 {
    return scan_line(line, false);
}

fn scan_left(line: &str) -> u8 {
    let reversed_line: String = line.chars().rev().collect();
    return scan_line(&reversed_line as &str, true);
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part_two() {
        let input = "
            two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
49threenjdgrmgfnfhcgz
            ";

        let output = part2(input);

        assert_eq!(output, 324);
    }
}
